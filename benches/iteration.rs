use criterion::{black_box, criterion_group, criterion_main, Criterion};


pub fn iteration_bench(cr: &mut Criterion) {
    let chars_str = "abcdefghijklmnopqrstuvwxyz";
    let chars_vec: Vec<char> = chars_str.chars().collect();

    cr.bench_function("iteration_str", |b| b.iter(|| {
        let mut result: char = '?';
        for ch in chars_str.chars() {
            result = black_box(ch);
        }
        result
    }));

    cr.bench_function("iteration_vec", |b| b.iter(|| {
        let mut result: char = '?';
        for &ch in &chars_vec {
            result = black_box(ch);
        }
        result
    }));
}


criterion_group!(benches, iteration_bench);
criterion_main!(benches);
