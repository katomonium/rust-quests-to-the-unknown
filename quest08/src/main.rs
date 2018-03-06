// Flow Control

fn main() {
    
    println!("Basic `if`");

    let num = 42;

    if num < 0 {
        print!("num is negative");
    } else if num == 42 {
        print!("num is the Awnser");
    } else {
        print!("num is positive");
    }

    let num2 = 
        if num > 25 && num < 50 {
            println!(", and kinda big");
            num / 10            // It return a number
        } else {
            println!(", and kinda smoll");
            num * 10            // It return a number

        };

    println!("{} -> {}", num, num2);

    // ========================================================================
    
    println!("\nThe Ancian `loop`");

    let mut count = 0u32;
    
    println!("Let's count!");
    
    // Infinite loop
    loop {
        count += 1;

        if count%7 == 0 {
            println!("Seven");
            continue;
        }

        println!("{}", count);

        if count == 15 {
            println!("Ok, enough!");
            break;
        }
    }

    // ========================================================================
    
    println!("\nThe Ancian `loop` 2: Nested");

    #[allow(unreachable_code)]
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
    
    // ========================================================================
    
    println!("\nThe Ancian `loop` 3: Gimme the result");

    count = 0;

    let result = loop {
        count += 1;

        if count == 11 { break count*2; }
    };

    println!("result: {}", result);

    // ========================================================================
    
    println!("\nFizzBuzz: `while` editon");

    let mut n = 1;
    while n < 101 {
        if n%15 == 0 { println!("fizzbuzz"); }
        else if n%3 == 0 { println!("fizz"); }
        else if n%5 == 0 { println!("buzz"); }
        else { println!("{}", n); }

        n += 1;
    }

    // ========================================================================
    
    println!("\nFizzBuzz: `for in` edition");
    
    for n in 1..101 {
        if n%15 == 0 { println!("fizzbuzz"); }
        else if n%3 == 0 { println!("fizz"); }
        else if n%5 == 0 { println!("buzz"); }
        else { println!("{}", n); }
    }

    println!("");

    let langs = vec!["Pascal", "Ruby", "JS", "C++", "Python", "Rust"];
    for lang in langs.iter() {
        match lang {
            &"JS" => println!("This lang is . Watch out.assync"),
            _ => println!("Hello {}", lang),
        }
    }
    println!("{:?}\n", langs);
    
    // `langs` is consumed
    for lang in langs.into_iter() {
        match lang {
            "JS" => println!("This lang is . Watch out.assync"),
            _ => println!("Hello {}", lang),
        }
    }
    // println!("{:?}", langs);
    println!("");
    
    let mut langs = vec!["Pascal", "Ruby", "JS", "C++", "Python", "Rust"];
    for lang in langs.iter_mut() {
        match lang {
            &mut "JS" => println!("This lang is . Watch out.assync"),
            _ => println!("Hello {}", lang),
        }
    }
    println!("{:?}", langs);

    // ========================================================================
    
    println!("\n`match`");

    let number = 12;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teenager"),
        _ => println!("Ain't special"),
    }
    
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
    
    // ========================================================================
    
    println!("\n`match`: Destructin `tuple`");

    let point = (-2, 0);

    println!("Tell me about {:?}", point);
    match point { 
        (0, y) => println!("It's over the y axis with y = {}", y),
        (x, 0) => println!("It's over the x axis with x = {}", x),
        _      => println!("It's not special in anyway"),
    }

    // ========================================================================
    
    println!("\n`match`: Destructin `enum`");

    #[allow(dead_code)]
    enum Color {
        Blue,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
    }

    let c = Color::RGB(238, 255, 34);
    match c {
        Color::Blue => println!("It's blue"),
        Color::RGB(r, g, b) => println!("R: {}, B: {}, G: {}", r, g, b),
        Color::HSV(h, s, v) => println!("H: {}, S: {}, V: {}", h, s, v),
    }

    // ========================================================================
    
    println!("\n`match`: Destructin `pointer/ref`");

    let reference = &4;
    match reference {
        &val => println!("Got a value via destructing: {:?}", val),
    }
    
    // Is possible dereference before matching
    match *reference {
        val => println!("Got a value via destructing: {:?}", val),
    }
    
    // Using `ref` creates a reference to a value
    let ref _is_a_referenc = 3;

    let value = 5;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("I added 10. `mut_value`: {:?}", m);
        },
    }

    // ========================================================================
    
    println!("\n`match`: Destructin `struct`");

    struct Truc { x: (u32, u32), y: f32 };

    let t = Truc { x: (1, 2), y: 3.14158 };

    match t {
        Truc { x: (a, 2), y: b } => println!("a: {:?}, b: {:?}", a, b),
        _ => println!("Meh..."),
    }
    
    // ========================================================================
    
    println!("\n`match`: Guards");

    let pair = (2, -2);

    match pair {
        (x, y) if x == y    => println!("I don't see any diference"),
        (x, y) if x+y == 0  => println!("Kaboooon ...."),
        (x, _) if x%2 == 0  => println!("Someone likes 2's"),
        _                   => println!("You tried"),
    }

    // ========================================================================
    
    println!("\n`match`: Binding");

    fn age() -> u32 {
        17
    }

    match age() {
        0           => println!("Ok, a think something is wrong"),
        n @ 1...12  => println!("Well, go do something, you just {}", n),
        n @ 13...19 => println!("You can drink, just a little, and far from anyone, you still {}", n),
        _n          => println!("VODKA"),
    }

    // ========================================================================
    
    println!("\n`if let`");

    let number = Some(7);

    if let Some(i) = number {
        println!("Gotcha: {}", i);
    }

    let letter: Option<i32> = None;
    if let Some(i) = letter {
        println!("Gotcha: {}", i);
    } else {
        println!("Didn't match a number.");
    }

    let emoticon: Option<i32> = None;
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Gotcha: {}", i);
    } else if i_like_letters {
        println!("Didn't match a number.");
    } else {
        println!("I don't like letters");
    }

    // ========================================================================
    
    println!("\n`if let`: `enum`");
    
    enum Foo {
        Bar,
        Quaz(u32)
    }

    let a = Foo::Bar;
    if let Foo::Bar = a {
        println!("a is Foo::Bar");
    }

    let b = Foo::Quaz(33);
    if let Foo::Quaz(i) = b {
        println!("b is Foo::Quaz with {}", i);
    }

    // ========================================================================
    
    println!("\n`while let`");

    let mut opt = Some(0);
    while let Some(i) = opt {
        if i > 17 {
            println!("I can drink now");
            opt = None;

        } else if i == 11 {
            println!("The Demogorgon is near");
            opt = Some(i+2);

        } else {
            println!("`i` is `{:?}`. Try again.", i);
            opt = Some(i+1);
        }
    }

}
