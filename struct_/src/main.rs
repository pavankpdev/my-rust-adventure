// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

/*
    REFACTOR WITH TUPLES
    WHY?
    1. Group related params together
    2. But the caveat is that you lose the meaning of the params, and you have to remember the order/index

*/

// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


/*
    REFACTOR WITH STRUCTS
    WHY?
    1. Group related params together with appropriate names

*/
struct Rectangle {
    width: u32,
    height: u32,
}

// METHOD SYNTAX
impl Rectangle {
    // @params: accepts a reference to a Rectangle instance, because it doesn't need the ownership of the instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // associated function, no self argument passed
    // @params: accepts two u32 params
    // @return: returns a Rectangle instance
    // @purpose: to create an instance of Rectangle with a square shape
    fn square(size: u32) -> Rectangle {
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
        width: 40,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );
}

