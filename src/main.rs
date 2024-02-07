use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    //The "standard" for brainfuck interpreters/compilers is 30000
    let mut mem_space: Vec<u8> = vec![0; 30000];
    let cleaned_code: String = lex(code);

    let mut ins_ptr: usize = 0;
    let mut data_ptr: u32 = 0;

    let locs: Vec<(usize, usize)> = find_loop_locs(&cleaned_code);

    while ins_ptr < cleaned_code.len() {
        let ins = cleaned_code.as_bytes()[ins_ptr as usize] as char;
        //println!("{}", data_ptr as usize);
        //mem_space[data_ptr as usize] -= 1 as u8

        let data_val = &mut mem_space[data_ptr as usize];

        match ins {
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '+' => *data_val = data_val.wrapping_add(1),
            '-' => *data_val = data_val.wrapping_sub(1),
            '.' => print!("{}", *data_val as char),
            ',' => println!("Not implemented"),
            '[' => {
                if *data_val == 0 {
                    ins_ptr = find_jump_loc(ins_ptr, &locs, true);
                }
            }

            ']' => {
                if *data_val != 0 {
                    ins_ptr = find_jump_loc(ins_ptr, &locs, false);
                }
            }

            _ => println!("Invalid command..."),
        }

        ins_ptr += 1;
    }
    println!("DONE!");
}

fn find_jump_loc(loc: usize, locs: &[(usize, usize)], forward: bool) -> usize {
    if !forward {
        for (open, close) in locs {
            if *close == loc {
                return *open;
            }
        }
    } else {
        for (open, close) in locs {
            if *open == loc {
                return *close;
            }
        }
    }

    0
}

fn find_loop_locs(code: &String) -> Vec<(usize, usize)> {
    let mut locs: Vec<(usize, usize)> = Vec::new();

    for (index, c) in code.char_indices() {
        if c == '[' {
            let loc: (usize, usize) = (index, find_close(code, index));
            locs.push(loc);
        }
    }

    return locs;
}

fn find_close(code: &String, start_index: usize) -> usize {
    let (_, search_in) = code.split_at(start_index + 1);
    let mut counter: i32 = 1;
    let mut close_pos: i32 = start_index as i32;

    let mut next_code = search_in.chars();

    while counter > 0 {
        let c = next_code
            .next()
            .expect(&format!("Matching ] not found for [ at {start_index}"));
        close_pos += 1;

        if c == '[' {
            counter += 1;
        } else if c == ']' {
            counter -= 1;
        }
    }

    close_pos.try_into().unwrap_or(0)
}

fn lex(code_string: &str) -> String {
    //Remove anything that isn't a command character, and return the new string
    let mut lexed: String = String::new();

    for c in code_string.chars() {
        match c {
            '+' => lexed += "+",
            '<' => lexed += "<",
            '>' => lexed += ">",
            '-' => lexed += "-",
            '.' => lexed += ".",
            ',' => lexed += ",",
            '[' => lexed += "[",
            ']' => lexed += "]",

            _ => lexed += "",
        }
    }

    return lexed;
}