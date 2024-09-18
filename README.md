![build status](https://github.com/krisvanrens/memo-cache-rs/actions/workflows/build-and-test.yml/badge.svg)

# MemoCache

A small, fixed-size cache with retention management, e.g. for use with memoization.

This library does not use the standard library so it is compatible with `#[no_std]` crates.

## Introduction

Sometimes it can be beneficial to speedup pure function calls by using [memoization](https://en.wikipedia.org/wiki/Memoization).

The cache storage required for implementing such a speedup often is an associative container (i.e. a key/value store).
Programming language standard libraries provide such containers, often implemented as a [hash table](https://en.wikipedia.org/wiki/Hash_table) or a [red-black tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree).
These implementations are fine for performance, but do not actually cover all cases because of the lack of retention management

Suppose your input data covers the whole space that can be represented by a 64-bit integer.
There probably is some (generally non-uniform) distribution with which the input values arrive, but it's possible that over time *all* possible values pass by.
Any cache without retention management will then grow to potentially enormous dimensions in memory which is undesirable.

The cache implemented in this library uses a FIFO-style sequential data storage with fixed size, pre-allocated memory.
When the cache is full, the oldest item is evicted.

## Example usage

Suppose we have a pure method `calculate` on a type `Process` (without memoization):

```rs
struct Process { /* ... */ }

impl Process {
    fn calculate(&self, input: u64) -> f64 {
        // ..do expensive calculation on `input`..
    }
}
```

Each single call to this function results in the resource costs of the calculation.
We can add memoization to this function in two different ways:

- Using `MemoCache::get_or_insert_with`,
- Using `MemoCache::get` and `MemoCache::insert`.

For each of the following examples: each call to `calculate` will first check if the input value is already in the cache.
If so: use the cached value, otherwise update the cache with a new, calculated value.

The cache is fixed-size, so if it is full, the oldest key/value pair will be evicted, and memory usage is constant.

See the `examples/` directory for more example code.

### Example A: `get_or_insert_with`

```rs
struct Process {
    cache: MemoCache<u64, f64, 32>,
}

impl Process {
    fn calculate(&mut self, input: u64) -> f64 {
        *self.cache.get_or_insert_with(&input, |&i| /* ..do calculation on `input`.. */)
    }
}
```

For fallible insert functions, there's `get_or_try_insert_with`.

### Example B: `get` and `insert`

```rs
struct Process {
    cache: MemoCache<u64, f64, 32>,
}

impl Process {
    fn calculate(&mut self, input: u64) -> f64 {
        if let Some(result) = self.cache.get(&input) {
            *result // Use cached value.
        } else {
            let result = /* ..do calculation on `input`.. */;
            self.cache.insert(input, result);
            result
        }
    }
}
```

## Performance notes

The use of a simple sequential data storage does have performance impact, especially for key lookup.
That's why this cache will only be beneficial performance-wise when used with a relatively small size, up to about 128 elements.

Run the included benchmarks using [criterion](https://crates.io/crates/criterion) by invoking: `cargo bench`

## TODO

- Improve benchmarks to be more useful and indicative.
- Investigate potential cache improvements (e.g. start [here](https://en.wikipedia.org/wiki/Cache_replacement_policies)).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
