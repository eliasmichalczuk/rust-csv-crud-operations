use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, ErrorKind, Read};
use std::process;
use std::thread;
fn main() {
    println!("Welcome to local CRUD ");

    loop {
        println!("What are you going to do today?");
        println!("Listen registered entities: 1");
        println!("Register another entity: 2");
        println!("Insert data: 3");
        println!("Quit: 4");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Could not read input");

        if 1 == buffer.trim().parse::<i32>().unwrap() {
            registered_entities();
        }
        if 2 == buffer.trim().parse::<i32>().unwrap() {
            register_another_entity();
        }
        if 3 == buffer.trim().parse::<i32>().unwrap() {
            insert_data();
        }
        if 3 == buffer.trim().parse::<i32>().unwrap() {
            break;
        }
    }
}

fn registered_entities() {
    let entityname = 0;
    let path = String::from("/home/eliasmichalczuk/dev/personal/rust/local-crud/src/");
    let mut entitiesfile = entities_file();
    let mut contents = String::new();
    entitiesfile.read_to_string(&mut contents).expect("");

    for i in contents.lines() {
        let mut v: Vec<&str> = i.split(",").collect();
        println!("Entity: {}", v.remove(entityname));
        print!("Fields: {}\n", v.join(","));
    }

    // insert_data();
}

fn insert_data() {
    let mut err = 1;
    println!("What kind of entity is this data?");
    let mut entity_name = String::new();
    io::stdin()
        .read_line(&mut entity_name)
        .expect("Could not read input");
    let mut file = match get_file(entity_name.trim()) {
        Ok(file) => {
            err = 0;
            file
        }
        Err(err) => {
            panic!("Problem opening file: {:?}", err);
        }
    };
    while err == 1 {
        println!("What kind of entity is this data?");
        io::stdin()
            .read_line(&mut entity_name)
            .expect("Could not read input");
        file = match get_file(entity_name.trim()) {
            Ok(file) => {
                err = 0;
                file
            }
            Err(err) => {
                panic!("Problem opening file: {:?}", err);
            }
        };
    }
    loop {
        println!("Enter data separated by comma");
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Could not read input");
        file.write(data.as_bytes()).unwrap();
    }
}

fn register_another_entity() {
    let path = String::from("/home/eliasmichalczuk/dev/personal/rust/local-crud/src/");
    let mut entitiesfile = entities_file();

    println!("Enter entity name, then fields separed by comma");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input");
    entitiesfile
        .write_all(format!("{}{}", "\n", buffer.trim()).as_bytes())
        .unwrap();
    let mut name_and_fields: Vec<&str> = buffer.trim().split(",").collect();
    match File::open(path.clone() + name_and_fields[0] + ".csv") {
        Ok(_) => {
            println!(
                "\n there is already a entity with name: {} \n",
                name_and_fields[0]
            );
            return;
        }
        Err(_) => {}
    }
    let mut crated_file = match File::create(path.to_string() + name_and_fields.remove(0) + ".csv")
    {
        Ok(fc) => fc,
        Err(e) => panic!(
            "Tried to create file entities but there was a problem: {:?}",
            e
        ),
    };
    match crated_file.write_all(name_and_fields.join(",").as_bytes()) {
        Ok(fc) => fc,
        Err(e) => panic!("error writing to created file: {:?}", e),
    };
}

fn entities_file() -> File {
    let entities_filename = "entities.csv";
    let path = String::from("/home/eliasmichalczuk/dev/personal/rust/local-crud/src/");
    let entities_file_result = match OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .open(path.clone() + entities_filename)
    {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(path.to_string() + entities_filename) {
                Ok(fc) => fc,
                Err(e) => panic!(
                    "Tried to create file entities but there was a problem: {:?}",
                    e
                ),
            }
        }
        Err(err) => {
            panic!("Problem opening file: {:?}", err);
        }
    };
    entities_file_result
}

fn get_file(file_name: &str) -> Result<File, io::Error> {
    let path = String::from("/home/eliasmichalczuk/dev/personal/rust/local-crud/src/");
    OpenOptions::new()
        .append(true)
        .open(path.clone() + file_name + ".csv")
}

struct Entity {
    name: String,
    filename: String,
    fields: Fields,
}

struct Fields {
    name: String,
    value: String,
}
