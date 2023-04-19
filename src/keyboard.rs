fn keyboard(){
    let mut name:String = String::new();
    let mut age: String = String::new();
    let mut country: String = String::new();

    println!("Please introduce your name: ");
    std::io::stdin().read_line(&mut name).expect("Failed to read line name");
    name = name.trim().to_string();

    println!("Introduce your age: ");
    std::io::stdin().read_line(&mut age).expect("Failed to read line age");

    println!("Introduce your country: ");
    std::io::stdin().read_line(&mut country).expect("Failed to read line country");

    // convert string to int
    let age: i32 = age.trim().parse().expect("Failed to parse age");

    println!("Hello {}. Welcome from {}, your age is {}", name, country, age);
}