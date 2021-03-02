use crate::cache::Cacheable;

use std::{borrow::Cow, hash::Hash};

pub struct LruCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    cache: lru::LruCache<K, V>,
}

impl<K, V> LruCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    pub fn new(cap: usize) -> LruCache<K, V> {
        LruCache { cache: lru::LruCache::new(cap) }
    }
}

impl<K, V> Cacheable<K, V> for LruCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    fn get(&mut self, k: &K) -> Option<Cow<'_, V>> {
        self.cache.get_mut(k).map(|x| Cow::Borrowed(&*x))
    }

    fn contains(&mut self, k: &K) -> bool {
        self.cache.contains(k)
    }

    fn set(&mut self, k: K, v: V) {
        self.cache.put(k, v);
    }

    fn remove(&mut self, k: K) -> bool {
        self.cache.pop(&k).is_some()
    }
}
