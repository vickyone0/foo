struct Dog;
struct Cat;

pub trait Discription{
    fn discribe() -> String;
}

impl Discription for Dog {
    fn discribe() -> String {
        String::from("I am a dog")
    }
}

impl Discription for Cat {
    fn discribe() -> String {
        String::from("I am a cat")
    }
}

fn drisction_type<T:Discription>() -> String
{
    T::discribe()
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    println!("{}", drisction_type::<Dog>());
    println!("{}", drisction_type::<Cat>());
    //println!("{}", drisction_type());
}