#[macro_use]
extern crate criterion;
use criterion::Criterion;

use rust_benchmarking::*;

const NBR_ELEM: usize = 10_000_000;


fn bench_method1(c: &mut Criterion) {
    // Set up the array
    let v = rust_benchmarking::get_randvec(NBR_ELEM);
    c.bench_function("checked_index", |b| b.iter(|| rust_benchmarking::checked_index(&v)));
}


fn bench_method2(c: &mut Criterion) {
    // Set up the array
    let v = rust_benchmarking::get_randvec(NBR_ELEM);
    c.bench_function("unchecked_index", |b| b.iter(|| rust_benchmarking::unchecked_index(&v)));
}

fn bench_method3(c: &mut Criterion) {
    // Set up the array
    let v = rust_benchmarking::get_randvec(NBR_ELEM);
    c.bench_function("iterator_access", |b| b.iter(|| rust_benchmarking::iterator_access(&v)));
}

fn bench_method4(c: &mut Criterion) {
    // Set up the array
    let v = rust_benchmarking::get_randvec(NBR_ELEM);
    c.bench_function("backwards_index", |b| b.iter(|| rust_benchmarking::backwards_index(&v)));
}

fn bench_method5(c: &mut Criterion) {
    // Set up the array
    let v = rust_benchmarking::get_randvec(NBR_ELEM);
    c.bench_function("skip10_index", |b| b.iter(|| rust_benchmarking::skip10_index(&v)));
}

criterion_group!(benches, bench_method1, bench_method2, bench_method3, bench_method4, bench_method5);
criterion_main!(benches);
