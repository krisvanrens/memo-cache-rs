use criterion::{criterion_group, criterion_main, Criterion};
use memo_cache::MemoCache;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use std::{collections::HashMap, ops::RangeInclusive, thread, time};

// Pseudo random number generator seed value (used for all benches).
const RNG_SEED_VALUE: u64 = 42;

// Uniform input data distribution settings.
const UNIFORM_RANGE_NARROW: RangeInclusive<i32> = -30..=30;
const UNIFORM_RANGE_WIDE: RangeInclusive<i32> = -100..=100;

// Normal input data distribution settings.
const NORMAL_VARIANCE_NARROW: f32 = 5.0;
const NORMAL_VARIANCE_WIDE: f32 = 30.0;

// Fixed-size cache capacity for all benches.
const MEMO_CACHE_CAPACITY: usize = 64; // [elements].

/// Calculate cache size given a capacity (assuming u32 -> u32 caches).
fn cache_size(capacity: usize) -> usize {
    capacity * (std::mem::size_of::<u32>() + std::mem::size_of::<f32>())
}

fn fake_expensive_calculation() {
    thread::sleep(time::Duration::from_millis(1));
}

// Hash map cache, uniform distribution.
fn bench_hash_map_uniform(c: &mut Criterion) {
    let mut g = c.benchmark_group("HashMap - Uniform distribution");

    let mut cache = HashMap::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    g.bench_function("narrow range", |b| {
        b.iter_batched(
            || rng.gen_range(UNIFORM_RANGE_NARROW),
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

    let mut cache = HashMap::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    g.bench_function("wide range", |b| {
        b.iter_batched(
            || rng.gen_range(UNIFORM_RANGE_WIDE),
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

    g.finish();
}

// Fixed-size cache, uniform distribution.
fn bench_memo_cache_uniform(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!(
        "MemoCache (size: {}) - Uniform distribution",
        MEMO_CACHE_CAPACITY
    ));

    let mut cache = MemoCache::<_, _, MEMO_CACHE_CAPACITY>::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    g.bench_function("narrow range", |b| {
        b.iter_batched(
            || rng.gen_range(-10..=10),
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

    let mut cache = MemoCache::<_, _, MEMO_CACHE_CAPACITY>::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    g.bench_function("wide range", |b| {
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

    g.finish();
}

// Hash map cache, normal distribution.
fn bench_hash_map_normal(c: &mut Criterion) {
    let mut g = c.benchmark_group("HashMap - Normal distribution");

    let mut cache = HashMap::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    let normal = Normal::new(0.0, NORMAL_VARIANCE_NARROW).unwrap();
    g.bench_function("narrow range", |b| {
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

    let mut cache = HashMap::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    let normal = Normal::new(0.0, NORMAL_VARIANCE_WIDE).unwrap();
    g.bench_function("wide range", |b| {
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

    g.finish();
}

// Fixed-size cache, normal distribution.
fn bench_memo_cache_normal(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!(
        "MemoCache (size: {}) - Normal distribution",
        MEMO_CACHE_CAPACITY
    ));

    let mut cache = MemoCache::<_, _, MEMO_CACHE_CAPACITY>::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    let normal = Normal::new(0.0, NORMAL_VARIANCE_NARROW).unwrap();
    g.bench_function("narrow range", |b| {
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

    let mut cache = MemoCache::<_, _, MEMO_CACHE_CAPACITY>::new();
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED_VALUE);
    let normal = Normal::new(0.0, NORMAL_VARIANCE_WIDE).unwrap();
    g.bench_function("wide range", |b| {
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

    g.finish();
}

criterion_group!(
    benches,
    bench_hash_map_uniform,
    bench_memo_cache_uniform,
    bench_hash_map_normal,
    bench_memo_cache_normal
);
criterion_main!(benches);
