use std::io;

fn sum(x: f32, y: f32) -> f32 {
    return x + y;
}

fn subtract(x: f32, y: f32) -> f32 {
    return x - y;
}

fn multiply(x: f32, y: f32) -> f32 {
    return x * y;
}

fn divide(x: f32, y: f32) -> Result<f32, String> {
    if x != 0.0 || y != 0.0 {
        Ok(x / y)
    } else {
        Err(String::from("Division by Zero!!"))
    }
}

fn main() {
    loop {
        let mut input = String::new();
        let x: f32;
        let operation: char;
        let y: f32;
        loop {
            input.clear();
            println!("Enter the first number");
            let _ = io::stdin().read_line(&mut input).expect("Failed to read");
            match input.trim().parse::<f32>() {
                Ok(n) => {
                    x = n;
                    break;
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        loop {
            input.clear();
            println!("Enter the operation (+, -, x, /)");
            let _ = io::stdin().read_line(&mut input).expect("Failed to read");
            match input.trim().parse::<char>() {
                Ok(n) => {
                    operation = n;
                    break;
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        loop {
            input.clear();
            println!("Enter the second number");
            let _ = io::stdin().read_line(&mut input).expect("Failed to read");
            match input.trim().parse::<f32>() {
                Ok(n) => {
                    y = n;
                    break;
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        input.clear();
        match operation {
            '+' => {
                println!("Result: {}", sum(x.clone(), y.clone()));
            }
            '-' => {
                println!("Result: {}", subtract(x, y));
            }
            'x' => {
                println!("Result: {}", multiply(x, y));
            }
            '/' => match divide(x, y) {
                Ok(n) => println!("Result: {}", n),
                Err(e) => println!("Error: {}", e),
            },
            _ => {
                println!("Invalid operation");
            }
        }
        println!("Press enter to continue");
        let _ = io::stdin().read_line(&mut input);
    }
}
