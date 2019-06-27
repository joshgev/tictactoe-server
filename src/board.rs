use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Piece {
	X,
	O
}

enum RowOrCol {
	Row(usize),
	Col(usize)
}

enum Diagonal {
	One,
	Two
}

#[derive(Serialize)]
pub struct Board {
	data: [[Option<Piece>;3]; 3]
}

impl Board {
	pub fn set(&mut self, r: usize, c: usize, piece: Piece) {
		self.data[r][c] = Some(piece)
	}

	pub fn new() -> Self {
		return Self {
			data: [
				[None, None, None], 
				[None, None, None],
				[None, None, None]
			]
		}
	}

	pub fn get_empty_positions(&self) -> Vec<(usize, usize)> {
		let mut ret = vec![];
		for r in 0..3 {
			for c in 0..3 {
				if let None = self.data[r][c] {
					ret.push((r, c))
				}
			}
		}
		ret
	}

	fn check_row_or_col(&self, roc: RowOrCol) -> Option<Piece> {
		let mut set = HashSet::new();
		for x in 0..3 {
			let p = match roc {
				RowOrCol::Row(r) => &self.data[r][x],
				RowOrCol::Col(c) => &self.data[x][c]
			};
			
			match p {
				None => return None,
				Some(ref x) => set.insert(x)
			};
		}

		let l: Vec<_> = set.drain().collect();
		if l.len() == 1 {
			return Some(l[0].clone())
		}
		None
	}

	fn check_diagonals(&self, diagonal: Diagonal) -> Option<Piece> {
		let mut set = HashSet::new();
		for x in 0..3 {
			let p = match diagonal {
				Diagonal::One => &self.data[x][x],
				Diagonal::Two => &self.data[x][2-x],
			};

			match p {
				None => return None,
				Some(ref x) => set.insert(x)
			};
		}

		let l : Vec<_> = set.drain().collect();
		if l.len() == 1 {
			return Some(l[0].clone())
		}
		None
	}


	pub fn get_winning_piece(&self) -> Option<Piece> {
		// First check rows
		for r in 0..3 {
			if let Some(p) = self.check_row_or_col(RowOrCol::Row(r)) {
				return Some(p)
			}
		}

		// Now cols
		for c in 0..3 {
			if let Some(p) = self.check_row_or_col(RowOrCol::Col(c)) {
				return Some(p)
			}
		}

		if let Some(p) = self.check_diagonals(Diagonal::One) {
			return Some(p)
		}

		if let Some(p) = self.check_diagonals(Diagonal::Two) {
			return Some(p)
		}
	
		None
	}

}