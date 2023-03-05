fn main() {
    let tup: (i32, bool, f32) = (-16, true, 17.8);

    // can destructure a tuple
    let (x, y, z) = tup;

    println!("the value of y is {y}");

    // or index with .
    let negative_sixteen = tup.0;

    println!("the value of tup.0 is {negative_sixteen}");
}
