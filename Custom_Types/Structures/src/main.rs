use core::fmt::Formatter;
use core::fmt::Display;
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// // A unit struct
// struct Unit;

// // A tuple struct
// struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: f32,
    bottom_right: f32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<T,E> {
        write!(f, "{} {}", 
                  self.top_left, 
                  self.bottom_right
                )
    }
}

fn main() {
    // // Create struct with field init shorthand
    // let name = String::from("Peter");
    // let age = 27;
    // let peter = Person { name, age };

    // // Print debug struct
    // println!("{:?}", peter);

    // // Instantiate a `Point`
    // let point: Point = Point { x: 10.3, y: 0.4 };

    // // Access the fields of the point
    // println!("point coordinates: ({}, {})", point.x, point.y);

    // // Make a new point by using struct update syntax to use the fields of our
    // // other one
    // let bottom_right = Point { x: 5.2, ..point };

    // // `bottom_right.y` will be the same as `point.y` because we used that field
    // // from `point`
    // println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // // Destructure the point using a `let` binding
    // let Point { x: left_edge, y: top_edge } = point;

    // let _rectangle = Rectangle {
    //     // struct instantiation is an expression too
    //     top_left: 10,
    //     bottom_right: 20,
    // };

    // // Instantiate a unit struct
    // let _unit = Unit;

    // // Instantiate a tuple struct
    // let pair = Pair(1, 0.1);

    // // Access the fields of a tuple struct
    // println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // // Destructure a tuple struct
    // let Pair(integer, decimal) = pair;

    // println!("pair contains {:?} and {:?}", integer, decimal);

    // rect_are(&_rectangle);

    let point: Point = Point { x: 10.0, y: 4.0 };
    let num = 10.0;
    let r = square(&point, &num);
    println!(" {}" ,r);
}

fn rect_are(rec: &Rectangle) {
    let vys = rec.top_left * rec.bottom_right;
    println!("Obsah je {}",vys);

} 

fn square(point: &Point, num: &f32) -> Rectangle {
    
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: point.x,
        bottom_right: point.y,
    };

    return _rectangle;
}