use std::fmt;
use std::ops::{Index, IndexMut};

pub trait Hasher {
	type K;
	fn hash(&self, k: Self::K) -> usize;
	fn size(&self) -> usize;
}

// TODO: Eventually move to own module.
pub struct Map<V, H> {
	hash: H,
	backing: Box<[V]>,
}

impl<V: Default, H: Hasher> Map<V, H> {
	pub fn new(hash: H) -> Self {
		let size = hash.size();
		let mut vec : Vec<V> = Vec::with_capacity(size);
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

impl<V, H> fmt::Debug for Map<V, H> where V: fmt::Debug {
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

#[test]
fn it_works() {
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

	let mut mymap = Map::new(Cuboid::new(10, 20, 30));
	mymap.insert((0, 3, 7), "Hello ".to_string());
	mymap[(0, 3, 7)].push(' ');
	mymap.insert((4, 19, 13), "lovely".to_string());
	mymap.insert((9, 8, 29), "World!".to_string());
	print!("{}", mymap.get((0, 3, 7))); // "Hello "
	print!("{}", mymap.get((2, 15, 2))); // ""
	print!("{}", mymap[(9, 8, 29)]); // "World!"
	print!("{}", mymap.get((7, 4, 23))); // ""
	println!();
}
