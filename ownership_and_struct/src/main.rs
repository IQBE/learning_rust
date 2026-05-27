// Basic program to learn a bit about structs, ownership and some other concepts from earlier.
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io,
};

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    hashed_password: u64,
    age: u16,
}

impl User {
    fn is_older(&self, other_user: &User) -> bool {
        self.age > other_user.age
    }

    fn check_password(&self, given_password: String) -> bool {
        self.hashed_password == User::hash_password(&given_password)
    }

    fn hash_password(password: &str) -> u64 {
        let mut s = DefaultHasher::new();
        password.hash(&mut s);
        s.finish()
    }
}

fn ask_user_input(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        println!("{prompt}:");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let inp = input.trim().to_string();
                if !inp.is_empty() {
                    return inp;
                }
                println!("{prompt} can't be empty. Please try again.")
            }
            Err(_) => {
                println!("Something went wrong while reading the input... Please try again");
            }
        }
    }
}

fn main() {
    let default_user = User {
        username: String::from("default_user"),
        email: String::from("none@mail.com"),
        hashed_password: User::hash_password("NO_PASSWORD_HAS_BEEN_SET"),
        age: 0,
    };

    let user1 = User {
        username: String::from("IQBE"),
        email: String::from("git@quateau.net"),
        hashed_password: User::hash_password("password"),
        age: 24,
    };

    let user2 = User {
        username: String::from("Steve"),
        email: String::from("steve@mail.com"),
        ..default_user
    };

    println!("Is user1 older then user 2? {}", user1.is_older(&user2));

    println!("Creating new user! Please input the users information.");
    let new_user = User {
        username: ask_user_input("Username"),
        email: ask_user_input("email"),
        hashed_password: User::hash_password(&ask_user_input("password")),
        age: ask_user_input("age").parse().unwrap_or(0),
    };
    println!("New user created! \n{:#?}", &new_user);

    println!("Login for {}.", &user1.username);
    loop {
        if user1.check_password(ask_user_input("Password")) {
            println!("The password is correct! Welcome");
            break;
        } else {
            println!("The password is incorrect... Try again!")
        }
    }
}
