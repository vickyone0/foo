struct Dog;
struct Cat;

trait Discription{
    fn discribe(&self) -> String;
}

impl Discription for Dog {
    fn discribe(&self) -> String {
        String::from("I am a dog")
    }
}

impl Discription for Cat {
    fn discribe(&self) -> String {
        String::from("I am a cat")
    }
}

fn drisction_type<T>(t: &T) -> String
where
    T: Discription,
{
    t.discribe()
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    println!("{}", drisction_type(&dog));
    println!("{}", drisction_type(&cat));
}