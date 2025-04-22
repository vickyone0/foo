

struct Labrador {}
struct Retriever {}
#[derive(Debug)]
struct Poodle {}
struct Dachshund {}

use std::marker::PhantomData;

#[derive(Debug)]
struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>,
}

impl Dog<Labrador> {
    fn barking(&self) {
        println!("Woof! Woof!");
    }
}

impl Dog<Retriever> {
    fn bark(&self) {
        println!("bovu bovu!");
    }
}
fn main() {
let my_poodle: Dog<Poodle> = Dog {
name: "Jeffrey".into(),
breed: PhantomData,
}; 

let my_laboror: Dog<Labrador> = Dog {
    name: "Jeffrey".into(),
    breed: PhantomData,
    }; 

    my_laboror.barking();


    println!("My dog's name is {:?}", my_poodle);
}