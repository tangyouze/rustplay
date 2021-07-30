fn main() {
	let arr: [u32; 4] = [5, 6, 7, 9];
	let t = summary(&arr);
	match t {
		None => {
			println!("none")
		}
		Some(t) => {
			println!("some {}", t)
		}
	}

	let arr: [u32; 4] = [2100000000, 2100000000, 2100000000, 2100000000];
	let t = summary(&arr);
	match t {
		None => {
			println!("none, overflow!!")
		}
		Some(t) => {
			println!("some {}", t)
		}
	}
}

fn summary(x: &[u32]) -> Option<u32> {
	let mut t: u32 = 0;
	for i in 0..x.len() {
		match t.checked_add(x[i]) {
			Some(v) => t = v,
			None => return None,
		}
	}

	return Some(t);
}
