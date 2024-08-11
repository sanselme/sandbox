// SPDX-License-Identifier: GPL-3.0

mod util;

use crate::util::{pigify, Command};
use lib_util::number::{average, median, mode};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    println!("Average: {}", average(&numbers));
    println!("Median: {}", median(&mut numbers));
    println!("Mode: {}", mode(&numbers));

    let word = "apple";
    println!("{}", pigify(word));

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();
    println!("Type 'All' to list all employees by department");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'Quit' to quit");
    for line in stdin.lock().lines() {
        let input = line.expect("error: unable to read user input");
        match Command::from_input(&input) {
            Some(Command::Quit) => break,
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            }
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
                None => println!("I don not recognize that department"),
            },
            Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
            None => println!("Input error!"),
        }
    }

    println!("Have a nice day!");
}
