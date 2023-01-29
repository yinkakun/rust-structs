#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.area() > other_rectangle.area()
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 45,
    };

    println!("The rectangle: {:?}", rect1);
    println!("The area of this rectangle is {}", rect1.area());

    if rect1.width() {
        println!("This rectangle has a non zero width: {}", rect1.width);
    }

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
}
