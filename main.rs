use std::io;
use std::fs;
use std::path::Path;

fn main() {
    loop {

        let mut usern = String::new();
        println!("Welcome to RustyTodo! Please enter your username:");
        let usern = input(usern);

        let mut userconf = String::new();
        println!("You have input {usern}. Is that the correct username? [y/n]");

        if userconf {
            break;
        } else {
            continue;
        }
    }

    isfile = checkfile(usern);
    if isfile {
        println!("isfile");
    } else {
        println!("Welcome, it seems you are a new user");
    }
}

fn input(var: String) -> String {
    let mut var = String::new();
    io::stdin().read_line(&mut var);
    return var
}

fn checkfile(var: String) -> bool {
    let isfile = assert_eq!(Path::new("/RustyToDo/users/{var}.txt").try_exists());
    if isfile {
        return true
    } else {
        Path::new("/RustyToDo/users/{var}.txt");
        return false
    }
}

fn boolcheck(var: String) -> bool {
    loop {
        let mut var = String::new();
        let yorn: bool = if input(var) == "y" {
            true
        } else if input(var) == "n" {
            false
        } else {
            println!("Sorry, incorrect input. Please try again:");
        }
    }
}
