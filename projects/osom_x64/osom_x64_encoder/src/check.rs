const _: () = {
    // Note: A bunch of assumptions that we operate on. This is mostly guaranteed by
    // the Rust standard, but better safe than sorry.
    assert!(size_of::<i8>() == 1);
    assert!(size_of::<i8>() == size_of::<u8>());
    assert!(size_of::<i16>() == 2);
    assert!(size_of::<i16>() == size_of::<u16>());
    assert!(size_of::<i32>() == 4);
    assert!(size_of::<i32>() == size_of::<u32>());
    assert!(size_of::<i64>() == 8);
    assert!(size_of::<i64>() == size_of::<u64>());
    assert!(size_of::<isize>() == size_of::<usize>());
    assert!(size_of::<usize>() <= 8);
    assert!(size_of::<f32>() == 4);
    assert!(size_of::<f64>() == 8);
    assert!(i8::MIN == -128);
    assert!(i8::MAX == 127);
};
