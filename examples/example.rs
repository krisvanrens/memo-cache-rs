use memo_cache::MemoCache;
use rand_distr::{Distribution, Normal};
use std::{collections::HashMap, thread, time};

fn some_expensive_calculation(_: i32) -> f32 {
    thread::sleep(time::Duration::from_millis(50)); // ...zzzZZzz...
    std::f32::consts::PI
}

struct Process {
    pub cache1: HashMap<i32, f32>,
    pub cache2: MemoCache<i32, f32, 32>,
}

impl Process {
    fn new() -> Self {
        Self {
            cache1: HashMap::new(),
            cache2: MemoCache::new(),
        }
    }

    fn regular_method(&self, input: i32) -> f32 {
        some_expensive_calculation(input)
    }

    fn memoized_method1(&mut self, input: i32) -> f32 {
        if let Some(value) = self.cache1.get(&input) {
            *value
        } else {
            let result = some_expensive_calculation(input);
            self.cache1.insert(input, result);
            result
        }
    }

    fn memoized_method2(&mut self, input: i32) -> f32 {
        if let Some(value) = self.cache2.get(&input) {
            *value
        } else {
            let result = some_expensive_calculation(input);
            self.cache2.insert(input, result);
            result
        }
    }
}

fn main() {
    // This test runs three individual test cases:
    //
    //   1. a regular (non-memoized) method,
    //   2. a method memoized using a hash map,
    //   3. a method memoized using a MemoCache cache.
    //
    // Each of the methods are fed a series of random input numbers from a
    // normal distribution for which they (fake) "calculate" a result value.
    // The memoized methods keep a local cache of result values for input
    // values. The hash map will definitely perform best, but has no retention
    // management -- its memory usage will grow with every new inserted input
    // value. The method using the MemoCache cache will use a fixed-capacity
    // cache and will perform at best as good as the hash map cache version,
    // and in the worst case as bad as the regular (non-memoized) method.

    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 30.0).unwrap();

    let mut p = Process::new();

    println!("Running tests..");

    let now = time::Instant::now();
    for _ in 0..100 {
        p.regular_method(normal.sample(&mut rng) as i32);
    }

    let d_regular = now.elapsed();

    let now = time::Instant::now();
    for _ in 0..100 {
        p.memoized_method1(normal.sample(&mut rng) as i32);
    }

    let d_memoized1 = now.elapsed();

    let now = time::Instant::now();
    for _ in 0..100 {
        p.memoized_method2(normal.sample(&mut rng) as i32);
    }

    let d_memoized2 = now.elapsed();

    println!("Done. Timing results:");
    println!("Regular:              {} ms", d_regular.as_millis());
    println!("Memoized (hash):      {} ms", d_memoized1.as_millis());
    println!("Memoized (MemoCache): {} ms", d_memoized2.as_millis());

    let get_size = |capacity| capacity * (std::mem::size_of::<u32>() + std::mem::size_of::<f32>());

    println!("Post-test occupied cache sizes:");
    println!("  Hash:      {} bytes", get_size(p.cache1.capacity()));
    println!("  MemoCache: {} bytes", get_size(p.cache2.capacity()));
}
