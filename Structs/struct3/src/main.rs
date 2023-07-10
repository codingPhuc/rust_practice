struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // we used self instead of rectangle &Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // the self is short for self: &self

    // we used self here for the same resond we dont used rectangle we dont want to take onwer ship
    //
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // the associated function is like a contructor you much pass in a value before an instand is created
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "the area of the rectangle is {} square pixer.",
        rect1.area()
    );
    // the argument with parentheses it know that we are using the method if not it will default to the field
    println!(
        " this is the method width  {} this is the field width {} ",
        rect1.width(),
        rect1.width
    );
    println!(
        " Rectangle 1 is larger than Rectangle 2 : {} ",
        rect1.can_hold(&rect2)
    );
    let square = Rectangle::square(59);
    println!(
        " this is a square height {} this is a square weight {} ",
        square.height, square.width
    );
}

/* Structs let you create custom types that are meaningful
for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.
*/
