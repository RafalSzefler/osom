use criterion::{criterion_group, criterion_main, Criterion};
use osom_utils::arrays::DynamicArray;
use rand::RngCore;
use std::hint::black_box;

#[inline(never)]
fn pass<T>(_val: T) {}

fn fill_array(size: u32, mut seed: u32) {
    for _ in 0..10 {
        let mut arr = DynamicArray::new().unwrap();
        for _ in 0..size {
            arr.push(seed).unwrap();
            seed += 1;
        }
        pass(black_box(arr));
    }
}

fn fill_vec(size: u32, mut seed: u32) {
    for _ in 0..10 {
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(seed);
            seed += 1;
        }
        pass(black_box(vec));
    }
}

fn criterion_setup(c: &mut Criterion) {
    let seed = {
        let mut rng = rand::thread_rng();
        let mut val = 0;
        for _ in 0..300 {
            val = rng.next_u32();
        }
        val
    };

    let bencher = |c: &mut Criterion, count: u32| {
        let real_count = count + 7;
        c.bench_function(format!("array {count}").as_str(), |b| {
            b.iter(|| fill_array(black_box(real_count), black_box(seed)))
        });
        c.bench_function(format!("vec {count}").as_str(), |b| {
            b.iter(|| fill_vec(black_box(real_count), black_box(seed)))
        });
    };

    bencher(c, 10);
    bencher(c, 10 << 4);
    bencher(c, 10 << 8);
    bencher(c, 10 << 12);
    bencher(c, 10 << 16);
}

criterion_group!(_benches, criterion_setup);
criterion_main!(_benches);
