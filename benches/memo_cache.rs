use criterion::{criterion_group, criterion_main, Criterion};
use memo_cache::MemoCache;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{collections::HashMap, thread, time};

fn fake_expensive_calculation() {
    thread::sleep(time::Duration::from_millis(1));
}

// Hash map cache, uniform distribution (memory size: 201 * (32 + 32) bits = 1608 bytes).
fn bench_hash_map_uniform(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut cache = HashMap::new();
    c.bench_function("HashMap (control, uniform)", |b| {
        b.iter(|| {
            let input = rng.gen_range(-100..=100);
            if cache.get(&input).is_none() {
                fake_expensive_calculation();
                cache.insert(input, 42);
            }
        })
    });
}

// Hash map cache, normal distribution (memory size: ~201 * (32 + 32) bits = 1608 bytes).
fn bench_hash_map_normal(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 30.0).unwrap();
    let mut cache = HashMap::new();
    c.bench_function("HashMap (control, normal)", |b| {
        b.iter(|| {
            let input = normal.sample(&mut rng) as i32;
            if cache.get(&input).is_none() {
                fake_expensive_calculation();
                cache.insert(input, 42);
            }
        })
    });
}

// 64-element fixed-size cache, uniform distribution (memory size: 64 * (32 + 32) bits = 512 bytes).
fn bench_memo_cache64_uniform(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut cache = MemoCache::<_, _, 64>::new();
    c.bench_function("MemoCache (size: 64, uniform)", |b| {
        b.iter(|| {
            let input = rng.gen_range(-100..=100);
            if cache.get(&input).is_none() {
                fake_expensive_calculation();
                cache.insert(input, 42);
            }
        })
    });
}

// 64-element fixed-size cache, normal distribution (memory size: 64 * (32 + 32) bits = 512 bytes).
fn bench_memo_cache64_normal(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 30.0).unwrap();
    let mut cache = MemoCache::<_, _, 64>::new();
    c.bench_function("MemoCache (size: 64, normal)", |b| {
        b.iter(|| {
            let input = normal.sample(&mut rng) as i32;
            if cache.get(&input).is_none() {
                fake_expensive_calculation();
                cache.insert(input, 42);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_hash_map_uniform,
    bench_hash_map_normal,
    bench_memo_cache64_uniform,
    bench_memo_cache64_normal,
);
criterion_main!(benches);
