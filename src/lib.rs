pub trait Hasher {
	type K;
	fn hash(&self, k: Self::K) -> usize;
	fn size(&self) -> usize;
}

// TODO: Eventually move to own module.
pub struct Map<V: Default, H: Hasher> {
	hash: H,
	backing: Vec<V>,
}

impl<V: Default + Clone, H: Hasher> Map<V, H> {
	pub fn new(hash: H) -> Self {
		let size = hash.size();
		Map {
			hash: hash,
			backing: vec![V::default(); size],
		}
	}

	pub fn print_size(&self) {
		println!("Size is {}.", self.hash.size());
	}
}
