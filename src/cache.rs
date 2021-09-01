pub mod lru;

use std::borrow::Borrow;
use std::hash::{BuildHasher, Hash};

use crate::Meter;

/// A trait for a cache.
pub trait Cache<K, V, S, M>
where
    K: Eq + Hash,
    S: BuildHasher,
    M: Meter<K, V>,
{
    fn with_meter_and_hasher(cap: u64, meter: M, hash_builder: S) -> Self;
    fn get<'a, Q>(&'a mut self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    fn get_mut<'a, Q>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    fn peek<'a, Q>(&'a self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    fn peek_mut<'a, Q>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    fn peek_by_policy(&self) -> Option<(&K, &V)>;
    fn put(&mut self, k: K, v: V) -> Option<V>;
    fn pop<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    fn pop_by_policy(&mut self) -> Option<(K, V)>;
    fn contains<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn capacity(&self) -> u64;
    fn set_capacity(&mut self, cap: u64);
    fn size(&self) -> u64;
    fn clear(&mut self);
}
