pub fn brackets_are_balanced(string: &str) -> bool {
    let open: Vec<String> = vec!["(".to_string(), "[".to_string(), "{".to_string()];
    let close: Vec<String> = vec![")".to_string(), "]".to_string(), "}".to_string()];

    let mut stack: Vec<String> = Vec::new();

    for x in string.chars() {
        if close.contains(&x.to_string()) && stack.is_empty() {
            stack.push(x.to_string());
            break;
        }
        
        println!("{:?} contains {}", open, string);
        if open.contains(&x.to_string()) {
            stack.push(x.to_string());
            println!("current stack: {:?}", stack); 
        }

        if close.contains(&x.to_string()) {
            let el = stack.get(stack.len() - 1); 

            if el == Some(&"(".to_string()) && x.to_string() != ")".to_string() || 
                el == Some(&"[".to_string()) && x.to_string() != "]".to_string() ||
                el == Some(&"{".to_string()) && x.to_string() != "}".to_string() {
                    break;
            }

            stack.pop();

        }
    
    }

    stack.is_empty()

}
