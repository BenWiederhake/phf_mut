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

use {PerfectHash, HashInverse, Map, Set};

/* === Example use case === */

struct Pairs {
    n: usize,
}

impl Pairs {
    pub fn new(n: usize) -> Self {
        Pairs { n: n }
    }

    fn sort((u, v): (usize, usize)) -> (usize, usize) {
        if u > v {
            (v, u)
        } else {
            (u, v)
        }
    }

    fn size_when(n: usize) -> usize {
        (n + 1) * n / 2
    }
}

impl PerfectHash for Pairs {
    type K = (usize, usize);

    fn hash(&self, k: Self::K) -> usize {
        let (a, b) = Self::sort(k);
        a + Self::size_when(b)
    }

    fn size(&self) -> usize {
        Self::size_when(self.n)
    }
}

impl HashInverse for Pairs {
    fn invert(&self, index: usize) -> Self::K {
        assert!(index < self.size());
        /* Count down from n-1 to 0. */
        for sub in 0..self.n {
            let use_b = self.n - 1 - sub;

            let offset = Self::size_when(use_b);
            if offset <= index {
                /* Found the largest 'use_b' that results
                   in a compatible offset */
                let use_a = index - offset;
                assert!(use_a <= use_b);
                return (use_a, use_b);
            }
        }
        panic!("Unsigned integer {} appears to be strictly smaller than zero 0.",
               index);
    }
}

/* === Actual tests: Map === */

#[test]
fn test_map_basics() {
    let mut mymap = Map::new(Pairs::new(10));
    mymap.insert((3, 7), String::from("Hello"));
    mymap[(7, 3)].push(' ');
    mymap.insert((4, 3), String::from("lovely"));
    mymap.insert((2, 9), String::from("World!"));
    assert_eq!("Hello ", mymap.get((3, 7)));
    assert_eq!("Hello ", mymap.get((7, 3)));
    assert_eq!("", mymap.get((2, 2)));
    assert_eq!("World!", mymap[(2, 9)]);
    assert_eq!("World!", mymap[(9, 2)]);
    assert_eq!("", mymap.get((7, 4)));
    assert_eq!("", mymap.get((6, 6)));
}

#[test]
fn test_map_iter() {
    let mut mymap = Map::new(Pairs::new(3));
    mymap.insert((0, 1), 42);
    mymap.insert((1, 2), 123);
    mymap.insert((1, 1), 0xCAFE);
    mymap[(1, 0)] = 5;
    let value_vec = mymap.values().map(|x| *x).collect::<Vec<_>>();
    assert_eq!(vec![0, 5, 0xCAFE, 0, 123, 0], value_vec);
    let entry_vec = mymap.iter().map(|((a, b), &v)| (a, b, v))
                         .collect::<Vec<_>>();
    assert_eq!(vec![(0, 0, 0), (0, 1, 5),
                    (1, 1, 0xCAFE), (0, 2, 0),
                    (1, 2, 123), (2, 2, 0),], entry_vec);
}

/* === Actual tests: Set === */

#[test]
fn test_set_basics() {
    let mut myset = Set::new(Pairs::new(10));
    myset.insert((7, 6));
    myset.insert((4, 3));
    assert_eq!(true, myset.contains((7, 6)));
    assert_eq!(true, myset.contains((6, 7)));
    assert_eq!(false, myset.contains((7, 8)));
    assert_eq!(true, myset.contains((4, 3)));
    assert_eq!(false, myset.contains((9, 8)));
    assert_eq!(false, myset.contains((1, 0)));
    assert_eq!(false, myset.contains((0, 1)));
    assert_eq!(false, myset.contains((5, 5)));

    myset.insert((1, 0));
    assert_eq!(true, myset.contains((7, 6)));
    assert_eq!(true, myset.contains((6, 7)));
    assert_eq!(false, myset.contains((7, 8)));
    assert_eq!(true, myset.contains((4, 3)));
    assert_eq!(false, myset.contains((9, 8)));
    assert_eq!(true, myset.contains((1, 0))); /* Change */
    assert_eq!(true, myset.contains((0, 1))); /* Change */
    assert_eq!(false, myset.contains((5, 5)));

    myset.erase((4, 3));
    assert_eq!(true, myset.contains((7, 6)));
    assert_eq!(true, myset.contains((6, 7)));
    assert_eq!(false, myset.contains((7, 8)));
    assert_eq!(false, myset.contains((4, 3))); /* Change */
    assert_eq!(false, myset.contains((9, 8)));
    assert_eq!(true, myset.contains((1, 0)));
    assert_eq!(true, myset.contains((0, 1)));
    assert_eq!(false, myset.contains((5, 5)));
}

#[test]
fn test_set_iter() {
    let mut myset = Set::new(Pairs::new(10));
    myset.insert((7, 6));
    myset.insert((4, 3));
    myset.insert((1, 0));
    myset.insert((1, 4));
    let myset: Set<_> = myset;
    let as_vec = myset.iter().collect::<Vec<_>>();
    assert_eq!(vec![(0, 1), (1, 4), (3, 4), (6, 7)], as_vec);
}
