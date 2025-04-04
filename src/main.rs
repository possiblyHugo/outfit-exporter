#![allow(warnings)]
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use std::{fs::OpenOptions, io::Write};
use text_io::read;
use url::Url;

enum ExitOption {
    NewOutfit,
    Exit,
}

enum UrlError {
    NotCatalogLink,
    LinkParsingError,
    NoItemId,
}

fn main() {
    println!("{}", "[Outfit Menu]".yellow().bold());
    let mut exit_value: ExitOption = ExitOption::NewOutfit;

    loop {
        let outfit_ids: Vec<String> = create_outfit();
        exit_value = exit_menu(outfit_ids.join(","));

        match exit_value {
            ExitOption::NewOutfit => continue,
            ExitOption::Exit => break,
        }
    }

    println!("{}", "EXITING".red().bold());
}

fn parse_link(link: &String) -> Result<String, UrlError> {
    if let Ok(target_url) = Url::parse(&link) {
        let url_segments = target_url
            .path_segments()
            .map(|c| c.collect::<Vec<_>>())
            .unwrap();

        for segment in url_segments {
            if segment.trim().parse::<i64>().is_ok() {
                return Ok(segment.to_string());
            }
        }
        Err(UrlError::NoItemId)
    } else {
        Err(UrlError::LinkParsingError)
    }
}

fn create_outfit() -> Vec<String> {
    let mut value: String = String::from("");
    let mut outfit_list: Vec<String> = Vec::new();
    println!("{}", "First input: ".blue());
    loop {
        value = read!();

        if value <= String::from("-1") {
            break;
        }

        if let Ok(catalog_id) = parse_link(&value) {
            outfit_list.push(catalog_id);
        } else if let Err(E) = parse_link(&value) {
            eprintln!("Error: {}", E);
        }
        println!("{}", "next input: ".blue());
    }
    return outfit_list;
}

fn write_file(outfit_list: &String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("outfits.txt")
        .expect("Cannot open file");

    file.write(outfit_list.as_bytes()).expect("Write failed");

    println!("{}", "Successfully wrote to outfits.txt".green());
}

fn exit_menu(outfit_list: String) -> ExitOption {
    let options = [
        "1. Create a new outfit",
        "2. Set To Clipboard",
        "3. Write to file",
        "4. Exit",
    ];
    let mut value: i32 = 0;

    loop {
        println!("{}", "[EXIT MENU]".red().bold());

        for option in options {
            println!("{}", option);
        }

        value = read!();

        if value == 1 {
            return ExitOption::NewOutfit;
        } else if value == 2 {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(outfit_list.clone());
            println!("Done");
        } else if value == 3 {
            write_file(&outfit_list);
        } else if value == 4 {
            return ExitOption::Exit;
        }
    }
}
