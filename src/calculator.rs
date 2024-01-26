use regex::Regex;

fn main() {
    println!("Calculator");

    // Regex
    let re_add = Regex::new(r"(\d+)\s?([\+\-])\s?(\d+)").unwrap();
    let re_multiplication = Regex::new(r"(\d+)\s?([/\*])\s?(\d+)").unwrap();

    // Bring User Data
    println!("Write your expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // Multiply and divide
    loop {
        let caps = re_multiplication.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let sign = caps.get(2).unwrap().as_str();

        if sign == "*" {
            let result = left_value * right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        } else if sign == "/" {
            let result = left_value / right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        }
    }

    // Add and subtract
    loop {
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let sign = caps.get(2).unwrap().as_str();

        if sign == "+" {
            let result = left_value + right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        } else if sign == "-" {
            let result = left_value - right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        }
    }

    // Show Result
    println!("Result: {}", expression);
}
