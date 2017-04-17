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
```

## TODOs

* Make it feature-complete?
    * `Default`, `Index`, `Clone`, `PartialEq`, `Eq`
    * nicer `Debug` for `HashInverse`-instances
    * consuming `IntoIterator`s
    * `::collect` target? (`FromIterator<(K,V)>`)
    * Likewise, `Extend<K,V>`
    * all `impl IntoIterator for Map`
    * Provide a general `impl Iterator<Item=H::K>` for all `HashInverse`?
* Ask people for feedback on making it "Idiomatic Rust"
* Try to compile as `nostdlib`, after all I don't use anything anyway, I guess.

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/masked_permute/issues/new) or submit PRs.
