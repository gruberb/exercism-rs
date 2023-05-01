use rand::Rng;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: generate_robot_name(),
       }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = generate_robot_name();
    }
}

fn generate_robot_name() -> String {
    let mut name = String::new();

    name.push(get_random_letter());
    name.push(get_random_letter());
    name.push(get_random_number());
    name.push(get_random_number());
    name.push(get_random_number());

    name
}

fn get_random_letter() -> char {
    let mut rng = rand::thread_rng();

    let letters = ('A'..='Z').collect::<Vec<char>>();
    let a = rng.gen_range(0..26);

    *letters.get(a).unwrap()
}

fn get_random_number() -> char {
    let mut rng = rand::thread_rng();

    let numbers = ('0'..='9').collect::<Vec<char>>();
    let a = rng.gen_range(0..=9);

    *numbers.get(a).unwrap()
}