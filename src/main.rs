extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    loop {
        let n: i8 = rand::thread_rng().gen_range(1, 4);
        println!("Paper, scissors, rock!");

        let played = strtochoice();

        if played != 4 {
            let choice = choicetostr(n);
            println!("{}", choice);
            if played == n {
                println!("Draw");
            } else if played - n % 3 == 1 {
                println!("Win");
            } else {
                println!("Lose");
            }
        } else {
            break;
        }
    }
}

fn choicetostr(n: i8) -> String {
    let choice;

    if n == 1 {
        choice = "Paper";
    } else if n == 2 {
        choice = "Scissors";
    } else {
        choice = "Rock";
    }
    return choice.to_string();
}

fn strtochoice() -> i8 {
    let n: i8;

    loop {
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Failed to read line");

        if { c.starts_with("q") || c.starts_with("Q") } {
            println!("Quit game");
            n = 4;
            break;
        } else if { c.starts_with("p") || c.starts_with("P") } {
            n = 1;
            break;
        } else if { c.starts_with("s") || c.starts_with("S") } {
            n = 2;
            break;
        } else if { c.starts_with("r") || c.starts_with("R") } {
            n = 3;
            break;
        } else {
            println!("Enter a playable action");
        }
    }
    return n;
}
