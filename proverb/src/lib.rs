pub fn build_proverb(list: &[&str]) -> String {
    let mut result: Vec<String> = Vec::new();
    
    for (index, word) in list.iter().enumerate() {
        let mut sentence = String::from("");
        if list.len() - 1 == index {
            sentence.push_str(&format!("And all for the want of a {}.", list[0]));
        } else {
            sentence.push_str(&format!("For want of a {} the {} was lost.", list[index], list[index+1]));
        }

        result.push(sentence);
    }
    
    result.join("\n")
}
