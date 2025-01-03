use std::sync::{Arc, Mutex};

use osom_utils::arrays::{DynamicArray, DynamicArrayError};
use rand::{Rng, RngCore};

#[test]
fn test_array() -> Result<(), DynamicArrayError> {
    let mut array = DynamicArray::<i32>::new()?;
    assert_eq!(array.len(), 0);
    assert_eq!(array.as_slice(), &[]);
    array.push(0)?;
    assert_eq!(array.len(), 1);
    assert_eq!(array.as_slice(), &[0]);
    array.push(1)?;
    assert_eq!(array.len(), 2);
    assert_eq!(array.as_slice(), &[0, 1]);
    array.push(-1)?;
    assert_eq!(array.len(), 3);
    assert_eq!(array.as_slice(), &[0, 1, -1]);
    array.push(14)?;
    assert_eq!(array.len(), 4);
    assert_eq!(array.as_slice(), &[0, 1, -1, 14]);
    Ok(())
}

#[test]
fn test_array_2() -> Result<(), DynamicArrayError> {
    let mut array = DynamicArray::<i32>::new()?;
    let mut vec = Vec::<i32>::new();
    for idx in 0..1024 {
        array.push(idx)?;
        vec.push(idx);
    }

    assert_eq!(array.len() as usize, vec.len());
    assert_eq!(array.as_slice(), vec.as_slice());

    assert_eq!(array.pop().unwrap(), 1023);
    assert_eq!(array.pop().unwrap(), 1022);
    assert_eq!(array.pop().unwrap(), 1021);
    vec.pop();
    vec.pop();
    vec.pop();
    assert_eq!(array.len() as usize, vec.len());
    assert_eq!(array.as_slice(), vec.as_slice());
    Ok(())
}

#[test]
fn test_array_drop() {
    struct Foo {
        idx: i32,
        array: Arc<Mutex<Vec<i32>>>,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            let mut vec = self.array.lock().unwrap();
            vec.push(self.idx);
        }
    }

    let arc_vec = Arc::new(Mutex::new(Vec::<i32>::new()));
    let mut dyn_array = DynamicArray::new().unwrap();
    let mut expected = Vec::new();
    for idx in 0..1024 {
        let value = (idx * 2 + 7) % 13;
        dyn_array
            .push(Foo {
                idx: value,
                array: arc_vec.clone(),
            })
            .unwrap();
        expected.push(value);
    }

    let last_idx = -1;
    dyn_array
        .push(Foo {
            idx: last_idx,
            array: arc_vec.clone(),
        })
        .unwrap();

    {
        let vec = arc_vec.lock().unwrap();
        assert_eq!(vec.as_slice(), &[]);
    }

    let last = dyn_array.pop().unwrap();
    assert_eq!(last.idx, last_idx);

    drop(dyn_array);

    {
        let vec = arc_vec.lock().unwrap();
        assert_eq!(vec.as_slice(), expected.as_slice());
    }

    drop(last);
    expected.push(last_idx);

    {
        let vec = arc_vec.lock().unwrap();
        assert_eq!(vec.as_slice(), expected.as_slice());
    }
}

#[test]
fn test_dyn_array_to_array() {
    let mut dyn_array = DynamicArray::<i32>::new().unwrap();
    let mut vec = Vec::<i32>::new();
    for idx in 0..5000 {
        dyn_array.push(idx).unwrap();
        vec.push(idx);
    }

    assert_eq!(dyn_array.len() as usize, vec.len());
    assert_eq!(dyn_array.as_slice(), vec.as_slice());

    let array = dyn_array.into_array();
    assert_eq!(array.len() as usize, vec.len());
    assert_eq!(array.as_slice(), vec.as_slice());
}

#[test]
fn test_rand_array() {
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let start = rng.gen_range(10000..20000) as u32;
        let end = rng.gen_range(0..5000) + 15000 as u32;

        let mut dyn_array = DynamicArray::new().unwrap();
        let mut expected = Vec::new();
        for _ in start..end {
            let value = rng.next_u64();
            dyn_array.push(value).unwrap();
            expected.push(value);
        }

        assert_eq!(dyn_array.as_slice(), expected.as_slice());
        let arr = dyn_array.into_array();
        assert_eq!(arr.as_slice(), expected.as_slice());
    }
}
