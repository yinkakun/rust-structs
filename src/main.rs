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
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 50,
    };

    println!("The rectangle: {:?}", rect);
    println!("The area of this rectangle is {}", rect.area());

    if rect.width() {
        println!("This rectangle has a non zero width: {}", rect.width);
    }
}
