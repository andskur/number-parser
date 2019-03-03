// Program that accepts a numeric literal through stdin and writes out a human-readable english form of it
use std::io;

// Dictionary with english numbers names
const ONE: &'static [&'static str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const TEEN: &'static [&'static str] = &["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const TEN: &'static [&'static str] = &["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
const BIG: &'static [&'static str] = &["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

fn main() {
    // Infinity loop for accepting user integer string from stdin
    loop {
        // Accepting string from stdin
        println!("Input an Integer:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("incompatible input");
        // remove "new line" character
        input.truncate(input.len() -1);

        // Check if inout is empty
        if input.len() < 1 {
            println!("empty string");
            continue
        }

        // Convert string to int64
        let number: i64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you can enter only numbers between {} and {}", i64::min_value(), i64::max_value());
                continue;
            },
        };

        // convert given int to human-readable english
        let output = convert_numbers(number);

        // print getting value to stdout
        println!("{}", output);

    }
}

fn convert_numbers(mut number: i64) -> String {
    // String for output
    let mut output = String::new();

    // String for temporary strings
    let mut tmp: String;

    // Hardcode checking MinInt
    if number == i64::min_value() {
        output.push_str("negative nine quintillion, two hundred twenty three quadrillion, three hundred seventy two trillion, thirty six billion, eight hundred fifty four million, seven hundred seventy five thousand, eight hundred seven");
        return output;
    }

    // Hardcode checking Zero
    if number == 0 {
        output.push_str(ONE[0]);
        return output;
    }

    // Check if number is negative
    if number < 0 {
        // Push "negative" to the start of vector
        output.push_str("negative ");
        number *= -1;
    }

    // Dividing input number by three digits segments
    let mut segments = vec![];
    while number > 0 {
        segments.push(number % 1000);
        number = number / 1000;
    }
    segments.reverse();

    // Get english worlds from segments
    let mut length:i32 = (segments.len() - 1) as i32;
    for segment in segments {
        // Pass empty segment
        if segment == 0{
            length -= 1;
            continue;
        }

        // Get segment numbers
        let _hundreds = (segment / 100 % 10) as usize;
        let _tens = (segment / 10 % 10) as usize;
        let _units = (segment % 10) as usize;
        if _hundreds > 0 {
            tmp = format!("{} hundred ", ONE[_hundreds]);
            output.push_str(&tmp);
        }

        // If segment has only hundreds values - add BIG and pass over
        if _tens == 0 && _units == 0 {
            if BIG[length as usize] != "" {
                tmp = format!("{}, ", BIG[length as usize]);
                output.push_str(&tmp);
                length -= 1;
                continue
            }
        }

        // Check if 'tens' unit more than 20
        if _tens == 0 {
            tmp = format!("{} ", ONE[_units]);
            output.push_str(&tmp);
        } else if _tens == 1 {
            tmp = format!("{} ", TEEN[_units]);
            output.push_str(&tmp);
        } else {
            // Check if ten not like 20, 30, ..., 90
            if _units > 0 {
                tmp = format!("{} {} ", TEN[_tens], ONE[_units]);
                output.push_str(&tmp);
            } else {
                tmp = format!("{} ", TEN[_tens]);
                output.push_str(&tmp);
            }
        }

        // Add big if needed
        if BIG[length as usize] != "" {
            tmp = format!("{}, ", BIG[length as usize]);
            output.push_str(&tmp);
        }

        length -= 1;
    }

    return output;
}
