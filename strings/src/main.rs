mod read_lines;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines::readlines("/etc/hosts") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }

    let str = String::from("robcsi".to_string());
    // str.split_whitespace();
    println! ("{}", str);

    let slice = &str[0..4];
    println! ("{}", slice);

    let interval = [0, 1, 2, 3, 4, 5 ,6, 7];
    println! ("{}", interval.iter().sum::<i32>());

    let mut v: Vec<bool> = Vec::new();
    v.push(true);
    v.push(false);

    for val in &v {
        println!("{}", val);

        match v.get(1) {
            Some(val) => println!("{}", val),
            None => ()
        }
    }
}
