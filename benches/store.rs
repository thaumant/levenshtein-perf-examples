use std::ptr;
use std::cmp::max;
use criterion::{criterion_group, criterion_main, Criterion};

fn store_simple<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    for val in iter {
        buffer.push(val);
    }
}

fn store_unsafe<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    let mut capacity = buffer.capacity();
    let mut i = 0;
    for item in iter {
        if i >= capacity {
            buffer.reserve(max(capacity * 2, 1));
            capacity = buffer.capacity();
        }
        unsafe { *buffer.get_unchecked_mut(i) = item; }
        i += 1;
    }
    unsafe { buffer.set_len(i); }
}

fn store_raw<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    let mut i   = 0;
    let mut p   = buffer.as_mut_ptr();
    let mut cap = buffer.capacity() as isize;

    for item in iter {
        if i >= cap {
            buffer.reserve(max(cap * 2, 1) as usize);
            cap = buffer.capacity() as isize;
            p = buffer.as_mut_ptr();
        }
        unsafe {
            ptr::write(p.offset(i), item);
            i += 1;
        }
    }
    unsafe {
        buffer.set_len(i as usize);
    }
}


pub fn store_bench(cr: &mut Criterion) {
    let string = "abcdefghijklmnopqrstuvwxyz";

    cr.bench_function("store_simple", |b| {
        let mut buffer = Vec::with_capacity(string.chars().count());
        b.iter(|| {
            store_simple(&mut buffer, string.chars());
        })
    });

    cr.bench_function("store_unsafe", |b| {
        let mut buffer = Vec::with_capacity(string.chars().count());
        b.iter(|| {
            store_unsafe(&mut buffer, string.chars());
        })
    });

    cr.bench_function("store_raw", |b| {
        let mut buffer = Vec::with_capacity(string.chars().count());
        b.iter(|| {
            store_raw(&mut buffer, string.chars());
        })
    });
}


criterion_group!(benches, store_bench);
criterion_main!(benches);
