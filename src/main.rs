use std::fs::File;
use std::io;
use std::io::prelude::*;

mod tape;

fn main() {
    let bf_file = std::env::args().nth(1);
    match bf_file {
        Some(filename) => {
            match interpret(&filename) {
                Ok(()) => (),
                Err(err) => println!("Error: {:?}", err),
            }
        },
        None => println!("Usage: mindcrush [filename.bf]"),
    };
}

fn interpret(filename: &String) -> io::Result<()> {
    let mut f = File::open(filename)?;
    let mut code = String::new();
    f.read_to_string(&mut code)?;
    let program = code.chars()
        .filter(|c| "+-,.[]<>".contains(*c))
        .collect();
    parse(&program)?;
    Ok(())
}

fn parse(code: &Vec<char>) -> io::Result<()> {
    let mut tape = tape::Tape::new();
    let mut code_head = 0usize;
    let mut branch_stack: Vec<usize> = Vec::new();
    
    while code_head < code.len() {
        match code[code_head] {
            '+' => tape.inc(),
            '-' => tape.dec(),
            '>' => tape.fwd(),
            '<' => tape.bwd(),
            '.' => {
                if let Some(c) = std::char::from_u32(tape.get()) {
                    print!("{}", c);
                }
                io::stdout().flush()?;
            }
            ',' => {
                let mut input = [0u8];
                io::stdin().read_exact(&mut input)?;
                tape.put(input[0] as u32);
            }
            '[' => {
                branch_stack.push(code_head);
                if tape.get() == 0 {
                    while code[code_head] != ']' {
                        code_head += 1;
                    }
                }
            }
            ']' =>
                if tape.get() != 0 {
                    code_head = *branch_stack.last().expect("Mismatched ]");
                } else {
                    branch_stack.pop().expect("Mismatched [");
                },
            _ => ()
        };
        code_head += 1;
    }
    Ok(())
}
