// Functions

fn is_divisible_by(a: u32, b: u32) -> bool {
    if b == 0 { return false; }

    a % b == 0
}


fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) { println!("fizzbuzz"); }
    else if is_divisible_by(n, 5) { println!("buzz"); }
    else if is_divisible_by(n, 3) { println!("fizz"); }
    else { println!("{}", n); }
}


fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}


// ============================================================================

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}


impl Point {
    // Static methods
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}


#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}


impl Rectangle {
    // An instance method
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;

        self.p2.x += x;
        self.p2.y += y;
    }
}


struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}


// ============================================================================




fn main() {
    fizzbuzz_to(30);
    println!("");

    let rec = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("rec.perimeter: {}", rec.perimeter());
    println!("rec.area: {}", rec.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    
    square.translate(4.2, 4.2);
    println!("square.translate(4.2, 4.2): {:?}", square);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // ========================================================================
    
    println!("\nclosure");

    let closure_annotated = |i: i32| -> i32 { i + 42 };
    println!("closure_annottated: {}", closure_annotated(2));

    // ========================================================================
    
    println!("\nclosure: Capturing");

    let color = "pink";
    // Uses `&color`
    let print = || println!("`color`: {}", color);
    print();
    print();

    let mut count = 0;
    // Uses `&mut count`
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();

    let movable = Box::new(3);
    // Uses `T`
    let comsume = || {
        println!("`movable`: {:?}", movable);
        std::mem::drop(movable);
    };
    comsume();
    
    // ========================================================================
    
    println!("\nHigher Order Functions");

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all squared odd numbers under 1000");
    let upper = 1000;

    // Imperative side
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n*n;

        if n_squared >= upper { break; }
        else if is_odd(n_squared) { acc += n_squared; }
    }
    println!("imperative way: {}", acc);

    // Functional way
    let sum_of_squared_odd_numbers: u32 = 
        (0..).map(|n| n*n)
             .take_while(|&n_squared| n_squared < upper)
             .filter(|&n_squared| is_odd(n_squared))
             .fold(0, |acc, n_squared| acc + n_squared);
    println!("functional way: {}", sum_of_squared_odd_numbers);


}
