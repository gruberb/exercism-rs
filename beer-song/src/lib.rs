trait Amount {
    fn zero(&self) -> String;
    fn one(&self) -> String;
    fn two(&self) -> String;
    fn more(&self) -> String;
}

impl Amount for i32 {
    fn zero(&self) -> String {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    }

    fn one(&self) -> String {
        String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    }

    fn two(&self) -> String {
        String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n") 
    }

    fn more(&self) -> String {
        String::from(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", self, self, self - 1)) 
    }
}


pub fn verse(n: i32) -> String {
    match n {
        0 => n.zero(),
        1 => n.one(),
        2 => n.two(),
        _ => n.more()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result = String::from("");

    for x in (end..=start).rev() {
        result.push_str(&format!("{}\n", verse(x)));
    }

    format!("{}\n",result.trim().to_string())
}

