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
use phf_mut::{PerfectHash, Map};

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

fn main() {
    let mut mymap = Map::new(Pairs::new(10));
    mymap.insert((3, 7), String::from("Hello"));
    mymap[(7, 3)].push(' ');
    mymap.insert((4, 3), String::from("lovely"));
    mymap.insert((2, 9), String::from("World!"));
    print!("{}", mymap.get((3, 7))); // "Hello "
    print!("{}", mymap.get((2, 2))); // ""
    print!("{}", mymap.get((2, 9))); // "World!"
    print!("{}", mymap.get((7, 4))); // ""
    println!();
}
