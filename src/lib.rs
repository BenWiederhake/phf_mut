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

    pub fn contains(&self, k: H::K) -> bool {
        let idx = self.hash.hash(k);
        self.backing.get(idx).unwrap()
    }
}

impl<H: Hasher + Default> Default for Set<H> {
    fn default() -> Self {
        Self::new(H::default())
    }
}
