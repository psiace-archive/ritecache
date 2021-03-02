use crate::cache::Cacheable;

use std::{borrow::Cow, hash::Hash};

pub struct LfuCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    cache: lfu::LFUCache<K, V>,
}

impl<K, V> LfuCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    pub fn new(cap: usize) -> LfuCache<K, V> {
        LfuCache { cache: lfu::LFUCache::with_capacity(cap).unwrap() }
    }
}

impl<K, V> Cacheable<K, V> for LfuCache<K, V>
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
        self.cache.set(k, v);
    }

    fn remove(&mut self, k: K) -> bool {
        self.cache.remove(k)
    }
}
