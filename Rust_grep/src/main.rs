use colored::Colorize;

use std::env::args;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 2 {
        println!("You did not specify the file name {}", "correctly".red().bold());
        return;
    }

    let word = &args[1];
    let n = args.len();

    for i in 2..n {
        
        let lines = read_lines(&args[i]); 
        for line in &lines {
            let lines_len = line.len();
            if lines_len<word.len() || !line.contains(word) {continue;}
            print!("{}: ",args[i].red());
            let parts: Vec<&str> = line.split(word).collect();
            let iterated_parts = parts.iter().enumerate();
            if parts[0].is_empty(){
                print!("{}",word.green());
            }
            for (i,&part) in iterated_parts{
                if i==0 && part.is_empty(){ continue;}
                if part.is_empty() {
                    if i==parts.len()-1{
                        continue;
                    }
                    print!("{}",word.green());
                    continue;
                }
                print!("{}",part);
                if i!=parts.len()-1{
                    print!("{}",word.green());
                }
               
            }
            
            println!();
            
        }
    }}
