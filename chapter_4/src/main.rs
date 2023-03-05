fn main() {
    
    let some_string = String::from("yours");
    
    change(&some_string);
}

fn change(s: &String) {
    s.push_str(", just kidding");
}