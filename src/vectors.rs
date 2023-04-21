fn vectors() {
    let mut names: Vec<String> = Vec::new();
    

    for _i in 0..3 {
        let mut name:String = String::new();
        println!("Please introduce your name: ");
        std::io::stdin().read_line(&mut name).expect("Failed to read line name");
        names.push(name);
    }

    println!("Quantity: {}", names.len());

    for name in names{
        println!("Name: {}", name);
    }
}