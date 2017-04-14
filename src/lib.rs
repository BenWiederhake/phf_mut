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

	pub fn print_size(&self) {
		println!("Size is {}.", self.hash.size());
	}

	pub fn insert(&mut self, k: H::K, v: V) {
		self.backing[self.hash.hash(k)] = v;
	}

	pub fn get(&mut self, k: H::K) -> &V {
		&self.backing[self.hash.hash(k)]
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
	};

	let mut mymap = Map::new(Cuboid::new(10, 20, 30));
	mymap.print_size();
	mymap.insert((0, 3, 7), "Hello ");
	mymap.insert((4, 19, 13), "lovely");
	mymap.insert((9, 8, 29), "World!");
	print!("{}", mymap.get((0, 3, 7))); // "Hello "
	print!("{}", mymap.get((2, 15, 2))); // ""
	print!("{}", mymap.get((9, 8, 29))); // "World!"
	print!("{}", mymap.get((7, 4, 23))); // ""
	println!();
}
