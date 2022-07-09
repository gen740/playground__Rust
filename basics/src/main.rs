#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(100);
    println!(
        "The rectangle 2 has width {}, and height {}",
        rect2.width, rect2.height
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
