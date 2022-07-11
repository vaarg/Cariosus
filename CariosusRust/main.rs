use std::collections::HashMap;
use std::io;
use std::io::Write;

#[allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: u64,
    username: String,
    password: String,
    administrator: bool,
}

#[derive(Clone)]
pub struct Users {
    store: HashMap<String, User>,
}

impl Users {
    pub fn new() -> Users {
        Users { store: HashMap::new() }
    }

    pub fn join(&mut self, user: User) {
        self.store.insert(user.username.clone(), user);
    }

    pub fn check_login(self, username: String, password: String) -> bool {
        if let Some(u) = self.store.get(username.as_str()) {
            return u.password == password;
        } else {
            false
        }
    }

    pub fn check_user_exist(self, username: String) -> bool {
        if let Some(u) = self.store.get(username.as_str()) {
            true
        } else {
            false
        }        
    }
}

fn login(users: Users) {
    
    println!("\nPlease login:\n");
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Option error!");
    let username = username.trim();
    
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Option error!");
    let password = password.trim();

    if users.check_login(username.to_string(), password.to_string()) {
        println!("Welcome {username}!");
        if username == "Admin" {
            println!("!FLAG!");
        }
    } else {
        println!("\nInvalid login!");
        main();
    }
}

fn notes (users: Users) {
    //placeholder
}

fn register(mut users: Users, id: u64) {
    
    let username: String = loop {
        print!("Enter username: ");
        io::stdout().flush().unwrap();
        let mut username = String:: new();
        io::stdin()
            .read_line(&mut username)
            .expect("Username error.");
        let username = username.trim();
        let users_test = users.clone();

        if users_test.check_user_exist(username.to_string()) {
            println!("Username is already taken!");
            continue;
        } else {
            let username = username.to_string();
            break username;
        }
    };
    let username = username.trim();
    
    let password = loop {
        print!("Enter password: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Password error.");
        
        print!("Confirm password: ");
        io::stdout().flush().unwrap();
        let mut password2 = String::new();
        io::stdin()
            .read_line(&mut password2)
            .expect("Password error.");

        if password.eq(&password2) {
            break password;
        } else {
            println!("Passwords don't match!");
            continue;
        }      
    };
    let password = password.trim();
    let id = id + 1;

    users.join(User {id: id, username: username.to_string(), password: password.to_string(), administrator: false});
    login(users);

}

fn main() {
    
    let mut users = Users::new();
    let id = 1;
    users.join(User {id: id, username: "Admin".to_string(), password: "Admin".to_string(), administrator: true});

    println!("\nWelcome to Cariosus Corp!");

    let option = loop {
        print!("\nWould you like to:\n  [1] Login\n  [2] Register\n\nEnter option [1] or [2]: ");
        io::stdout().flush().unwrap();
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Option error!");
        if option == "1\n" || option =="2\n" {
            break option;
        } else {
            println!("\nInvalid option!");
            continue;
        }
    };
    
    let option = option.trim();
    if option == "1" {
        login(users);
    } else if option == "2" {
        register(users, id);
    }

}
