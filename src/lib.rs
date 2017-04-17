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

extern crate bit_vec;

use std::fmt;
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

pub trait Hasher {
    type K;
    fn hash(&self, k: Self::K) -> usize;
    fn size(&self) -> usize;
}

pub trait HasherInverse: Hasher {
    fn invert(&self, hash: usize) -> Self::K;
}

pub struct Map<V, H> {
    hash: H,
    backing: Box<[V]>,
}

impl<V: Default, H: Hasher> Map<V, H> {
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

impl<V, H: HasherInverse> Map<V, H> {
    pub fn iter(&self) -> MapIter<H, V> {
        MapIter {
            backing: self.backing.iter(),
            hash: &self.hash,
            pos: 0,
        }
    }

    pub fn iter_mut(&mut self) -> MapIterMut<H, V> {
        MapIterMut {
            backing: self.backing.iter_mut(),
            hash: &self.hash,
            pos: 0,
        }
    }
}

pub struct MapIter<'a, H: 'a, V: 'a> {
    // TODO: Sub-optimal approach.  Now the position is saved twice.
    backing: std::slice::Iter<'a, V>,
    hash: &'a H,
    pos: usize,
}

impl<'a, H: HasherInverse, V: 'a> Iterator for MapIter<'a, H, V> {
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

impl<'a, H: HasherInverse, V: 'a> Iterator for MapIterMut<'a, H, V> {
    type Item = (H::K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.backing.next().map(|value| {
            let key = self.hash.invert(self.pos);
            self.pos += 1;
            (key, value)
        })
    }
}

impl<V, H: Hasher> Map<V, H> {
    pub fn from_initial(hash: H, init: Vec<V>) -> Self {
        let size = hash.size();
        assert_eq!(size, init.len());
        Map {
            hash: hash,
            backing: init.into_boxed_slice(),
        }
    }

    pub fn insert(&mut self, k: H::K, v: V) {
        self.backing[self.hash.hash(k)] = v;
    }

    pub fn get(&self, k: H::K) -> &V {
        &self.backing[self.hash.hash(k)]
    }

    pub fn get_mut(&mut self, k: H::K) -> &mut V {
        &mut self.backing[self.hash.hash(k)]
    }
}

impl<V, H> Map<V, H> {
    pub fn is_empty(&self) -> bool {
        self.backing.is_empty()
    }

    pub fn len(&self) -> usize {
        self.backing.len()
    }

    pub fn values(&self) -> std::slice::Iter<V> {
        self.backing.iter()
    }

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

impl<V, H: Hasher> Index<H::K> for Map<V, H> {
    type Output = V;

    fn index(&self, k: H::K) -> &V {
        self.get(k)
    }
}

impl<V, H: Hasher> IndexMut<H::K> for Map<V, H> {
    fn index_mut(&mut self, k: H::K) -> &mut V {
        self.get_mut(k)
    }
}

pub struct Set<H> {
    hash: H,
    backing: bit_vec::BitVec,
}

impl<H: Hasher> Set<H> {
    pub fn new(hash: H) -> Self {
        let size = hash.size();
        Set {
            hash: hash,
            backing: bit_vec::BitVec::from_elem(size, false),
        }
    }

    pub fn insert(&mut self, k: H::K) -> bool {
        let idx = self.hash.hash(k);
        let ret = self.backing.get(idx).unwrap();
        self.backing.set(idx, true);
        ret
    }

    pub fn erase(&mut self, k: H::K) -> bool {
        let idx = self.hash.hash(k);
        let ret = self.backing.get(idx).unwrap();
        self.backing.set(idx, false);
        ret
    }

    fn has(&self, index: usize) -> bool {
        self.backing.get(index).unwrap()
    }

    pub fn contains(&self, k: H::K) -> bool {
        let idx = self.hash.hash(k);
        self.has(idx)
    }

    pub fn iter(&self) -> SetIter<H> {
        SetIter {
            next: self.backing.len(),
            set: self,
        }
    }
}

// TODO: How to impl IntoIterator for Set<H> itself?
impl<'a, H: HasherInverse> IntoIterator for &'a Set<H> {
    type Item = H::K;
    type IntoIter = SetIter<'a, H>;

    fn into_iter(self) -> SetIter<'a, H> {
        self.iter()
    }
}

impl<H: Hasher + Default> Default for Set<H> {
    fn default() -> Self {
        Self::new(H::default())
    }
}

pub struct SetIter<'a, H: Hasher + 'a> {
    next: usize,
    set: &'a Set<H>,
}

impl<'a, H: HasherInverse> Iterator for SetIter<'a, H> {
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
