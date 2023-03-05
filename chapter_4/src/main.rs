fn main() {
    
    let some_string = String::from("yours");
    
    let (s2, len) = calculate_length(some_string);

    println!("the length of {} is {}", s2, len); 

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}