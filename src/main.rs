fn main() {
    let age: u8 = 25;
    let name: &str = "John";
    println!("I am {} and I am {} years old", name, age);

    get_data_from_user()
}

fn get_data_from_user() {
    /*
    The difference between &str and String is
    that &str is an immutable reference to a string somewhere else, while String is a growable,
    mutable, owned UTF-8 encoded string.
     */
    println!("Enter your name: ");
    // `let mut name = String::new();` is creating a new mutable variable `name` of type `String` and
    // initializing it with an empty string. This variable can be modified later in the code.
    let mut name = String::new();
    let mut age = String::new();
    // `std::io::stdin().read_line(&mut name).unwrap()` is reading input from the user and storing it
    // in the `name` variable.
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Enter your age: ");
    std::io::stdin().read_line(&mut age).unwrap();

    // The line `let age_as_number: u8 =
    // age.trim().parse().unwrap();` is converting the
    // user input stored in the `age` variable from a
    // string to an unsigned 8-bit integer (`u8`).
    let age_as_number: u8 = age.trim().parse().unwrap();

    print!(
        "Hello {}. You are {} years old",
        name.trim().to_string(),
        age_as_number
    );
}
