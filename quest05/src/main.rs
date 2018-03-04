// Types
#![allow(overflowing_literals)]
#![allow(non_camel_case_types)]

type NanoSecond = u64;
type Inch = u64;
type u64T = u64;

fn main() {
    
    // Explicit conversion
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    
    // Overflow
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("  -1 as a u8 is: {}", (-1i8) as u8);

    println!(" 128 as a u8 is: {}", 128 as u8);
    println!(" 128 as a i8 is: {}", 128 as i8);

    // ========================================================================
    println!("");

    // Suffixed literals
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    
    // ========================================================================
    println!("");

    let elem = 5u8;
    
    // The compiler doesn't know the type of `vec`
    let mut vec = Vec::new();
    
    // The compiler infers that `vec` is a vector of `u8`
    vec.push(elem);

    println!("{:?}", vec);
    
    // ========================================================================
    
    let ns: NanoSecond = 5 as u64T;
    let ic: Inch = 2 as u64T;

    println!("{} ns + {} inchs = {} units?", ns, ic, ns+ic);

}
