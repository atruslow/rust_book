#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize
}

impl Rectangle {

    fn area(&self) -> usize {
        &self.width * &self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height > other.height && self.width > other.width
    }

    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

}



fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}