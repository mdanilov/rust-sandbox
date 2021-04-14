#[derive(Debug)] // annotation to derive the `Debug` trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {:#?}", rect); // printing using the debug formatting

    println!("The area of the rectangle is {} square pixels", area(&rect));

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle is {} square pixels", rect.area());
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    Rectangle::square(10);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
