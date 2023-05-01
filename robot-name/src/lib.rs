use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot { name: Self::generate_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        let mut name = String::new();

        loop {
            // Generate random name
            name = format!("{}{}{:03}", rng.gen_range(b'A'..=b'Z') as char,
                           rng.gen_range(b'A'..=b'Z') as char, rng.gen_range(0..=999));

            // Check if the name already exists
            if !ROBOT_NAMES.lock().unwrap().contains(&name) {
                break;
            }
        }

        // Add the name to the set of existing names
        ROBOT_NAMES.lock().unwrap().insert(name.clone());

        name
    }

    pub fn reset_name(&mut self) {
        ROBOT_NAMES.lock().unwrap().remove(&self.name);
        self.name = Self::generate_name();
    }
}

// A set of all existing robot names to prevent collisions
lazy_static! {
    static ref ROBOT_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}