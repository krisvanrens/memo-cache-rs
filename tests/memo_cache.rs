mod tests_external {
    use memo_cache::MemoCache;

    #[test]
    fn test_empty() {
        let c = MemoCache::<bool, bool, 2>::new();

        assert_eq!(c.capacity(), 2);

        // NOTE: Even though the cache memory is pre-allocated, each cache slot should be marked as "empty".
        assert_eq!(c.get(&true), None);
        assert_eq!(c.get(&false), None);
    }

    #[test]
    fn test_simple() {
        let mut c = MemoCache::<_, _, 3>::new();

        c.insert("The Answer", 42); // K = &str, V = i32

        assert_eq!(c.get("The Answer"), Some(&42));
    }

    #[test]
    fn test_contains_key() {
        let mut c = MemoCache::<&str, i32, 3>::new();

        assert_eq!(c.contains_key("hello"), false);

        c.insert("hello", 42);

        assert_eq!(c.contains_key("hello"), true);
    }

    #[test]
    fn test_get() {
        let mut c = MemoCache::<&str, i32, 3>::new();

        assert_eq!(c.get("hello"), None);

        c.insert("hello", 42);

        assert_eq!(c.get("hello"), Some(&42));
    }

    #[test]
    fn test_get_mut() {
        let mut c = MemoCache::<&str, i32, 3>::new();

        assert_eq!(c.get_mut("hello"), None);

        c.insert("hello", 42);

        if let Some(v) = c.get_mut("hello") {
            assert_eq!(v, &mut 42);

            *v = 100;
        }

        assert_eq!(c.get("hello"), Some(&100));
    }

    #[test]
    fn test_get_or_insert_with() {
        let mut c = MemoCache::<String, i32, 3>::new();

        assert_eq!(c.contains_key("hello"), false);
        assert_eq!(c.contains_key("hi"), false);

        // Insert a new key.
        assert_eq!(
            c.get_or_insert_with(&"hello".to_owned(), |s| {
                assert_eq!(s, "hello");
                42
            }),
            &42
        );

        assert_eq!(c.get("hello"), Some(&42));
        assert_eq!(c.contains_key("hi"), false);

        // Insert another new key.
        assert_eq!(
            c.get_or_insert_with(&"hi".to_owned(), |s| {
                assert_eq!(s, "hi");
                17
            }),
            &17
        );

        assert_eq!(c.get("hello"), Some(&42));
        assert_eq!(c.get("hi"), Some(&17));

        // Get an existing key (function is not called).
        assert_eq!(
            c.get_or_insert_with(&"hello".to_owned(), |_| {
                assert!(false);
                13 // NOTE: Key already exists, this value is not used.
            }),
            &42
        );
    }

    #[test]
    fn test_get_or_try_insert_with() {
        let mut c = MemoCache::<String, i32, 3>::new();

        assert_eq!(c.contains_key("hello"), false);
        assert_eq!(c.contains_key("hi"), false);

        // Insert a new key.
        assert_eq!(
            c.get_or_try_insert_with(&"hello".to_owned(), |s| -> Result<_, ()> {
                assert_eq!(s, "hello");
                Ok(42)
            }),
            Ok(&42)
        );

        assert_eq!(c.get("hello"), Some(&42));
        assert_eq!(c.contains_key("hi"), false);

        // Insert another new key.
        assert_eq!(
            c.get_or_try_insert_with(&"hi".to_owned(), |s| -> Result<_, ()> {
                assert_eq!(s, "hi");
                Ok(17)
            }),
            Ok(&17)
        );

        assert_eq!(c.get("hello"), Some(&42));
        assert_eq!(c.get("hi"), Some(&17));

        // Get an existing key (function is not called).
        assert_eq!(
            c.get_or_try_insert_with(&"hello".to_owned(), |_| -> Result<_, ()> {
                assert!(false);
                Ok(13) // NOTE: Key already exists, this value is not used.
            }),
            Ok(&42)
        );

        // Failing insert attempt for a nonexistent key.
        assert_eq!(
            c.get_or_try_insert_with(&"G'day".to_owned(), |s| {
                assert_eq!(s, "G'day");
                Err("Whoops")
            }),
            Err("Whoops")
        );

        assert_eq!(c.get("G'day"), None);
    }

    #[test]
    fn test_clear() {
        let mut c = MemoCache::<&str, i32, 3>::new();

        assert_eq!(c.get("hello"), None);

        c.insert("hello", 42);

        assert_eq!(c.get("hello"), Some(&42));

        c.clear();

        assert_eq!(c.get("hello"), None);
    }

    #[test]
    fn test_borrow() {
        let mut c = MemoCache::<String, i32, 3>::new();

        c.insert("hello".to_owned(), 42);

        // Get using borrowed type (i.e. `&str`).
        assert_eq!(c.contains_key("hello"), true);
        assert_eq!(c.get("hello"), Some(&42));
        assert_eq!(c.get_mut("hello"), Some(&mut 42));
    }

    #[test]
    fn test_nonempty() {
        let mut c = MemoCache::<String, i32, 3>::new();

        assert_eq!(c.capacity(), 3);

        let kvs = vec![
            ("veni".to_owned(), 19),
            ("vidi".to_owned(), 23),
            ("vici".to_owned(), 29),
        ];
        let kv0 = kvs.get(0).unwrap();
        let kv1 = kvs.get(1).unwrap();
        let kv2 = kvs.get(2).unwrap();

        assert_eq!(c.get(&kv0.0), None);
        assert_eq!(c.get(&kv1.0), None);
        assert_eq!(c.get(&kv2.0), None);

        c.insert(kv0.0.clone(), kv0.1);

        assert_eq!(c.get(&kv0.0), Some(&kv0.1));

        c.insert(kv1.0.clone(), kv1.1);

        assert_eq!(c.get(&kv1.0), Some(&kv1.1));

        c.insert(kv2.0.clone(), kv2.1);

        assert_eq!(c.get(&kv2.0), Some(&kv2.1));

        // The cache is now full, and another insertion will make the first key/value be removed.

        c.insert("blah".to_owned(), 42);

        assert_eq!(c.get(&kv0.0), None);
        assert_eq!(c.get(&kv1.0), Some(&kv1.1));
        assert_eq!(c.get(&kv2.0), Some(&kv2.1));

        c.insert("bleh".to_owned(), 42);
        c.insert("bloh".to_owned(), 42);

        assert_eq!(c.get(&kv1.0), None);
        assert_eq!(c.get(&kv2.0), None);
    }

    #[test]
    fn test_duplicate_insertions() {
        let mut c = MemoCache::<String, i32, 2>::new();

        let kv0 = ("John".to_owned(), 17);
        let kv1 = ("Doe".to_owned(), 19);

        assert_eq!(c.get(&kv0.0), None);
        assert_eq!(c.get(&kv1.0), None);

        c.insert(kv0.0.to_owned(), kv0.1);
        c.insert(kv1.0.to_owned(), kv1.1);

        assert_eq!(c.get(&kv0.0), Some(&kv0.1));
        assert_eq!(c.get(&kv1.0), Some(&kv1.1));

        // Inserting a duplicate key/value should be a no-op.

        c.insert(kv0.0.to_owned(), kv0.1);

        assert_eq!(c.get(&kv0.0), Some(&kv0.1));
        assert_eq!(c.get(&kv1.0), Some(&kv1.1));

        // Inserting a duplicate key with a new value should update the value.

        assert_eq!(c.get(&kv0.0), Some(&kv0.1));
        assert_eq!(c.get(&kv1.0), Some(&kv1.1));

        c.insert(kv0.0.to_owned(), 42);

        assert_eq!(c.get(&kv0.0), Some(&42)); // Updated.
        assert_eq!(c.get(&kv1.0), Some(&kv1.1));
    }
}
