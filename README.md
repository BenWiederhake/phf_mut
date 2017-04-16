# phf_mut

> Perfectly hashed mutable containers.

Assume you want a map from keys to values, your key domain is small-ish,
and you already have a perfect hash function.

The `phf` package supports immutable, compile-time generated maps.
But what about mutable maps and sets?

It seems there isn't such a crate yet, so I wrote my own.

In the case of maps, assume some kind of default element.
Note that we will assume that the map will be rather full.
In the case of a sparse map, `HashMap` probably can't be beaten anyway.

My personal use case is a *completely* filled map and a default-constructible type.
So the container is always considered full.

In the case of sets, assume that the domain (set of possible keys)
is small enough to be representable by some kind of bitset.
Again, `HashSet` plus a custom wrapper (in order to override `PartialEq`)
is going to beat this implementation for very small domains, or for sparse sets.

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [TODOs](#todos)
- [Contribute](#contribute)

## Background

Example: you have an integer grid throughout a small cuboid,
and you want to store some `V` for most nodes of the grid.
You *could* write `myvec[x + w*y + w*h*z]` at every call site.
Or you could just write it once, pass this function as the perfect hash function,
and let this crate handle the rest.

## Install

Add at an appropriate position to your `Cargo.toml`:

```TOML
[dependencies]
phf_mut = { git = "https://github.com/BenWiederhake/phf_mut.git" }
```

That should be it.  You'll be glad to hear that `phf_mut` itself
does not have any dependencies.

## Usage

Just use it!  No dependencies, and it's short enough.
The complexity lies in coming up with a nice API,
not in writing the code.

```Rust
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
```

## TODOs

* Make it feature-complete?
    * `Default`, `Index`, `Clone`, `PartialEq`, `Eq`
    * nicer `Debug` for `HasherInverse`-instances
    * consuming `IntoIterator`s
    * `::collect` target? (`FromIterator<(K,V)>`)
    * Likewise, `Extend<K,V>`
* Ask people for feedback on making it "Idiomatic Rust"
* Try to compile as `nostdlib`, after all I don't use anything anyway, I guess.

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/masked_permute/issues/new) or submit PRs.
