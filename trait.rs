struct Cow { name: &'static str }
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Cow {
    fn new(name: &'static str) -> Cow {
        Cow { name }
    }
}

impl Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }

    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name());
            self.naked = true;
        }
    }
}

impl Animal for Cow {
    fn name(&self) -> &'static str { self.name }
    fn noise(&self) -> &'static str { "moooooo!" }
}

impl Animal for Sheep {
    fn name(&self) -> &'static str { self.name }
    fn noise(&self) -> &'static str {
        if self.is_naked() { "baaaaah?" } else { "baaaaah!" }
    }
}

fn choose_animal(input: &str, s: &mut i32, c: &mut i32) -> Option<Box<dyn Animal>> {
    match input.to_lowercase().as_str() {
        "sheep" => {
            *s += 1;
            Some(Box::new(Sheep::new(format!("Random Sheep {}", s))))
        }
        "cow" => {
            *c += 1;
            Some(Box::new(Cow::new(format!("Random Cow {}", c))))
        }
        _ => None,
    }
}

fn main() {
    let mut s = 0;
    let mut c = 0;

    println!("Type 'sheep' or 'cow':");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    match choose_animal(input, &mut s, &mut c) {
        Some(animal) => {
            println!("You've chosen an animal:");
            animal.talk();
        }
        None => println!("Invalid choice, please type 'sheep' or 'cow'."),
    }

    let mut dolly = Sheep::new("Dolly".to_string());
    dolly.talk();
    dolly.shear();
    dolly.talk();

}

/*
Specificités du language:
mut = mutable variable (value which can change otherwise values are immutable)
declare datatype -> C-type-size : i32, f64...
declare a variable with or without explicit datatype if possible:
let t = true;
let f: bool = false;

let x = 5;
let y = &x;
assert_eq!(5, x);
assert_eq!(5, *y);
assert_eq!(5, y); //error : no implementation for `{integer} == &{integer}`
let x = 5;
let y = &mut x; //error cannot borrow as mutable
*/