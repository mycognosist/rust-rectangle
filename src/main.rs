#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    
    // Associated function use-case
    let sq = Rectangle::square(3);

    println!("sq is {:#?}", sq);

    // Print a Rectangle instance, notice the :? formatting
    // :? requires #[derive(Debug)], shown above

    println!("rect1 is {:#?}", rect1);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}

// impl is an implementation of a struct

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    // An example of an associated function:
    // a function that is associated with the struct but doesn't
    // take self as a parameter and therefore doesn't work with an
    // instance of the struct
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
