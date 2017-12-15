#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area( rect: Rectangle ) -> f32 {
    let area = (rect.p1.x - rect.p2.x) * (rect.p1.y - rect.p2.y);
    if area < 0.0
    {
        return -area
    }
    area
}

fn square( point: Point, sidelen: f32 ) -> Rectangle {
    let point2 = Point{ x: point.x + sidelen, y: point.y + sidelen };
    Rectangle{ p1: Point{ x: point.x, y: point.y }, p2: point2 }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    
    // Print debug struct
    println!("{:?}", peter);
    
    
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Area of my rectangle is {}.", rect_area(_rectangle));

    let mypoint = Point { x: 1.2, y: 3.0 };
    let _square = square(mypoint, 1.0);
    println!("My square: {:?}", _square);
}
