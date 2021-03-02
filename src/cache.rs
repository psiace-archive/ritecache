mod lfu;
mod lru;

pub use self::lfu::LfuCache;
pub use self::lru::LruCache;

use std::{borrow::Cow, hash::Hash};

pub trait Cacheable<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    fn get(&mut self, key: &K) -> Option<Cow<'_, V>>;
    fn contains(&mut self, key: &K) -> bool;
    fn set(&mut self, key: K, value: V);
    fn remove(&mut self, key: K) -> bool;
}
