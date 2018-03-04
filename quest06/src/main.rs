// Conversion

#[derive(Debug)]
struct Number {
    value: i32,
}


impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}


impl ToString for Number {
    fn to_string(&self) -> String {
        format!("The number is: {}", self.value)
    }
}


fn main() {
    
    let num = Number::from(42);
    println!("num: {:?}", num);
    println!("{}", num.to_string());

    let i = 5;
    let num2: Number = i.into();
    println!("num2: {:?}", num2);
    println!("{}", num2.to_string());

    let parsed: i32 = "42".parse().unwrap();
    let turbo_parsed: i32 = "24".parse::<i32>().unwrap();
    
    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);

}
