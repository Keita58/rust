use std::collections::HashMap;
use std::io;

fn canvi(text: &str) -> String {
    let alphabet = HashMap::from([
        ("0", "o"),
        ("1", "L"),
        ("2", "R"),
        ("3", "E"),
        ("4", "A"),
        ("5", "S"),
        ("6", "b"),
        ("7", "T"),
        ("8", "B"),
        ("9", "g"),
        ("a", "4"),
        ("b", "I3"),
        ("c", "["),
        ("d", ")"),
        ("e", "3"),
        ("f", "|="),
        ("g", "&"),
        ("h", "#"),
        ("i", "1"),
        ("j", ",_|"),
        ("k", ">|"),
        ("l", "1"),
        ("m", r"/\/\"),
        ("n", "^/"),
        ("o", "0"),
        ("p", "|*"),
        ("q", "(_,)"),
        ("r", "I2"),
        ("s", "5"),
        ("t", "7"),
        ("u", "(_)"),
        ("v", r"\/"),
        ("w", r"\/\/"),
        ("x", "><"),
        ("y", "j"),
        ("z", "2"),
    ]);

    let mut output = String::new();

    for i in text.to_lowercase().chars() {
        if let Some(value) = alphabet.get(i.to_string().as_str()) {
            output.push_str(value);
        }
        else {
            output.push(i);
        }
    }

    return output;
}

fn main() {
    let mut paraula = String::new();
    io::stdin()
        .read_line(&mut paraula)
        .expect("Failed to read line");

    let ret = canvi(&paraula);
    println!("{}", ret);
}
