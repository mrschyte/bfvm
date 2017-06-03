use std::io::{Read, Write};
use std::num::Wrapping;

struct VM {
    mem: std::vec::Vec<u8>,
    ptr: usize
}

#[derive(Debug)]
enum Op {
    Left,
    Right,
    Inc,
    Dec,
    Get,
    Put,
    JmpIfZero(usize),
    Jmp(usize)
}

fn eval(program: std::vec::Vec<Op>, machine: &mut VM) {
    let mut ip: usize = 0;

    while ip < program.len() {
        let op = program.get(ip).unwrap();
        // println!("ip:{} op:{:?}", ip, *op);

        match *op {
            Op::Left => machine.ptr = machine.ptr - 1,
            Op::Right => {
                machine.ptr = machine.ptr + 1;
                if machine.ptr >= machine.mem.len() {
                    machine.mem.push(0);
                }
            },
            Op::Inc => machine.mem[machine.ptr] = (Wrapping(machine.mem[machine.ptr]) + Wrapping(1)).0,
            Op::Dec => machine.mem[machine.ptr] = (Wrapping(machine.mem[machine.ptr]) - Wrapping(1)).0,
            Op::Get => { std::io::stdout().write(&[machine.mem[machine.ptr]]).unwrap(); () }
            Op::Put => machine.mem[machine.ptr] = {
                std::io::stdin().bytes().next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8)
                    .unwrap()
            },
            _ => ()
        }

        match *op {
            Op::JmpIfZero(target) => {
                if machine.mem[machine.ptr] == 0 {
                    ip = target
                } else {
                    ip = ip + 1
                }
            },
            Op::Jmp(target) => {
                ip = target
            },
            _ => ip = ip + 1
        }
    }
}

fn compile(input: String, pos: usize) -> std::vec::Vec<Op> {
    let mut res = Vec::new();

    while res.len() < input.len() {
        let ch = input.chars().nth(res.len()).unwrap();
        match ch {
            '<' => res.push(Op::Left),
            '>' => res.push(Op::Right),
            '+' => res.push(Op::Inc),
            '-' => res.push(Op::Dec),
            '.' => res.push(Op::Get),
            ',' => res.push(Op::Put),
            '[' => {
                // recursively compile loop block
                let abs = pos + res.len();
                let sub1 = compile(input[res.len() + 1..].to_string(), abs + 1);
                res.push(Op::JmpIfZero(abs + sub1.len() + 1));
                res.extend(sub1);
            },
            ']' => {
                res.push(Op::Jmp(pos - 1));
                return res;
            },
            _ => println!("Throwaway: {}", ch)
        }
    }
    return res;
}

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    let mut vm = VM {mem: vec![0], ptr: 0};
 
    for argument in &args[1..] {
        let program = compile(argument.to_string(), 0);
        eval(program, &mut vm);
    }
}
