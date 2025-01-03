use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Member {
    name: String,
    savings: f64,
}

#[derive(Debug)]
struct SavingsGroup {
    members: HashMap<String, Member>,
}

impl SavingsGroup {
    fn new() -> Self {
        SavingsGroup {
            members: HashMap::new(),
        }
    }

    fn add_member(&mut self, name: String) {
        if self.members.contains_key(&name) {
            println!("Member {} already exists!", name);
        } else {
            self.members.insert(
                name.clone(),
                Member {
                    name: name.clone(),
                    savings: 0.0,
                },
            );
            println!("Member {} added successfully!", name);
        }
    }

    fn record_savings(&mut self, name: String, amount: f64) {
        match self.members.get_mut(&name) {
            Some(member) => {
                member.savings += amount;
                println!("Recorded {} for {}. Total savings: {}", amount, name, member.savings);
            }
            None => println!("Member {} does not exist!", name),
        }
    }

    fn view_savings(&self, name: String) {
        match self.members.get(&name) {
            Some(member) => println!("{} has saved {} so far.", member.name, member.savings),
            None => println!("Member {} does not exist!", name),
        }
    }

    fn view_all_savings(&self) {
        if self.members.is_empty() {
            println!("No members in the group yet.");
        } else {
            println!("All group savings:");
            for member in self.members.values() {
                println!("{}: {}", member.name, member.savings);
            }
        }
    }
}

fn main() {
    let mut group = SavingsGroup::new();

    loop {
        println!("\nSmart Kibiina");
        println!("1. Add Member");
        println!("2. Record Savings");
        println!("3. View Member Savings");
        println!("4. View All Savings");
        println!("5. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().parse::<u32>();

        match choice {
            Ok(1) => {
                println!("Enter member name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read input");
                group.add_member(name.trim().to_string());
            }
            Ok(2) => {
                println!("Enter member name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read input");
                let name = name.trim().to_string();

                println!("Enter amount to save: ");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read input");
                let amount = amount.trim().parse::<f64>();

                match amount {
                    Ok(amount) => group.record_savings(name, amount),
                    Err(_) => println!("Invalid amount entered."),
                }
            }
            Ok(3) => {
                println!("Enter member name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read input");
                group.view_savings(name.trim().to_string());
            }
            Ok(4) => group.view_all_savings(),
            Ok(5) => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}