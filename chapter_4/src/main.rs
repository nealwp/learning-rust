fn main() {
    
    let mut some_string = String::from("yours");
    
    let r1 = &some_string;
    let r2 = &some_string;
    let r3 = &mut some_string;

    println!("{}, {}, {}", r1, r2, r3);
}
