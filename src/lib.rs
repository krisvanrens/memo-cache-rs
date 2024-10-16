#![no_std]

use core::borrow::Borrow;

/// Key equivalence trait, to support `Borrow` types as keys.
trait Equivalent<K: ?Sized> {
    /// Returns `true` if two values are equivalent, `false` if otherwise.
    fn equivalent(&self, k: &K) -> bool;
}

impl<Q: ?Sized, K: ?Sized> Equivalent<K> for Q
where
    Q: Eq,
    K: Borrow<Q>,
{
    fn equivalent(&self, k: &K) -> bool {
        self == k.borrow()
    }
}

/// A single key/value slot used in the cache.
#[derive(Clone, PartialEq)]
enum KeyValueSlot<K, V> {
    Used((K, V)),
    Empty,
}

impl<K, V> KeyValueSlot<K, V> {
    /// Check a used slot key for equivalence.
    #[cfg_attr(feature = "inline-more", inline)]
    fn is_key<Q>(&self, k: &Q) -> bool
    where
        Q: Equivalent<K> + ?Sized,
    {
        if let KeyValueSlot::Used(kv) = self {
            k.equivalent(&kv.0)
        } else {
            false
        }
    }

    /// Get the value of a used slot.
    #[cfg_attr(feature = "inline-more", inline)]
    fn get_value(&self) -> Option<&V> {
        if let KeyValueSlot::Used(kv) = self {
            Some(&kv.1)
        } else {
            None
        }
    }

    /// Get the value of a used slot (for mutation).
    #[cfg_attr(feature = "inline-more", inline)]
    fn get_value_mut(&mut self) -> Option<&mut V> {
        if let KeyValueSlot::Used(kv) = self {
            Some(&mut kv.1)
        } else {
            None
        }
    }

    /// Update the value of a used slot.
    #[cfg_attr(feature = "inline-more", inline)]
    fn update_value(&mut self, v: V) {
        if let KeyValueSlot::Used(kv) = self {
            kv.1 = v
        }
    }
}

/// A small, fixed-size, heap-allocated key/value cache with retention management.
pub struct MemoCache<K, V, const SIZE: usize> {
    buffer: [KeyValueSlot<K, V>; SIZE],
    cursor: usize,
}

impl<K, V, const SIZE: usize> MemoCache<K, V, SIZE>
where
    K: Clone + Eq,
    V: Clone,
{
    /// Create a new cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let c = MemoCache::<u32, String, 4>::new();
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new() -> Self {
        Self {
            buffer: [const { KeyValueSlot::Empty }; SIZE],
            cursor: 0,
        }
    }

    /// Get the (fixed) capacity of the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let c = MemoCache::<u32, String, 8>::new();
    ///
    /// assert_eq!(c.capacity(), 8);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub const fn capacity(&self) -> usize {
        SIZE
    }

    /// Replace slot under cursor and shift cursor position. Returns a reference to the replaced slot value.
    #[cfg_attr(feature = "inline-more", inline)]
    fn replace_and_shift(&mut self, k: K, v: V) -> &V {
        // SAFETY: The cursor value is assumed to be correct.
        let s = unsafe { self.buffer.get_unchecked_mut(self.cursor) };

        *s = KeyValueSlot::Used((k, v));

        // Move the cursor over the buffer elements sequentially, creating FIFO behavior.
        self.cursor = (self.cursor + 1) % SIZE;

        // SAFETY: The slot was filled with a key/value above.
        unsafe { s.get_value().unwrap_unchecked() }
    }

    /// Insert a key/value pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, &str, 4>::new();
    ///
    /// assert_eq!(c.get(&42), None);
    ///
    /// c.insert(42, "The Answer");
    ///
    /// assert_eq!(c.get(&42), Some(&"The Answer"));
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, k: K, v: V) {
        match self.buffer.iter_mut().find(|e| e.is_key(&k)) {
            Some(s) => s.update_value(v),
            None => {
                self.replace_and_shift(k, v);
            }
        }
    }

    /// Returns `true` if the cache contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, &str, 4>::new();
    ///
    /// assert_eq!(c.contains_key(&42), false);
    ///
    /// c.insert(42, "The Answer");
    ///
    /// assert_eq!(c.contains_key(&42), true);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.buffer.iter().any(|e| e.is_key(k))
    }

    /// Lookup a cache entry by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, &str, 4>::new();
    ///
    /// assert_eq!(c.get(&42), None);
    ///
    /// c.insert(42, "The Answer");
    ///
    /// assert_eq!(c.get(&42), Some(&"The Answer"));
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.buffer
            .iter()
            .find(|e| e.is_key(k))
            .map(|e| e.get_value().unwrap())
    }

    /// Lookup a cache entry by key (for mutation).
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, &str, 4>::new();
    ///
    /// c.insert(42, "The Answer");
    ///
    /// if let Some(v) = c.get_mut(&42) {
    ///     *v = "Another Answer";
    /// }
    ///
    /// assert_eq!(c.get(&42), Some(&"Another Answer"));
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.buffer
            .iter_mut()
            .find(|e| e.is_key(k))
            .map(|e| e.get_value_mut().unwrap())
    }

    /// Get the index for a given key, if found.
    #[cfg_attr(feature = "inline-more", inline)]
    fn get_key_index<Q>(&self, k: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.buffer.iter().position(|e| e.is_key(k))
    }

    /// Get a value, or, if it does not exist in the cache, insert it using the value computed by `f`.
    /// Returns a reference to the found, or newly inserted value associated with the given key.
    /// If a value is inserted, the key is cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, &str, 4>::new();
    ///
    /// assert_eq!(c.get(&42), None);
    ///
    /// let v = c.get_or_insert_with(&42, |_| "The Answer");
    ///
    /// assert_eq!(v, &"The Answer");
    /// assert_eq!(c.get(&42), Some(&"The Answer"));
    /// ```
    ///
    /// # Notes
    ///
    /// Because this crate is `no_std`, we have no access to `std::borrow::ToOwned`, which means we cannot create a
    /// version of `get_or_insert_with` that can create an owned value from a borrowed key.
    ///
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_or_insert_with<F>(&mut self, k: &K, f: F) -> &V
    where
        F: FnOnce(&K) -> V,
    {
        if let Some(i) = self.get_key_index(k) {
            // SAFETY: The key index was retrieved from a found key.
            unsafe { self.buffer[i].get_value().unwrap_unchecked() }
        } else {
            self.replace_and_shift(k.clone(), f(k))
        }
    }

    /// Get a value, or, if it does not exist in the cache, insert it using the value computed by `f`.
    /// Returns a result with a reference to the found, or newly inserted value associated with the given key.
    /// If `f` fails, the error is returned.
    /// If a value is inserted, the key is cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, &str, 4>::new();
    ///
    /// assert_eq!(c.get(&42), None);
    ///
    /// let answer : Result<_, &str> = Ok("The Answer");
    /// let v = c.get_or_try_insert_with(&42, |_| answer);
    ///
    /// assert_eq!(v, Ok(&"The Answer"));
    /// assert_eq!(c.get(&42), Some(&"The Answer"));
    ///
    /// let v = c.get_or_try_insert_with(&17, |_| Err("Dunno"));
    ///
    /// assert_eq!(v, Err("Dunno"));
    /// assert_eq!(c.get(&17), None);
    /// ```
    ///
    /// # Notes
    ///
    /// Because this crate is `no_std`, we have no access to `std::borrow::ToOwned`, which means we cannot create a
    /// version of `get_or_try_insert_with` that can create an owned value from a borrowed key.
    ///
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_or_try_insert_with<F, E>(&mut self, k: &K, f: F) -> Result<&V, E>
    where
        F: FnOnce(&K) -> Result<V, E>,
    {
        if let Some(i) = self.get_key_index(k) {
            // SAFETY: The key index was retrieved from a found key.
            Ok(unsafe { self.buffer[i].get_value().unwrap_unchecked() })
        } else {
            f(k).map(|v| self.replace_and_shift(k.clone(), v))
        }
    }

    /// Clear the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use memo_cache::MemoCache;
    ///
    /// let mut c = MemoCache::<u32, &str, 4>::new();
    ///
    /// assert_eq!(c.get(&42), None);
    ///
    /// c.insert(42, "The Answer");
    ///
    /// assert_eq!(c.get(&42), Some(&"The Answer"));
    ///
    /// c.clear();
    ///
    /// assert_eq!(c.get(&42), None);
    ///
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn clear(&mut self) {
        self.buffer
            .iter_mut()
            .for_each(|e| *e = KeyValueSlot::Empty);
        self.cursor = 0;
    }
}

impl<K, V, const SIZE: usize> Default for MemoCache<K, V, SIZE>
where
    K: Clone + Eq,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests_internal {
    use super::*;

    #[test]
    fn test_new_state() {
        const SIZE: usize = 8;

        let c = MemoCache::<i32, i32, SIZE>::new();

        // Verify cache size.
        assert_eq!(c.buffer.len(), SIZE);
        assert_eq!(c.capacity(), SIZE);

        // All slots should be empty.
        assert!(c.buffer.iter().all(|s| s == &KeyValueSlot::Empty));
    }

    #[test]
    fn test_cursor_state() {
        let mut c = MemoCache::<i32, i32, 2>::new();

        assert_eq!(c.cursor, 0);

        c.insert(1, 2);

        assert_eq!(c.cursor, 1);

        c.insert(3, 4);

        assert_eq!(c.cursor, 0);

        c.insert(5, 6);

        assert_eq!(c.cursor, 1);

        c.insert(7, 8);

        assert_eq!(c.cursor, 0);
    }
}
