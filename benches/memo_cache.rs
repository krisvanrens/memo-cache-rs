use criterion::{criterion_group, criterion_main, Criterion};
use memo_cache::MemoCache;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{collections::HashMap, thread, time};

/// Calculate cache size given a capacity (assuming u32 -> u32 caches).
fn cache_size(capacity: usize) -> usize {
    capacity * (std::mem::size_of::<u32>() + std::mem::size_of::<f32>())
}

fn fake_expensive_calculation() {
    thread::sleep(time::Duration::from_millis(1));
}

// Hash map cache, uniform distribution.
fn bench_hash_map_uniform(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut cache = HashMap::new();
    c.bench_function("HashMap (control, uniform)", |b| {
        b.iter_batched(
            || rng.gen_range(-100..=100),
            |input| {
                if cache.get(&input).is_none() {
                    fake_expensive_calculation();
                    cache.insert(input, 42);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });

    println!("Used cache size: {} bytes", cache_size(cache.capacity()));
}

// Hash map cache, normal distribution.
fn bench_hash_map_normal(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 30.0).unwrap();
    let mut cache = HashMap::new();
    c.bench_function("HashMap (control, normal)", |b| {
        b.iter_batched(
            || normal.sample(&mut rng) as i32,
            |input| {
                if cache.get(&input).is_none() {
                    fake_expensive_calculation();
                    cache.insert(input, 42);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });

    println!("Used cache size: {} bytes", cache_size(cache.capacity()));
}

// 128-element fixed-size cache, uniform distribution (fixed memory size: 128 * (32 + 32) bits = 1024 bytes).
fn bench_memo_cache_uniform(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut cache = MemoCache::<_, _, 128>::new();
    c.bench_function("MemoCache (size: 128, uniform)", |b| {
        b.iter_batched(
            || rng.gen_range(-100..=100),
            |input| {
                cache.get_or_insert_with(&input, |_| {
                    fake_expensive_calculation();
                    42
                });
            },
            criterion::BatchSize::SmallInput,
        )
    });

    println!("Used cache size: {} bytes", cache_size(cache.capacity()));
}

// 128-element fixed-size cache, normal distribution (fixed memory size: 128 * (32 + 32) bits = 1024 bytes).
fn bench_memo_cache_normal(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 30.0).unwrap();
    let mut cache = MemoCache::<_, _, 128>::new();
    c.bench_function("MemoCache (size: 128, normal)", |b| {
        b.iter_batched(
            || normal.sample(&mut rng) as i32,
            |input| {
                cache.get_or_insert_with(&input, |_| {
                    fake_expensive_calculation();
                    42
                });
            },
            criterion::BatchSize::SmallInput,
        )
    });

    println!("Used cache size: {} bytes", cache_size(cache.capacity()));
}

criterion_group!(
    benches,
    bench_hash_map_uniform,
    bench_hash_map_normal,
    bench_memo_cache_uniform,
    bench_memo_cache_normal,
);
criterion_main!(benches);
