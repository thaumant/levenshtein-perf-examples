use criterion::{criterion_group, criterion_main, Criterion};
use levenshtein_perf_examples::{
    levenshtein1,
    levenshtein2,
    levenshtein3,
    levenshtein4,
    levenshtein5,
};

pub fn versions_bench(cr: &mut Criterion) {
    let sample = [
        ("crate",     "trace"),
        ("captain",   "ptain"),
        ("dwayne",    "duane"),
        ("martha",    "marhta"),
        ("kitten",    "sitting"),
        ("mailbox",   "boxmail"),
        ("mailbox",   "alimbox"),
        ("dixon",     "dicksonx"),
        ("jellyfish", "smellyfish"),
    ];

    cr.bench_function("version1", |b| b.iter(|| {
        let mut result = 0;
        for &(str1, str2) in &sample {
            result += levenshtein1(str1, str2);
        }
        result
    }));

    cr.bench_function("version2", |b| b.iter(|| {
        let mut result = 0;
        for &(str1, str2) in &sample {
            result += levenshtein2(str1, str2);
        }
        result
    }));

    cr.bench_function("version3", |b| b.iter(|| {
        let mut result = 0;
        for &(str1, str2) in &sample {
            result += levenshtein3(str1, str2);
        }
        result
    }));

    cr.bench_function("version4", |b| b.iter(|| {
        let mut result = 0;
        for &(str1, str2) in &sample {
            result += levenshtein4(str1, str2);
        }
        result
    }));

    cr.bench_function("version5", |b| b.iter(|| {
        let mut result = 0;
        for &(str1, str2) in &sample {
            result += levenshtein5(str1, str2);
        }
        result
    }));
}


criterion_group!(benches, versions_bench);
criterion_main!(benches);
