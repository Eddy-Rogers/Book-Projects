use std::io;

fn main() {
    println!("Please input a temperature followed by a space and F or C to indicate the desired scale. Type 'Quit' to exit.");

    loop {
        let mut guess = String::new();

        io::stdin().
            read_line(&mut guess)
            .expect("Failed to read line");

        let mut split = guess.split_ascii_whitespace();

        // Parse Integer
        if guess.to_ascii_lowercase().eq("quit\r\n"){
            break;
        }

        let temps: i32 = match split.next().expect("Expected number").parse::<i32>() {
            Ok(temps) => temps,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        // Parse Char
        let scale:String = split.next().expect("Expected string").to_string();

        if scale.capacity() == 1 {
            convert(temps, scale.chars().nth(0).expect("Expected single character"));
        }
    }
}

// Takes a temperature, and a character indicating the scale to convert to.
fn convert(temp: i32, scale: char) {
    match scale.to_ascii_lowercase() {
        'f' => {
            let converted_temp = f64::from(temp) * f64::from(9) / f64::from(5) + 32f64;
            println!("{temp}°C is {:.2}°F", converted_temp);
        }
        'c' => {
            let converted_temp = (f64::from(temp) - 32f64) * (f64::from(5) / f64::from(9));println!("Input is: {}", f64::from(temp));
            println!("{temp}°F is {:.2}°C", converted_temp);
        }
        _ => {
            println!("Unrecognized scale: {}", scale);
        }
    }
}