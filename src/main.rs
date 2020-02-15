extern crate clap;

use std::io;

use cmd_args::Arguments;

mod cmd_args;

// TODO: 인자 받기 -t -s (탭, 스페이스)

fn main() {
    let parsed_args = Arguments::from_cmdline();
    if parsed_args.verbose {
        dbg!(&parsed_args);
    }

    for line in read_lines(&parsed_args) {
        let res = split_and_get(&line, &parsed_args);
        println!("{}", res.unwrap_or(""));
    }
}

fn split_and_get<'a>(line: &'a str, args: &'a Arguments) -> Option<&'a str> {
    let mut ret = None;
    let mut iter = line.split_ascii_whitespace();
    //    XXX: 왜 +1?
    for _i in 0..(args.col_idx + 1) {
        ret = iter.next();
    }
    ret
}

fn read_lines(args: &Arguments) -> Vec<String> {
    let stdin = io::stdin();
    let mut ret: Vec<String> = vec![];
    let mut n = 0;
    while 1 == 1 {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(len) if len > 0 => {
                // XXX: 여기 개선
                if args.ignore_head_line <= n {
                    ret.push(buffer);
                } else {
                    n += 1;
                }
            }
            _ => break,
        }
    }
    ret
}
