use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Contact {
    name: String,
    phone_number: String,
    email: String,
}

impl Contact {

    fn serialize(&self) -> String {
        format!("{},{},{}", self.name, self.phone_number, self.email)
    }

    fn deserialize(data: &str) -> Contact {
        let parts: Vec<&str> = data.split(',').collect();
        Contact {
            name: parts[0].to_string(),
            phone_number: parts[1].to_string(),
            email: parts[2].to_string(),
        }
    }
}

fn check_for_duplicates(contacts: &Vec<Contact>, new_contact: &Contact) -> bool {
    for contact in contacts {
        if contact.name == new_contact.name
            && contact.phone_number == new_contact.phone_number
            && contact.email == new_contact.email
        {
            return true;
        }
    }
    false
}

fn add_contact(contacts: &mut Vec<Contact>) {
    let mut new_contact = Contact {
        name: String::new(),
        phone_number: String::new(),
        email: String::new(),
    };
    
    print!("Enter name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_contact.name).unwrap();
    new_contact.name = new_contact.name.trim().to_string();

    print!("Enter phone number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_contact.phone_number).unwrap();
    new_contact.phone_number = new_contact.phone_number.trim().to_string();

    print!("Enter email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_contact.email).unwrap();
    new_contact.email = new_contact.email.trim().to_string().to_lowercase();

    if check_for_duplicates(contacts, &new_contact) {
        println!("The contact already exists!");
    }
    else {
        println!("Contact added");
        let filename = "contacts.csv";
        contacts.push(new_contact);
        save_contacts(&contacts, filename);
    }
}

fn display_contacts(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        println!("No contacts available.");
        return;
    }
    println!(" ");
    println!("Contacts List:");
    println!("-------------------");
    for (i, contact) in contacts.iter().enumerate() {
        println!("Contact {}:", i + 1);
        println!("Name: {}", contact.name);
        println!("Phone Number: {}", contact.phone_number);
        println!("Email: {}", contact.email);
        println!("-------------------");
    }
}

fn search_contact<'a>(contacts: &'a Vec<Contact>, name: &'a str) -> Option<&'a Contact> {
    if contacts.is_empty() {
        return None;
    }
    let mut low = 0;
    let mut high = contacts.len();

    while low < high {
        let mid = low + (high - low) / 2;
        let mid_name = &contacts[mid].name.to_lowercase();

        if mid_name == &name.to_lowercase() {
            return Some(&contacts[mid]);
        } else if mid_name < &name.to_lowercase() {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn find_contact_index(contacts: &Vec<Contact>, name: &str) -> Option<usize> {
    contacts.iter().position(|c| c.name == name)
}

fn edit_contact(contacts: &mut Vec<Contact>) {
    print!("Enter the name of the contact to edit: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    if let Some(index) = find_contact_index(contacts, name) {
        let contact = &mut contacts[index];
        println!("Editing contact: {}", contact.name);

        print!("Enter new phone number (leave blank to keep current): ");
        io::stdout().flush().unwrap();
        let mut new_phone_number = String::new();
        io::stdin().read_line(&mut new_phone_number).unwrap();
        let new_phone_number = new_phone_number.trim();
        if !new_phone_number.is_empty() {
            contact.phone_number = new_phone_number.to_string();
        }

        print!("Enter new email (leave blank to keep current): ");
        io::stdout().flush().unwrap();
        let mut new_email = String::new();
        io::stdin().read_line(&mut new_email).unwrap();
        let new_email = new_email.trim();
        if !new_email.is_empty() {
            contact.email = new_email.to_string();
        }

        println!("Contact updated.");
    } else {
        println!("Contact not found.");
    }
}

fn delete_contact(contacts: &mut Vec<Contact>) {
    print!("Enter the name of the contact to delete: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    if let Some(index) = find_contact_index(contacts, name) {
        contacts.remove(index);
        println!("Contact deleted.");
    } else {
        println!("Contact not found.");
    }
}

fn load_contacts(contacts: &mut Vec<Contact>, filename: &str) {
    let file = File::open(filename);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    contacts.push(Contact::deserialize(&line));
                }
            }
        }
        Err(_) => {
            eprintln!("Error opening file for reading.");
        }
    }
}

fn save_contacts(contacts: &Vec<Contact>, filename: &str) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename);
    match file {
        Ok(mut file) => {
            for contact in contacts {
                writeln!(file, "{}", contact.serialize()).unwrap();
            }
        }
        Err(e) => {
            eprintln!("Error opening file for writing: {}", e);
        }
    }
}

fn main() {
    let mut contacts = Vec::new();
    let filename = "contacts.csv";
    load_contacts(&mut contacts, filename);
    contacts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    loop {
        println!("Contact Management System");
        println!("1. Add Contact");
        println!("2. Display Contacts");
        println!("3. Search Contact");
        println!("4. Edit Contact");
        println!("5. Delete Contact");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => add_contact(&mut contacts),
            2 => display_contacts(&contacts),
            3 => {
               print!("Enter the name to search: ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();

            if let Some(contact) = search_contact(&contacts, name) {
                println!("Contact found:");
                println!("Name: {}", contact.name);
                println!("Phone Number: {}", contact.phone_number);
                println!("Email: {}", contact.email);
            } else {
                println!("The Contact does not exist");
                }
            }
            4 => edit_contact(&mut contacts),
            5 => delete_contact(&mut contacts),
            6 => {
                save_contacts(&contacts, filename);
                println!("Exiting and saving contacts...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
