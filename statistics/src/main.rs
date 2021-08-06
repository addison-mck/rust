use std::io;
use std::collections::HashMap;

struct Statistics {
    list: Vec<i32>,
}

impl Statistics {
    fn mean(&self) -> f64 {
        let mut sum = 0.0;
        let mut count = 0.0;
        for val in &self.list {
            sum += *val as f64;
            count += 1.0;
        };

        if count == 0.0 {
            0.0
        } else {
            sum / count
        }
    }

    fn median(&self) -> f64 {
        let mut sorted_list = self.list.clone();
        sorted_list.sort_unstable();
        let count = sorted_list.len();
        if count == 0 {
            return 0.0
        }
        let middle = count / 2;
        if count % 2 == 0 && count > 0 {
            let sum = sorted_list[middle - 1] + sorted_list[middle];
            sum as f64 / 2.0
        } else {
            sorted_list[middle] as f64
        }
    }

    fn mode(&self) -> Vec<i32> {
        let mut values = HashMap::new();
        let mut modes = Vec::new();
        for &val in &self.list {
            let count = values.entry(val).or_insert(0);
            *count += 1;
        };
        let mut maximum = i32::MIN;
        for (_, &count) in &values {
            if count > maximum {
                maximum = count;
            }
        }
        for (&key, &count) in &values {
            if count == maximum {
                modes.push(key)
            }
        }
        modes
    }
}

fn main() {
    println!("Enter a space separated list of numbers.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input_iter = input.split_whitespace();
    let mut input_vec = Vec::new();

    for number_str in input_iter {
        let number: i32 = match number_str.parse() {
            Ok(num) => num,
            Err(_) => panic!("Not a number")
        };
        input_vec.push(number);
    }

    let stats = Statistics {
        list: input_vec
    };

    println!("Mean: {}, Median: {}, Modes: {:?}", stats.mean(), stats.median(), stats.mode());
}
