use std::io;
use std::io::Write;
use std::process;

//
// UNDER CONSTRUCTION
//

fn login() {

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Error getting Username");

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Error getting Password");

    println!("Your credentials are {}:{}", username.trim(), password.trim());

}

fn register() {
    
    print!("Enter name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Error getting Name");

    print!("Enter age: ");
    io::stdout().flush().unwrap();
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Error getting Age");
    let age = age.trim();
    let int_age = match age.parse::<i32>() {
        Ok(int_age) => int_age,
        Err(_e) => -1,
    };
    if int_age < 0 {
        println!("\nInvalid age entered!\n");
        process::exit(1);
    }
    
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Error getting Username");

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password1 = String::new();
    io::stdin()
        .read_line(&mut password1)
        .expect("Error getting Password");

    print!("Confirm password: ");
    io::stdout().flush().unwrap();
    let mut password2 = String::new();
    io::stdin()
        .read_line(&mut password2)
        .expect("Error getting Password");

    if password1 != password2 {
        println!("Passwords do not match!");
        process::exit(1);
    }
    else {
        println!("Creating account {}!", username.trim());
        let _new_user = vec![
            Users::Name(name),
            Users::Age(int_age),
            Users::Username(username),
            Users::Password(password1),
        ];
    }

}

enum Users {

    Name(String), //Name
    Age(i32), //Age
    Username(String), //Username
    Password(String), //Password

}

fn main() {

    println!("\nWelcome to Cariosus Corp!\n\nWould you like to:\n[1] Login\n[2] Register");
    print!("\nEnter option [1] or [2]: ");
    io::stdout().flush().unwrap();
    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Option error!");
    let option = option.trim();

    if option == "1" {
        login();
    } 
    else if option == "2" {
        register();
    } 
    else {
        println!("Invalid option!");
    }

}

