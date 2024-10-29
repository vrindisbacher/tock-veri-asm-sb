#![allow(dead_code)]

flux_rs::defs! {
    fn map_set<K, V>(m:Map<K, V>, k: K, v: V) -> Map<K, V> { map_store(m, k, v) }
    fn map_get<K, V>(m: Map<K, V>, k:K) -> V { map_select(m, k) }
}

use std::hash::Hash;

/// define a type indexed by a map
#[derive(Debug)]
#[flux_rs::opaque]
#[flux_rs::refined_by(vals: Map<K, V>)]
pub struct Regs<K, V> {
    inner: std::collections::HashMap<K, V>,
}

#[flux_rs::generics(K as base, V as base)]
impl<K, V> Regs<K, V> {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn(self: &strg Regs<K,V>[@m], k: K, v: V) ensures self: Regs<K,V>[map_set(m.vals, k, v)])]
    pub fn set(&mut self, k: K, v: V)
    where
        K: Eq + Hash,
    {
        self.inner.insert(k, v);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn(&Regs<K, V>[@m], &K[@k]) -> Option<&V[map_get(m.vals, k)]>)]
    pub fn get(&self, k: &K) -> Option<&V>
    where
        K: Eq + Hash,
    {
        self.inner.get(k)
    }
}
