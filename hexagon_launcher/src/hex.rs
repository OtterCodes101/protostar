#![allow(dead_code)]

use crate::{APP_SIZE, PADDING};
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default)]
pub struct Hex {
	q: isize,
	r: isize,
	s: isize,
}

pub const HEX_CENTER: Hex = Hex { q: 0, r: 0, s: 0 };
pub const HEX_DIRECTION_VECTORS: [Hex; 6] = [
	Hex { q: 1, r: 0, s: -1 },
	Hex { q: 1, r: -1, s: 0 },
	Hex { q: 0, r: -1, s: 1 },
	Hex { q: -1, r: 0, s: 1 },
	Hex { q: -1, r: 1, s: 0 },
	Hex { q: 0, r: 1, s: -1 },
];

impl Hex {
	pub fn new(q: isize, r: isize, s: isize) -> Self {
		Hex { q, r, s }
	}

	pub fn get_coords(&self) -> [f32; 3] {
		let x = 3.0 / 2.0 * (APP_SIZE + PADDING) / 2.0 * (-self.q - self.s) as f32;
		let y = 3.0_f32.sqrt() * (APP_SIZE + PADDING) / 2.0
			* ((-self.q - self.s) as f32 / 2.0 + self.s as f32);
		[x, y, 0.0]
	}

	pub fn neighbor(self, direction: usize) -> Self {
		self + HEX_DIRECTION_VECTORS[direction]
	}

	pub fn scale(self, factor: isize) -> Self {
		Hex::new(self.q * factor, self.r * factor, self.s * factor)
	}

	/// outputs a hexagon at an outward spiral at position i, where i=0 is the center.
	pub fn spiral(i: usize) -> Self {
		if i == 0 {
			return HEX_CENTER;
		}

		// Calculate which ring this index belongs to
		let mut ring = 1;
		let mut cells_before_ring = 1;
		while i >= cells_before_ring + (ring * 6) {
			cells_before_ring += ring * 6;
			ring += 1;
		}

		// Calculate position within the ring
		let pos_in_ring = i - cells_before_ring;
		
		// Start at the top of the ring
		let mut hex = HEX_CENTER + HEX_DIRECTION_VECTORS[4].scale(ring as isize);
		
		// Move clockwise around the ring one step at a time
		for _ in 0..pos_in_ring {
			let current_side = (pos_in_ring / ring) % 6;
			hex = hex + HEX_DIRECTION_VECTORS[(current_side + 1) % 6];
		}

		hex
	}
}
impl Add for Hex {
	type Output = Hex;

	fn add(self, rhs: Self) -> Self::Output {
		Hex::new(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
	}
}
