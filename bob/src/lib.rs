pub fn reply(message: &str) -> &str {
    let m = String::from(message);
    
    if m.is_empty() || m.trim().is_empty() {
        return "Fine. Be that way!"
    }

    let contains_lowercase = m.find(|c: char| c.is_lowercase());
    let contains_alphabetic = m.find(|c: char| c.is_alphabetic());
    
    let all_uppercase = contains_lowercase == None;
    let last: String = m.chars().last().unwrap().to_string();
    let is_uppercase: bool = m.chars().last().unwrap().is_uppercase();
    
    if &*last == " " && m.trim().chars().last().unwrap().to_string() == "?" {
        return "Sure."
    }

    if contains_alphabetic == None && &*last == "?" {
        return "Sure."
    }
    
    if contains_alphabetic == None {
        return "Whatever."
    }

    if all_uppercase && &*last == "?" {
        return "Calm down, I know what I'm doing!"
    }
    
    if all_uppercase {
        return "Whoa, chill out!"
    }
    
    match &*last {
        "." => "Whatever.",
        "!" => "Whatever.",
        " " => "Whatever.",
        "?" => "Sure.",
        _ => "Fine. Be that way!"
    }
}
