use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("please enter a string");

        let command_words: Vec<String> = command
            .trim_end()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        let first_command = &command_words[0];

        match first_command {
            _ if first_command == "add" => {
                add_entry(command_words[1].clone(), command_words[3].clone(), &mut map);
                ()
            }
            _ if first_command == "view" => {
                view_department(command_words[1].clone(), &mut map);
                ()
            }
            _ => (),
        }
    }
}

fn add_entry(name: String, department: String, map: &mut HashMap<String, Vec<String>>) {
    match map.entry(department) {
        Entry::Vacant(e) => {
            e.insert(vec![name]);
            println!("successfully added to vec");
            ()
        }
        Entry::Occupied(mut e) => {
            e.get_mut().push(name);
            ()
        }
    }
}

fn view_department(department: String, map: &mut HashMap<String, Vec<String>>) {
    println!("getting {department}");
    println!("{0:?}", map.get(&department));
}
