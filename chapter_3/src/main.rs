fn main() {
    let y = {
        let x = 3;
        x + 1 
    };

    println!("y is equal to {y}");

    let z = {
        let x = 3;
        x + 1;  // adding this semi-colon changes this from an expression to a statement.
    };

    println!("z is equal to {z}");
}
