fn main() {
	let light = TrafficLight::Red;
	println!("light is {}", light.time());
	// println!("plus  {}", plus_one(3));
	let five = Some(5);

	// let six = five.map(|i| i + 1);
	// println!("six {:?}", six)

	let number_list = vec![1,2,3,2];
	let res = largest(&number_list);

	println!("largest {}", res)
}

enum TrafficLight {
	Red,
	Green,
	Yellow,
}

impl TrafficLight {
	fn time(&self) -> u8 {
		60
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn largest<T : PartialOrd>(list: &[T]) -> &T {
	let mut lag = &list[0];
	for item in list {
		if item > lag {
			lag = item
		}
	}
	return lag;
}

