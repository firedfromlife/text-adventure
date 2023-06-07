use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dialog {
    pub id: u32,
    pub text: String,
    pub options: Vec<Option>,
}

impl Dialog {
    fn new(id: u32, text: String, options: Vec<Option>) -> Dialog {
        Dialog {
            id: id,
            text: text,
            options: options,
        }
    }

    pub fn run(&self) -> u32 {
        // println!("{},{}", self.id, self.text);
        println!("{}", self.text);
        let mut n = 0;
        for i in &self.options {
            println!("{}) {}", n.to_string().cyan(), i.text);
            n += 1;
        }
        let line = prompt_x();
        let line = line.trim();

        match line.parse::<u32>() {
            Ok(num) => {
                if num < self.options.len() as u32 {
                    return self.options[num as usize].next;
                }
            }
            Err(_) => {}
        }
        return self.id;
    }
}

#[derive(Deserialize, Debug)]
pub struct Option {
    pub text: String,
    pub next: u32,
}

impl Option {
    fn new(text: String, next: u32) -> Option {
        Option {
            text: text,
            next: next,
        }
    }
}

pub fn prompt(s: &str) -> String {
    println!("{}", s);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

pub fn prompt_x() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

// pub fn choice(query: &str, options: Vec<String>) -> u32 {
//     loop {
//         println!("{}", query);
//         let mut n = 0;
//         for i in &options {
//             println!("{}) {}", n, i);
//             n += 1;
//         }
//         let line = promptX();
//         let line = line.trim();

//         match line.parse::<u32>() {
//             Ok(num) => {
//                 if num < options.len() as u32 {
//                     return num;
//                 }
//             }
//             Err(_) => {}
//         }
//     }
// }
