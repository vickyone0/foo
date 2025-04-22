struct  Rectangle {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

trait Shape{
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;

}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}
fn main() {
    let rect = Rectangle { width: 10, height: 5 };
    let square = Square { side: 4 };

    println!("Rectangle area: {}", rect.area());
    println!("Rectangle perimeter: {}", rect.perimeter());

    println!("Square area: {}", square.area());
    println!("Square perimeter: {}", square.perimeter());
}