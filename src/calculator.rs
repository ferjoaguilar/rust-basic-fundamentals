// Import libreries
use regex::Regex;

fn calculator(){
    // regular expressions
    let regex_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // Get user input
    let mut input = String::new();
    println!("Enter your expression:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    loop {
        // Apply operation
        let caps = regex_add.captures(input.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let num1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let addition = num1 + num2;

        input = input.replace(cap_expression, addition.to_string().as_str())
        
    }
    // Print result
    println!("Result: {}", input);
}