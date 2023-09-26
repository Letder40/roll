use std::{
    path::PathBuf,
    fs::{create_dir, DirEntry},
    env::var, 
    ffi::OsStr, 
    io::Write,
};

use rand::{self, Rng};

use crossterm::{
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor}
};

use tabled::{
    builder::Builder,
    settings::{
        Style,
    },
};

fn print_in_color(text: &str, color: Color) {
    execute!(
        std::io::stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor,
    ).unwrap();
}

fn list_profiles(profiles:&mut Vec<DirEntry>) {
    let home_path = var("HOME").unwrap();
    let path_str = format!("{home_path}/.roll_profiles");
    let roll_path = PathBuf::from(&path_str);
    
    if !roll_path.exists() {
        create_dir(&path_str).unwrap();
    };

    let mut counter = 1;
    let mut builder = Builder::default();
    let headers = vec!["ID", "path"];
    builder.set_header(headers);

    for profile in roll_path.read_dir().expect(format!("error in reading childs of {path_str}").as_str() ) {
        if let Ok(profile) = profile {
            if profile.path().extension().is_none() {
                continue;
            }else if profile.path().extension().unwrap() == OsStr::new("profile") {
                let row = [counter.to_string(), profile.path().as_os_str().to_str().unwrap().to_owned()];
                profiles.push(profile);
                counter += 1; 
                builder.push_record(row);
            }
        }
    }

    if profiles.len() == 0 {
        println!("There are not profiles in {}", path_str);
        return; 
    }

    let table = builder.build()
    .with(Style::rounded()).to_string();

    println!("{table}");

}

fn get_profile(profiles:&mut Vec<DirEntry>) -> &DirEntry {
    print_in_color("\n[?] ", Color::Cyan);
    println!("select a profile: ");
    print_in_color("> ", Color::Green);

    std::io::stdout().flush().unwrap();

    let mut input_buf: String = String::new();
    std::io::stdin().read_line(&mut input_buf).unwrap();

    let input_int = input_buf.trim().parse::<usize>().expect("\n[!] Error: There is not a profile with that identifier\n");

    if input_int > profiles.len() || input_int == 0 {
        println!("\n[!] Error: There is not a profile with that identifier")
    }

    &profiles[input_int - 1]
}

fn get_random_thing(selected_profile: &DirEntry) {
    let content = std::fs::read_to_string(selected_profile.path()).unwrap();
    let content_words: Vec<&str> = content.split('\n').filter(|word| !word.is_empty() ).collect();
    let rand_result = content_words[rand::thread_rng().gen_range(0..content_words.len())];
    print_in_color("\nresult: ", Color::Green);
    println!("{}", rand_result)
}

fn main() {
    let mut profiles: Vec<DirEntry> = Vec::new();
    list_profiles(&mut profiles);
    let selected_profile = get_profile(&mut profiles);
    get_random_thing(selected_profile);
}
