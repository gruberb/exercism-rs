pub struct Luhn {
    number: String,
}

impl Luhn {
    pub fn is_valid(self) -> bool {
        let mut digits = 0;
        let mut sum = 0;
        
        for (index, d) in self.number.chars().rev().filter(|c| c != &' ').enumerate() { 
            match d.to_digit(10) {
                Some(c) => {
                    digits += 1;
                    if index % 2 == 1 {
                        sum += if c * 2 > 9 { c * 2 - 9} else { c * 2};
                    } else { 
                        sum += c;
                    }
                },
                None => return false,
            }
        }

        digits > 1 && sum % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: std::string::ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            number: input.to_string(),
        }
    }
}
