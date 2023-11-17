// phf_mut – Perfectly hashed mutable containers
// Copyright (C) 2017  Ben Wiederhake
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Perfectly hashed mutable containers.

extern crate bit_vec;

use std::fmt;
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

/// The perfect hash function to be used in all further constructions.
pub trait PerfectHash {
    type K;
    fn hash(&self, k: Self::K) -> usize;
    fn size(&self) -> usize;
}

/// Inverse operation of the perfect hash function.
/// This is necessary for all operations that *generate* key values,
/// for example iteration.
pub trait HashInverse: PerfectHash {
    fn invert(&self, hash: usize) -> Self::K;

    /// Create a new iterator over the hash domain.
    fn iter(&self) -> KeyIter<Self> {
        KeyIter { next: 0, hash: self }
    }
}

/// Iterator over the domain of a `PerfectHash`.
pub struct KeyIter<'a, H: ?Sized + 'a> {
    hash: &'a H,
    next: usize,
}

impl<'a, H: HashInverse> Iterator for KeyIter<'a, H> {
    type Item = H::K;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.hash.size();
        if self.next == size {
            self.next = 0;
            None
        } else {
            let idx = self.next;
            self.next += 1;
            Some(self.hash.invert(idx))
        }
    }
}

/// A mutable, perfectly-hashed map.  Note that a `Map` is always full,
/// so you might prefer `std::collections::HashMap` for sparse maps.
pub struct Map<V, H> {
    hash: H,
    backing: Box<[V]>,
}

impl<V: Default, H: PerfectHash> Map<V, H> {
    /// Create a new `Map` full default values.
    /// Also see `from_initial` and `from_element`.
    pub fn new(hash: H) -> Self {
        let size = hash.size();
        let mut vec: Vec<V> = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(V::default());
        }
        Map {
            hash: hash,
            backing: vec.into_boxed_slice(),
        }
    }
}

impl<V: Clone, H: PerfectHash> Map<V, H> {
    /// Create a new `Map` full of copies of some value.
    /// Also see `from_initial` and `new`.
    pub fn from_element(hash: H, value: &V) -> Self {
        let size = hash.size();
        let mut vec: Vec<V> = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(value.clone());
        }
        Map {
            hash: hash,
            backing: vec.into_boxed_slice(),
        }
    }
}

impl<V, H: HashInverse> Map<V, H> {
    /// Directly create a new iterator over entries:
    /// `Iterator<Item=(K,&V)>`.
    pub fn iter(&self) -> MapIter<H, V> {
        MapIter {
            backing: self.backing.iter(),
            hash: &self.hash,
            pos: 0,
        }
    }

    /// Directly create a new iterator over mutable entries:
    /// `Iterator<Item=(K,&mut V)>`.
    pub fn iter_mut(&mut self) -> MapIterMut<H, V> {
        MapIterMut {
            backing: self.backing.iter_mut(),
            hash: &self.hash,
            pos: 0,
        }
    }
}

impl<'a, V, H: HashInverse> IntoIterator for &'a Map<V, H> {
    type Item = (H::K, &'a V);
    type IntoIter = MapIter<'a, H, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, H: HashInverse> IntoIterator for &'a mut Map<V, H> {
    type Item = (H::K, &'a mut V);
    type IntoIter = MapIterMut<'a, H, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct MapIter<'a, H: 'a, V: 'a> {
    // TODO: Sub-optimal approach.  Now the position is saved twice.
    backing: std::slice::Iter<'a, V>,
    hash: &'a H,
    pos: usize,
}

impl<'a, H: HashInverse, V: 'a> Iterator for MapIter<'a, H, V> {
    type Item = (H::K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.backing.next().map(|value| {
            let key = self.hash.invert(self.pos);
            self.pos += 1;
            (key, value)
        })
    }
}

pub struct MapIterMut<'a, H: 'a, V: 'a> {
    // TODO: Sub-optimal approach.  Now the position is saved twice.
    backing: std::slice::IterMut<'a, V>,
    hash: &'a H,
    pos: usize,
}

impl<'a, H: HashInverse, V: 'a> Iterator for MapIterMut<'a, H, V> {
    type Item = (H::K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.backing.next().map(|value| {
            let key = self.hash.invert(self.pos);
            self.pos += 1;
            (key, value)
        })
    }
}

impl<V, H: PerfectHash> Map<V, H> {
    /// Create a new `Map` from a given vector of values.
    /// The vector must be compatible to the PerfectHash.
    /// Also see `new` and `from_element`.
    pub fn from_initial(hash: H, init: Vec<V>) -> Self {
        let size = hash.size();
        assert_eq!(size, init.len());
        Map {
            hash: hash,
            backing: init.into_boxed_slice(),
        }
    }

    /// Overwrite the currently stored value for key `k` by `v`.
    /// The name `insert` is s homage to `HashMap::insert`.
    pub fn insert(&mut self, k: H::K, v: V) {
        self.backing[self.hash.hash(k)] = v;
    }

    /// Swaps the currently stored value for key `k` with `v`,
    /// so the old value is now stored in `v`.
    pub fn swap(&mut self, k: H::K, v: &mut V) {
        std::mem::swap(&mut self.backing[self.hash.hash(k)], v);
    }

    /// Directly get a reference the value for key `k`.
    /// Also see the `Index` implementation.
    pub fn get(&self, k: H::K) -> &V {
        &self.backing[self.hash.hash(k)]
    }

    /// Directly get a mutable reference the value for key `k`.
    /// Also see the `Index` implementation.
    pub fn get_mut(&mut self, k: H::K) -> &mut V {
        &mut self.backing[self.hash.hash(k)]
    }
}

impl<V, H> Map<V, H> {
    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.backing.is_empty()
    }

    /// Returns the amount of entries,
    /// which is always equal to the hasher's domain (i.e., `hasher.size()`).
    pub fn len(&self) -> usize {
        self.backing.len()
    }

    /// Directly create a new iterator over the values:
    /// `Iterator<Item=&V>`.
    pub fn values(&self) -> std::slice::Iter<V> {
        self.backing.iter()
    }

    /// Directly create a new iterator over the mutable values:
    /// `Iterator<Item=&mut V>`.
    pub fn values_mut(&mut self) -> std::slice::IterMut<V> {
        self.backing.iter_mut()
    }
}

impl<V, H> fmt::Debug for Map<V, H>
    where V: fmt::Debug
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", &*self.backing)
    }
}

impl<V: Clone, H: Clone> Clone for Map<V, H> {
    fn clone(&self) -> Self { Self {
        hash: self.hash.clone(),
        backing: self.backing.clone(),
    }}
}

impl<V, H: PerfectHash> Index<H::K> for Map<V, H> {
    type Output = V;

    fn index(&self, k: H::K) -> &V {
        self.get(k)
    }
}

impl<V, H: PerfectHash> IndexMut<H::K> for Map<V, H> {
    fn index_mut(&mut self, k: H::K) -> &mut V {
        self.get_mut(k)
    }
}

/// A mutable, perfectly-hashed set.  Note that a small domain is recommended.
/// For sparse sets, you might prefer `std::collections::HashSet`.
pub struct Set<H> {
    hash: H,
    backing: bit_vec::BitVec,
}

impl<H: PerfectHash> Set<H> {
    /// Create a new, empty set.
    pub fn new(hash: H) -> Self {
        let size = hash.size();
        Set {
            hash: hash,
            backing: bit_vec::BitVec::from_elem(size, false),
        }
    }

    /// Insert a key into the set, so that `contains`
    /// for an equal key returns `true` in the future.
    /// Returns whether this key already was in the set.
    pub fn insert(&mut self, k: H::K) -> bool {
        let idx = self.hash.hash(k);
        let ret = self.backing.get(idx).unwrap();
        self.backing.set(idx, true);
        ret
    }

    /// Erases a key from the set, so that `contains`
    /// for an equal key returns `false` in the future.
    /// Returns whether this key already was in the set.
    pub fn erase(&mut self, k: H::K) -> bool {
        let idx = self.hash.hash(k);
        let ret = self.backing.get(idx).unwrap();
        self.backing.set(idx, false);
        ret
    }

    fn has(&self, index: usize) -> bool {
        self.backing.get(index).unwrap()
    }

    /// Returns whether the key is in the set.
    pub fn contains(&self, k: H::K) -> bool {
        let idx = self.hash.hash(k);
        self.has(idx)
    }

    pub fn is_empty(&self) -> bool {
        !self.backing.any()
    }

    pub fn is_full(&self) -> bool {
        self.backing.all()
    }
}

impl<H: HashInverse> Set<H> {
    /// Create an iterator over the contained keys.
    pub fn iter(&self) -> SetIter<H> {
        SetIter {
            next: self.backing.len(),
            set: self,
        }
    }
}

impl<'a, H: HashInverse> IntoIterator for &'a Set<H> {
    type Item = H::K;
    type IntoIter = SetIter<'a, H>;

    fn into_iter(self) -> SetIter<'a, H> {
        self.iter()
    }
}

impl<H: PerfectHash + Default> Default for Set<H> {
    fn default() -> Self {
        Self::new(H::default())
    }
}

impl<H: Clone> Clone for Set<H> {
    fn clone(&self) -> Self { Self {
        hash: self.hash.clone(),
        backing: self.backing.clone(),
    }}
}

pub struct SetIter<'a, H: PerfectHash + 'a> {
    next: usize,
    set: &'a Set<H>,
}

impl<'a, H: HashInverse> Iterator for SetIter<'a, H> {
    type Item = H::K;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.set.hash.size();
        if self.next == size {
            /* Fresh start, or wrapped. */
            self.next = 0;
        } else {
            self.next += 1;
        }
        while self.next < size && !self.set.has(self.next) {
            self.next += 1;
        }
        if self.next == size {
            None
        } else {
            Some(self.set.hash.invert(self.next))
        }
    }
}
