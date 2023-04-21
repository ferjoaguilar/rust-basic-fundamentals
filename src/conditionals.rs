fn conditionals() {
    let mut age = String::new();
    println!("Type your age: ");
    std::io::stdin().read_line(&mut age).expect("Failed to read line");

    let age: i32 = age.trim().parse().expect("Failed to parse age");

    if age < 18 {
        println!("\x1b[31mYou are allowed to vote\x1b[0m");
    } else {
        println!("\x1b[32mYou are not allowed to vote\x1b[0m");
    }


    let mut pill = String::new();

    println!("Type your pill red or blue: ");
    std::io::stdin().read_line(&mut pill).expect("Failed to read line");

    let pill = pill.trim().to_lowercase();
    if pill == "blue" {
        println!("You are wake up in your bed and believe what you want to believe");
    }  else if pill == "red" {
        println!("You stay in wonderland and I show you how deep the rabbit hole goes");
    } else {
        println!("That's not an option Neo");
    }
}