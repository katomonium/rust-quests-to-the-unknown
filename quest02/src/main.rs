// Primitives

use std::fmt;

// Tuples can be used as function argument
fn reverse(pair: (i32, f32)) -> (f32, i32) {
    // Bind the tuple members to variables
    let (int, float) = pair;

    (float, int)
}


struct Matrix2Sq(f32, f32, f32, f32);

impl fmt::Display for Matrix2Sq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}  {})\n({}  {})", self.0, self.1, self.2, self.3)
    }
}


fn transpose_2sq(matrix: Matrix2Sq) -> Matrix2Sq {
    Matrix2Sq(matrix.0, matrix.2, matrix.1, matrix.3)
}


fn analize_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has size {}", slice.len());
}


fn main() {
    // A long tuple
    let long_tuple = ("Natani", 42u8, "Flora", 3.14156, "Trace", false, "Keith");
    println!("My long tuple: {:?}", long_tuple);
    
    // Accessing members
    println!("First: {}", long_tuple.0);
    println!("May the Forth: {}", long_tuple.3); 

    // "A circle inside a circle"
    let circ_in_circ = (("Maul", "Tyranus", "Vader"), ("Star Killer",));
    println!("CiC: {:?}", circ_in_circ);

    let pair = (42, 3.14156);
    println!("The Right Path: {:?} \t\t The Left Path {:?}", pair, reverse(pair));

    // From tuples to variables
    let tuple = (1, "Noooooooooooo", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    let matrix = Matrix2Sq(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose_2sq(matrix));


    // Fixed-size array
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr1);

    // The same thing, but using some compiler tricks
    let arr2 = [1, 2, 3, 4, 5, 21];
    println!("{:?}", arr2);

    // Acessing the elements
    println!("arr1[0]: {}", arr1[0]);
    println!("arr2[1]: {}", arr2[0]);

    // Discover the lenth of an array
    println!("arr2 lenth: {}", arr2.len());

    println!("From a array to a slice");
    analize_slice(&arr2);

    println!("From a part of an array to a slice");
    analize_slice(&arr2[1 .. 4]);

}
