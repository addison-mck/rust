use std::io;

fn main() {
    println!("Enter a temperature:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let mut chars = temperature.trim().chars();
    let unit = chars.next_back();

    let is_celsius = match unit {
        Some('f') => false,
        Some('F') => false,
        Some('c') => true,
        Some('C') => true,
        _ => panic!("Must put the unit after the number")
    };

    let number: f64 = match chars.as_str().trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number")
    };

    if is_celsius {
        println!("Converted temperature: {:.2}°F", number * 1.8 + 32.0);
    } else {
        println!("Converted temperature: {:.2}°C", (number - 32.0) / 1.8);
    }
}
