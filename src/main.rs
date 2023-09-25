use std::{
    path::PathBuf,
    fs::{create_dir, DirEntry},
    env::var, ffi::OsStr, io::Write,
};
use rand::{self, Rng};

fn list_profiles(profiles:&mut Vec<DirEntry>) {
    let home_path = var("HOME").unwrap();
    let path_str = format!("{home_path}/.roll_profiles");
    let roll_path = PathBuf::from(&path_str);
    
    if !roll_path.exists() {
        create_dir(&path_str).unwrap();
    };

    let mut counter = 1;

    for profile in roll_path.read_dir().expect(format!("error in reading childs of {path_str}").as_str() ) {
        if let Ok(profile) = profile {
            if profile.path().extension().is_none() {
                continue;
            }else if profile.path().extension().unwrap() == OsStr::new("profile") {
                println!("{} -> {}", counter, profile.path().display());
                profiles.push(profile);
                counter += 1; 
            }
        }
    }

    if profiles.len() == 0 {
        println!("There are not profiles in {}", path_str);
        return; 
    }
}

fn main() {
    let mut profiles: Vec<DirEntry> = Vec::new();
    list_profiles(&mut profiles);

    println!("\nselect a profile: ");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input_buf: String = String::new();
    std::io::stdin().read_line(&mut input_buf).unwrap();

    let input_int = input_buf.trim().parse::<usize>().expect("\n[!] Error: There is not a profile with that identifier\n");

    if input_int > profiles.len() || input_int == 0 {
        println!("\n[!] Error: There is not a profile with that identifier")
    }

    let selected_profile = &profiles[input_int - 1];
    let content = std::fs::read_to_string(selected_profile.path()).unwrap();
    let content_words: Vec<&str> = content.split('\n').filter(|word| !word.is_empty() ).collect();
    println!("Charged profile -> {}\n", selected_profile.file_name().to_str().unwrap() );

    let rand_result = content_words[rand::thread_rng().gen_range(0..content_words.len())];
    println!("result: {}", rand_result)
}
