// hello_rust

use std::fmt;

// This will create the implementation to make printable with `fmt::Debug`
#[derive(Debug)]
struct DebugPritable(i32);

#[derive(Debug)]
struct Deep(DebugPritable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8  
}

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// Implement `Display` for `Point3D`
impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn main() {
    // Line comment

    /*
     *  Block comment
     */
    
    /// Generate docs for the next item
    
    eprintln!("I'll showed in the io::stderc");

    // {} is the most basic placeholder
    println!("The answer to life the universe and everything: {}", 42);

    // Positional arguments
    println!("{0} and {1}. {1} e {0}.", "Goetia", "Macumba");
    
    // Named arguments
    println!("The {subject} is now {state} because {cause}.",
             subject="femboy demon snow leopard",
             state="demon ocelot",
             cause="they are not the same anymore");

    // :b -> binary, :x -> hexdecimal, :o -> octal
    println!("dec: {anwser}, hex: {anwser:x}, bin: {anwser:b}, octal: {anwser:o}", anwser=42);

    // Right-aling
    println!("{number:>width$}", number=42, width=8);
    println!("{number:>0width$}", number=42, width=8);

    
    println!("There is {0:?} {1:?}", 12, "Golden Knight");

    println!("Now {:?} will print.", DebugPritable(42));

    // The `derive` is a mystery box
    println!("The Deep struct now: {:?}.", Deep(DebugPritable(-72)));
    
    // The compiler definition of "pretty", it's sooooo JS
    let name = "KatoMono";
    let age = 19;
    let kato = Person{name, age};

    println!("{:#?}", kato);
    
    let point = Point3D { x: 3.3, y: -4.2, z: 3.14 };
    println!("Display vs Debug");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("\"Pretty Debug\": {:#?}", point);

}
