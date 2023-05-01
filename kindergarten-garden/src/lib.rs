use std::ascii::AsciiExt;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let plants = ["clover", "grass", "radishes", "violets"];

    let mut res: Vec<&str> = Vec::default();

    let mut first_half = diagram.to_string();
    let second_half = first_half.split_off(diagram.len() / 2);
    let second_half = second_half.trim();
    println!("f: {} s:{}", first_half, second_half);
    let position = alphabet.find(student.chars().nth(0).unwrap()).unwrap() + 1;
    println!("{}", position);
    let letters = vec![
        first_half.chars().nth(position * 2 - 2).unwrap(),
        first_half.chars().nth(position * 2 - 1).unwrap(),
        second_half.chars().nth(position * 2 - 2).unwrap(),
        second_half.chars().nth(position * 2 - 1).unwrap(),
    ];

    println!("letters: {:#?}", letters);
    res.push(
        plants
            .into_iter()
            .find(|p| {
                letters[0].to_string().to_ascii_lowercase() == p.chars().nth(0).unwrap().to_string()
            })
            .unwrap(),
    );
    res.push(
        plants
            .iter()
            .find(|p| {
                letters[1].to_string().to_ascii_lowercase() == p.chars().nth(0).unwrap().to_string()
            })
            .unwrap(),
    );
    res.push(
        plants
            .iter()
            .find(|p| {
                letters[2].to_string().to_ascii_lowercase() == p.chars().nth(0).unwrap().to_string()
            })
            .unwrap(),
    );
    res.push(
        plants
            .iter()
            .find(|p| {
                letters[3].to_string().to_ascii_lowercase() == p.chars().nth(0).unwrap().to_string()
            })
            .unwrap(),
    );

    println!("{:#?}", res);

    res
}
