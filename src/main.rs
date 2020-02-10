use std::io;

fn main() {
    let lines = readlines();
    let line = &lines[0];
    let res = split_and_get(line, 4);
    dbg!(res);

//    let lines = readlines().unwrap();
//    dbg!(lines);
//    println!("Hello, world!");
}

fn split_and_get(line: &str, index: u8) -> Option<&str> {
    let mut ret: Option<&str> = None;
    let mut iter = line.split_ascii_whitespace();
    for _i in 0..index {
        ret = iter.next();
    }
    ret
}

fn readlines() -> Vec<String> {
    let stdin = io::stdin();
    let mut ret: Vec<String> = vec![];
    while 1 == 1 {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(len) if len > 0 => { ret.push(buffer) }
            _ => break,
        }
    }
    ret
}
