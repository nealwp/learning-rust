fn main() {
    
    let some_string = String::from("yours");
    
    let len = calculate_length(&some_string);

    println!("the length of {} is {}", some_string, len); 

}

fn calculate_length(s: &String) -> usize {
    s.len()
}