struct Animal {
	age: i32,
}

type Cat = Animal;
type Dog = Animal;
type LoudDog = Dog;

trait Talk {
	fn talk(&self) -> ();
}

impl Talk for Animal {
	default fn talk(&self) { // note the use of the default 
		println!("<silence>");
	}
}

impl Talk for Cat {
	fn talk(&self) {
		println!("Meow");
	}
}

impl Talk for Dog {
	fn talk(&self) {
		println!("Woof!");
	}
}

impl Talk for LoudDog {
	fn talk(&self) {
		println!("WOOF!!");
	}
}



fn main() {
	let fluffy = Cat(Animal { age: 4 });
	let max = Dog(Animal { age: 2 });
	let neighbours_dog = LoudDog(Animal { age: 7 });

	fluffy.talk();
	max.talk();
	neighbours_dog.talk();
}
