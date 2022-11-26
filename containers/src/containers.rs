pub mod container {
    pub struct Container<K: Eq, V> {
        values: Vec<(K, V)>,
    }

    impl<K: Eq, V> Container<K, V> {
        /// Creates a new [`Container<K, V>`] instance.
        pub(crate) fn new() -> Self {
            Container { values: vec![] }
        }
        /// Inserts a new key-value pair into the container.
        pub(crate) fn insert(&mut self, key: K, value: V) {
            self.values.push((key, value));
        }
        /// Returns an optional reference to the value associated with the given key.
        pub(crate) fn get(&mut self, k: &K) -> Option<&V> {
            self.values.iter().find(|(key, _)| key == k).map(|(_, v)| v)
        }
    }
}
