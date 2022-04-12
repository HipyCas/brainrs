use ascii::{AsciiChar, AsciiString};
use human_panic::setup_panic;
use pico_args::{Arguments, Error as ArgError};
use std::io;
use std::io::Read;
use std::path::PathBuf;

use brainrs::interpreter::Interpreter;

// TODO When printing debug messages for each character in a loop, highlight the character with some ansi codes. To do that, instead of returning a text, do iter.map(|index?| if index == iter [currecnt iteration] { "[32m{}", character })
// TODO Implement check to only save char if the number is below maximum allowed by ascii crate
// TODO As you can find in the reference http://brainfuck.org/brainfuck.html (implement all mentioned), ! is used to separate code from input. The idea is that you could also add a second ! to give the interpreter a desired output, which it will test against, being like a testing framework. This ends like: <code>!<input>!<expected_output_for_testing>
// TODO Make vector work with bigger numbers, not limited ti u8, but ensure that the number fits into a u8 before printing

type FlagKey<'a> = [&'a str; 2];

#[doc(hidden)]
fn main() {
    /*
    let mut cells: Vec<u8> = vec![0];
    let mut pos: usize = 0;
    let mut output = AsciiString::new();
    */

    setup_panic!();

    let mut args = pico_args::Arguments::from_env();

    let mut tape = vec![0];

    match args.values_from_str::<FlagKey, PathBuf>(["-i", "--include"]) {
        Ok(paths) => {
            for path in paths {
                tape = Interpreter::with_tape(
                    &std::fs::read_to_string(path).unwrap(),
                    #[cfg(feature = "toml-config")]
                    toml::from_str(
                        &std::fs::read_to_string(
                            std::env::current_dir().unwrap().join("brainrs.toml"),
                        )
                        .unwrap_or(String::new()),
                    )
                    .unwrap(),
                    tape,
                )
                .exec_tape()
                .unwrap()
                .1;
            }
        }
        Err(e) => panic!("{}", e),
    }

    let mut input = String::new();
    match args.value_from_str::<FlagKey, PathBuf>(["-f", "--file"]) {
        Ok(p) => input = std::fs::read_to_string(p).unwrap(),
        Err(e) => match e {
            ArgError::MissingOption(_) => {
                io::stdin()
                    .read_to_string(&mut input)
                    .expect("Failed to read standard input");
            }
            ArgError::OptionWithoutAValue(_) => {
                eprintln!("Missing value for -f/--file option");
                std::process::exit(1);
            }
            _ => panic!("{:?}", e),
        },
    }

    /*
    let mut iter = 0;
    let mut to_continue = 0;
    for ch in input.chars() {
        if to_continue > 0 {
            to_continue -= 1;
            iter += 1;
            continue;
        }
        match ch {
            '>' => increment_pointer(&mut pos, &mut cells, true)
                .expect("Error while incrementing pointer position"),
            '<' => decrement_pointer(&mut pos, true).unwrap(),
            '+' => {
                cells[pos] += 1;
                println!(
                    "(main) + > Incremented cell value in position {} from {} to {}",
                    pos,
                    cells[pos] - 1,
                    cells[pos]
                );
            }
            '-' => {
                println!(
                    "(main) - > Substracting 1 from value {} in position {}",
                    cells[pos], pos
                );
                cells[pos] -= 1
            }
            '.' => {
                println!(
                    "(main) . > Adding char {} ({}) to output ({})",
                    unsafe { AsciiChar::from_ascii_unchecked(cells[pos]) },
                    cells[pos],
                    output
                );
                output.push(unsafe { AsciiChar::from_ascii_unchecked(cells[pos]) })
            } //print!("{}", unsafe { AsciiChar::from_ascii_unchecked(cells[pos]) }),
            '[' => {
                // TODO Get conditional logic into here
                if cells[pos] > 0 {
                    println!(
                        "(main) [ > Value in position {} is greater than 0 ({}), diving into loop",
                        pos, cells[pos]
                    );
                    let res = start_loop(
                        &mut pos,
                        &mut cells,
                        &input[iter + 1..], /*&find_loop_text(&text[iter..]).expect(&format!(
                                                "({}) Loop not closed in position {}",
                                                text, iter
                                            )),*/
                    );
                    to_continue = res.1;
                    output.push_str(&res.0);
                } else {
                    println!("(main) [ > Loop not needed to be run");
                    //return (output, input.len());
                }
            } // ! NEED ITERATOR TO CHECK WHERE THE LOOP IS,
            ']' => panic!(
                "(main) Opening bracket not found for closing bracket in position {}",
                iter
            ),
            '%' => {
                println!(
                    "(main) % > Printing output: {} (length: {})",
                    output,
                    output.len()
                );
                print!("{}", output)
            }
            ' ' | '\n' | '\t' | '\r' | _ => {} //_ => panic!("Invalid character: {}", ch),
        }
        iter += 1; // TODO Make it better with iterators, maybe with a .map ?
        match ch {
            '>' | '<' | '+' | '-' | '[' | ']' | '.' | '%' => {
                //std::thread::sleep(std::time::Duration::from_millis(500))
            }
            _ => {}
        }
    }
    */
    let interpreter = Interpreter::with_tape(
        &input,
        #[cfg(feature = "toml-config")]
        toml::from_str(
            &std::fs::read_to_string(std::env::current_dir().unwrap().join("brainrs.toml"))
                .unwrap_or(String::new()),
        )
        .unwrap(),
        tape,
    );
    // ! WEIRDLY NOT WORKING, WHAT THE F HELL HAPPENS??? interpreter.prompt_input();
    let res = match interpreter.exec_tape() {
        Ok(res) => format!("{}", res.0),
        Err(e) => format!("ERROR: {}", e),
    };
    println!("Output: {} {:?}", res, res.as_bytes());
}
