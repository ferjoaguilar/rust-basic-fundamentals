fn cicles(){
    let number_one = 123;
    let number_two = 123;

    let add = number_one + number_two;

    loop {
        // show number_one + number_two
        println!("Please write add {} and {}:", number_one, number_two);
        let mut user_add = String::new();
        std::io::stdin().read_line(&mut user_add).expect("Failed to read line");
        
        let user_add: i32 = user_add.trim().parse().expect("Failed to parse user_add");
    
        if user_add == add {
            println!("\x1b[32mYou are right!\x1b[0m");
            break;
          
        } else {
            println!("\x1b[31mYou are wrong!\x1b[0m");
    
        }
    }
}