    #[test]
    fn test_interference_cache_store_get() {
        let mut cache = ArchetypicalInterferenceCache::new(100);
        let coefficients = [0.5; 22];

        let pattern = InterferencePattern {
            constructive_nodes: vec![],
            destructive_nodes: vec![],
            pattern: 0.5,
            coherence: 0.7,
        };

        // First get should miss
        assert!(cache.get(&coefficients).is_none());
        assert_eq!(cache.stats.misses, 1);

        // Store the pattern
        cache.store(&coefficients, pattern.clone());

        // Second get should hit
        let retrieved = cache.get(&coefficients);
        assert!(retrieved.is_some());
        let pattern_value = retrieved.unwrap().pattern;
        assert_eq!(cache.stats.hits, 1);
        assert_eq!(pattern_value, 0.5);
    }
