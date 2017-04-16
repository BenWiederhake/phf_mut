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

use {Hasher, HasherInverse, Map, Set};

/* === Example use case === */

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

impl HasherInverse for Cuboid {
    fn invert(&self, index: usize) -> Self::K {
        let x = index % self.w;
        let index = index / self.w;
        let y = index % self.h;
        let index = index / self.h;
        assert!(index < self.d);
        let z = index;
        (x, y, z)
    }
}

/* === Actual tests: Map === */

#[test]
fn test_map_basics() {
    let mut mymap = Map::new(Cuboid::new(10, 20, 30));
    mymap.insert((0, 3, 7), "Hello".to_string());
    mymap[(0, 3, 7)].push(' ');
    mymap.insert((4, 19, 13), "lovely".to_string());
    mymap.insert((9, 8, 29), "World!".to_string());
    assert_eq!("Hello ", mymap.get((0, 3, 7)));
    assert_eq!("", mymap.get((2, 15, 2)));
    assert_eq!("World!", mymap[(9, 8, 29)]);
    assert_eq!("", mymap.get((7, 4, 23)));
}

/* === Actual tests: Set === */

#[test]
fn test_set_basics() {
    let mut myset = Set::new(Cuboid::new(10, 20, 30));
    myset.insert((7, 6, 5));
    myset.insert((4, 3, 2));
    myset.insert((1, 0, 8));
    assert_eq!(true, myset.contains((7, 6, 5)));
    assert_eq!(false, myset.contains((7, 8, 9)));
    assert_eq!(true, myset.contains((4, 3, 2)));
    assert_eq!(false, myset.contains((9, 10, 11)));
    assert_eq!(true, myset.contains((1, 0, 8)));
    assert_eq!(false, myset.contains((5, 15, 25)));

    myset.erase((4, 3, 2));
    assert_eq!(true, myset.contains((7, 6, 5)));
    assert_eq!(false, myset.contains((7, 8, 9)));
    assert_eq!(false, myset.contains((4, 3, 2))); /* Change */
    assert_eq!(false, myset.contains((9, 10, 11)));
    assert_eq!(true, myset.contains((1, 0, 8)));
    assert_eq!(false, myset.contains((5, 15, 25)));
}

#[test]
fn test_set_iter() {
    let mut myset = Set::new(Cuboid::new(10, 20, 30));
    myset.insert((7, 6, 5));
    myset.insert((4, 3, 2));
    myset.insert((1, 0, 8));
    let myset: Set<_> = myset;
    let as_vec = myset.iter().collect::<Vec<_>>();
    assert_eq!(vec![(4, 3, 2), (7, 6, 5), (1, 0, 8)], as_vec);
}
