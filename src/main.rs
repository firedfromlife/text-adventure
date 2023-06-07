#![allow(dead_code)]
pub mod dialog;
use dialog::Dialog;
use std::{env, fs::File, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_file_path: &Path;
    if args.len() > 1 {
        json_file_path = Path::new(&args[1]);
    } else {
        json_file_path = Path::new("./data/dialog.json");
    }
    let file = File::open(json_file_path).expect("error opening file");
    let dialogs: Vec<Dialog> = serde_json::from_reader(file).expect("error reading file");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut index = dialogs[0].run();
    let mut should_quit = false;
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        index = dialogs.iter().find(|&d| d.id == index).unwrap().run();
        if should_quit {
            break;
        }
        if index == 999999 || index == 999991 {
            should_quit = true;
        }
    }
}
