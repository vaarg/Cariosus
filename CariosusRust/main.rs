use std::collections::HashMap;
use std::io;
use std::io::Write;

#[allow(dead_code)]

pub struct User {
    id: u64,
    username: String,
    password: String,
    administrator: bool,
}

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
        println!("Invalid login!");
    }
}

fn register(mut users: Users, id: u64) {
    
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String:: new();
    io::stdin()
        .read_line(&mut username)
        .expect("Username error.");
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
    // println!("Found {:?} entries", users.store.len());
    // assert_eq!(2, users.store.len())
    login(users);

}

fn main() {
    
    let id = 1;
    
    let mut users = Users::new();
    users.join(User {id: id, username: "Admin".to_string(), password: "Admin".to_string(), administrator: true});

    print!("\nWelcome to Cariosus Corp!\n\nWould you like to:\n[1] Login\n[2] Register\n\nEnter option [1] or [2]: ");
    io::stdout().flush().unwrap();
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Option error!");
    let option = option.trim();

    if option == "1" {
        login(users);
    } else if option == "2" {
        register(users, id);
        // login(users);
    } else {
        println!("Invalid option!");
    }

}
