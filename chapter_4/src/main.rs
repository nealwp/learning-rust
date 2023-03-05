fn main() {
    
    let mut some_string = String::from("yours");
    
    let r1 = &mut some_string;
    let r2 = &mut some_string;

    println!("{}, {}", r1, r2);
}
