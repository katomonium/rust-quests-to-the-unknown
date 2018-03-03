// Custom types
#![allow(dead_code)]

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// A struct inside a struct
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}


fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { p1, p2 } = rect;
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;
    
    return ((x1 - x2) * (y1 - y2)).abs();
}


fn square(point: Point, n: f32) -> Rectangle {
    let p2 = Point { x: point.x - n, y: point.y - n };

    return Rectangle { p1: point, p2 };
}

// ============================================================================

// An `enum` to classify a web event
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPressed(char),
    Paste(String),
    Click { x: f64, y: f64 }
}

enum Langs {
    Pascal,
    Ruby,
    JS,
    Cxx,
    Python,
    Rust
}

// Enun can have explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

// ============================================================================

fn is_big(n: i32) -> bool {
    n > THRESHOLD 
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPressed(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted '{}'.", s),
        WebEvent::Click { x, y } => {
            println!("clicked at (x={}, y={}).", x, y);
        }
    }
}


fn main() {
    let name = "KatoMono";
    let age = 19;
    let kato = Person { name, age };
    println!("{:?}", kato);
    
    // Creating a point
    let point = Point { x: 0.3, y: 0.4 };
    // Accessing the data
    println!("Point coordenates: ({}, {})", point.x, point.y);
    
    // Update syntax to create a new struct
    let new_point = Point { x: -4.2, ..point };
    println!("Second point: ({}, {})", new_point.x, new_point.y);
    
    // Destructure the strut
    let Point { x: my_x, y: my_y } = point;

    let _rect = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: new_point,
    };

    // Instantiate a tuple struct
    let _pair = Pair(1, 0.1);
    println!("Acessing the tuple struct: ({}, {})", _pair.0, _pair.1);

    // Instantiate a unit struct
    let _nil = Nil;
    
    let rect2 = Rectangle {
        p1: Point { x: 1.2, y: -3.4 },
        p2: Point { x: 2.2, y: -6.8 }
    };

    println!("rect_area(rect2) = {}", rect_area(rect2));
    
    let p3 = Point { x: 5f32, y: 5f32 };
    let rect3 = square(p3, 5f32);
    println!("square((5, 5), 5) = {:?}", rect3);

    // ========================================================================
    
    println!("");

    let pressed = WebEvent::KeyPressed('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 0.42, y: 4.2 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    use Color::*;
    use Langs::*;

    // `enum` can be casted to integers
    println!("Langs::Rust = {}", Rust as i32);
    println!("Color::Green = {:06x}", Green as i32);

    // ========================================================================
    
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    
    let n = 42;
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

}

