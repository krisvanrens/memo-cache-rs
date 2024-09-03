# MemoCache

![build status](https://github.com/krisvanrens/memo-cache-rs/actions/workflows/build-and-test.yml/badge.svg)

A small, fixed-size cache with retention management, e.g. for use with memoization.

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
        // ..do some expensive calculation..
    }
}
```

We can add memoization to this function as follows:

```rs
struct Process {
    cache: MemoCache<u64, f64, 32>,
}

impl Process {
    fn calculate(&mut self, input: u64) -> f64 {
        if let Some(result) = self.cache.get(&input) {
            *result // Use cached value.
        } else {
            let result = /* ..do some expensive calculation.. */;
            self.cache.insert(input, result);
            result
        }
    }
}
```

Each call to `calculate` will first check if the input value is already in the cache.
If so: use the cached value, otherwise update the cache with a new, calculated value.

The cache is fixed-size, so if it is full, the oldes key/value pair will be evicted, and memory usage is constant.

## Performance notes

The use of a sequential data storage does have performance impact, especially for key lookup.
That's why this cache will only be beneficial performance-wise when used with a relatively small size, up to about 128 elements.

Run the included benchmarks using `criterion`: `cargo bench`

## TODO

- Extend documentation (on `[no_std`, example, etc.).
- Add `get_or_else` method that can nicely deal with references to cached items instead of being forced to use the cache in an imperative fashion.
- Improve benchmarks to be more useful and indicative.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
