use std::ptr;
use criterion::{criterion_group, criterion_main, Criterion};

fn store_simple<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    for val in iter {
        buffer.push(val);
    }
}

fn store_unsafe<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    let mut i = 0;
    let mut cap = buffer.capacity();
    for item in iter {
        if i >= cap {
            buffer.reserve(1);
            cap = buffer.capacity();
        }
        unsafe {
            *buffer.get_unchecked_mut(i) = item;
        }
        i += 1;
    }
    unsafe {
        buffer.set_len(i);
    }
}

fn store_raw<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    let mut i = 0;
    let mut p = buffer.as_mut_ptr();
    let mut cap = buffer.capacity();
    for item in iter {
        if i >= cap {
            buffer.reserve(1);
            p = buffer.as_mut_ptr();
            cap = buffer.capacity();
        }
        unsafe {
            ptr::write(p.add(i), item);
        }
        i += 1;
    }
    unsafe {
        buffer.set_len(i);
    }
}

fn store_extend<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    buffer.extend(iter);
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

    cr.bench_function("store_extend", |b| {
        let mut buffer = Vec::with_capacity(string.chars().count());
        b.iter(|| {
            store_extend(&mut buffer, string.chars());
        })
    });
}


criterion_group!(benches, store_bench);
criterion_main!(benches);
