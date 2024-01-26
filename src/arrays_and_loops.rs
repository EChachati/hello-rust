fn main() {
    let mut names: Vec<String> = Vec::new();

    for _ in 0..3 {
        let mut name = String::new();
        println!("Write a name: ");
        std::io::stdin().read_line(&mut name).unwrap();
        names.push(name.trim().to_string());
    }

    for name in names.iter() {
        println!("Hello {}", name);
    }

    println!("1st Name: {}", names[0]);
    println!("2nd Name: {}", names.get(1).unwrap());

    println!("{:?}", names);
}
