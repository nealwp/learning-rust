fn main() {
    let s1 = String::from("hello world");
    
    takes_ownership(s1);

    println!("{}", s1); // this wont work


    let x = 5;
    makes_copy(x);
    println!("{}", x); // but this still does
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
