// Variable binding

fn main() {
    
    // The binding lives in the main function
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        
        // Shadowing
        let long_lived_binding = 3.141568_f32;
        println!("inner long: {}", long_lived_binding);
    }

    println!("outter long: {}", long_lived_binding);
    
    // Shadowing
    let long_lived_binding = 'a';
    println!("outter long: {}", long_lived_binding);

}
