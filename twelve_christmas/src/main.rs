fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let gifts = [
        "a patridge in a pear tree",
        "2 turtle doves",
        "3 french hens",
        "4 calling birds",
        "5 golden rings",
        "6 geese a laying",
        "7 swans a swimming",
        "8 maids a milking",
        "9 ladies dancing",
        "10 lords a leaping",
        "11 pipers piping",
        "12 drummers drumming"
    ];

    let mut count = 0;

    for day in days {
        println!("On the {day} day of christmas my true love gave to me");
        let gift = gifts[count];
        println!("{gift}");
        count += 1;
    }
}
