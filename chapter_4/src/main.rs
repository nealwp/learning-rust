fn main() {
    
    let mut some_string = String::from("yours");
    
    change(&mut some_string);
}

fn change(s: &mut String) {
    s.push_str(", just kidding");
}