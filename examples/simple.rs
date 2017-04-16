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

extern crate phf_mut;
use phf_mut::{Hasher, Map};

struct Cuboid {
    w: usize,
    h: usize,
    d: usize,
}

impl Cuboid {
    pub fn new(w: usize, h: usize, d: usize) -> Self {
        assert!(w > 0);
        assert!(h > 0);
        assert!(d > 0);
        Cuboid { w: w, h: h, d: d }
    }
}

impl Hasher for Cuboid {
    type K = (usize, usize, usize);

    fn hash(&self, (x, y, z): Self::K) -> usize {
        x + self.w * y + self.w * self.h * z
    }

    fn size(&self) -> usize {
        self.hash((self.w - 1, self.h - 1, self.d - 1)) + 1
    }
}

fn main() {
    let mut mymap = Map::new(Cuboid::new(10, 20, 30));
    mymap.insert((0, 3, 7), "Hello ");
    mymap.insert((4, 19, 13), "lovely");
    mymap.insert((9, 8, 29), "World!");
    print!("{}", mymap.get((0, 3, 7))); // "Hello "
    print!("{}", mymap.get((2, 15, 2))); // ""
    print!("{}", mymap.get((9, 8, 29))); // "World!"
    print!("{}", mymap.get((7, 4, 23))); // ""
    println!();
}
