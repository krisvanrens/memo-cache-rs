use memo_cache::MemoCache;
use rand_distr::{Distribution, Normal};
use std::{collections::HashMap, thread, time};

fn some_expensive_calculation(_: i32) -> f32 {
    thread::sleep(time::Duration::from_millis(20)); // ...zzzZZzz...
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

    /// Regular method, taking the calculation penalty, always.
    fn regular(&self, input: i32) -> f32 {
        some_expensive_calculation(input)
    }

    /// Memoized method, using a `HashMap` cache (no retention management).
    fn memoized1(&mut self, input: i32) -> f32 {
        if let Some(value) = self.cache1.get(&input) {
            *value
        } else {
            let result = some_expensive_calculation(input);
            self.cache1.insert(input, result);
            result
        }
    }

    /// Memoized method, using a `MemoCache` cache (using `get` and `insert`).
    fn memoized2a(&mut self, input: i32) -> f32 {
        if let Some(value) = self.cache2.get(&input) {
            *value
        } else {
            let result = some_expensive_calculation(input);
            self.cache2.insert(input, result);
            result
        }
    }

    /// Memoized method, using a `MemoCache` cache (using `get_or_insert_with`).
    fn memoized2b(&mut self, input: i32) -> f32 {
        *self
            .cache2
            .get_or_insert_with(&input, |&x| some_expensive_calculation(x))
    }
}

fn main() {
    // This test runs three individual test cases:
    //
    //   1. a regular (non-memoized) method,
    //   2. a method memoized using a hash map,
    //   3. a method memoized using a MemoCache cache (two notation variants).
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

    // Use the same input data for all tests:
    let inputs = (0..100)
        .map(|_| normal.sample(&mut rng) as i32)
        .collect::<Vec<_>>();

    let mut p = Process::new();

    println!("Running tests..");

    let now = time::Instant::now();
    inputs.iter().fold(0.0, |sum, &i| sum + p.regular(i));

    let d_regular = now.elapsed();

    let now = time::Instant::now();
    inputs.iter().fold(0.0, |sum, &i| sum + p.memoized1(i));

    let d_memoized1 = now.elapsed();

    let now = time::Instant::now();
    inputs.iter().fold(0.0, |sum, &i| sum + p.memoized2a(i));

    let d_memoized2a = now.elapsed();

    let now = time::Instant::now();
    inputs.iter().fold(0.0, |sum, &i| sum + p.memoized2b(i));

    let d_memoized2b = now.elapsed();

    println!("Done. Timing results:");
    println!("Regular:                {} ms", d_regular.as_millis());
    println!("Memoized (hash):        {} ms", d_memoized1.as_millis());
    println!("Memoized (MemoCache A): {} ms", d_memoized2a.as_millis());
    println!("Memoized (MemoCache B): {} ms", d_memoized2b.as_millis());

    let get_size = |capacity| capacity * (std::mem::size_of::<u32>() + std::mem::size_of::<f32>());

    println!("Post-test occupied cache sizes:");
    println!("  Hash:      {} bytes", get_size(p.cache1.capacity()));
    println!("  MemoCache: {} bytes", get_size(p.cache2.capacity()));
}
