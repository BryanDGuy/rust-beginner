// There are three types of structures in Rust
// The tuple structs, which are, basically, named tuples.
// The classic C structs.
// Unit structs, which are field-less, are useful for generics.

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rect;

    (bottom_right.x - top_left.x) * (top_left.y - bottom_right.y)
}

// Returns a Rectanlge with its lower left corner at `point` and a width
// and height equal to `length`
fn square(point: Point, length: f32) -> Rectangle {
    let Point { x: bottom_left_x, y: bottom_left_y } = point;

    let top_left = Point { x: bottom_left_x, y: bottom_left_y + length };
    let bottom_right = Point { x: bottom_left_x + length, y: bottom_left_y };

    Rectangle { top_left: top_left, bottom_right: bottom_right }
}

pub fn structures() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields
    // from 'point'
    let bottom_right = Point { x: 5.2, ..point};

    // `bottom_right.y` will be the same as `point.y` because we used
    // that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    println!("rectangle area {}", rect_area(_rectangle));

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let bottom_left = Point { x: 0.0, y: 0.0 };
    let length = 6.4;
    println!("rectangle is {:?}", square(bottom_left, length));
}