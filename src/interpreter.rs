use ascii::{AsciiChar, AsciiString};
use std::convert::TryInto;

use crate::config::BrainrsConfig;
use crate::lexer::{Lexer, Token};

// TODO Prompt for input if not provided in file

#[derive(Debug)]
pub struct Interpreter<'a> {
    lexer: Lexer,
    pointer: usize,
    tape: Vec<u8>,
    input: std::str::Chars<'a>,
    raw_in: String,
    output: AsciiString,
    config: BrainrsConfig, // TODO config: BrainrsConfig: fixed-tape, fixed-tape-size, tape-element-size (u8, u16, ...), utf8 or ascii, verbose, output/result print format (show bytes, show title, ...)
}

impl<'a> Interpreter<'a> {
    /// Creates a new Interpreter based on the passed input and with the provided config
    pub fn new(input: &'a str, config: BrainrsConfig) -> Interpreter {
        let split = input.split_once('!').unwrap_or((input, ""));
        Interpreter {
            lexer: Lexer::new(split.0),
            input: split.1.chars(),
            pointer: 0,
            tape: vec![0],
            output: AsciiString::new(),
            raw_in: String::new(),
            config, // TODO Input (prompt or file)
        }
    }

    pub fn prompt_input(&'a mut self) -> std::io::Result<()> {
        if self.lexer.tokens.contains(&Token::Unknown(',')) && self.input.as_str() == "" {
            std::io::stdin().read_line(&mut self.raw_in)?;
            self.input = self.raw_in.chars();
        };
        Ok(())
    }

    pub fn exec(mut self) -> Result<AsciiString, String> {
        self.loop_instr(0)?;
        Ok(self.output)
    }

    #[doc(hidden)]
    fn increment_pointer(&mut self) -> Result<(), String> {
        self.pointer += 1;
        if self.pointer >= self.tape.len() {
            self.tape.push(0);
        }
        Ok(())
    }

    #[doc(hidden)]
    fn decrease_pointer(&mut self) -> Result<(), String> {
        if self.pointer == 0 {
            return Err(String::from("Cannot run negative pointer"));
        }
        self.pointer -= 1;
        Ok(())
    }

    #[doc(hidden)]
    fn loop_instr(&mut self, start_index: usize) -> Result<usize, String> {
        let mut i = start_index;
        loop {
            let token = match self.lexer.tokens.get(i) {
                Some(tok) => tok,
                None => return Ok(i),
            };
            // println!(
            //   "Processing: {:?} (#{}) [{}, {:?}]",
            //   token, i, self.tape[self.pointer], self.tape
            // );
            match token {
                Token::Return => self.decrease_pointer()?,
                Token::Advance => self.increment_pointer()?,
                Token::Increase => {
                    if self.tape[self.pointer] + 1 == std::u8::MAX {
                        // TODO Make it loop back to 0 instead?
                        return Err(String::from("Exceeded maximum number storage"));
                    }
                    self.tape[self.pointer] += 1;
                }
                Token::Decrease => {
                    if self.tape[self.pointer] == 0 {
                        return Err(String::from("Cell value cannot go lower than 0"));
                    }
                    self.tape[self.pointer] -= 1;
                }
                Token::GetInput => {
                    self.tape[self.pointer] = self
                        .input
                        .next()
                        .ok_or(String::from("Reached end of provided input"))?
                        as u8
                }
                Token::PushOutput => {
                    self.output
                        .push(unsafe { AsciiChar::from_ascii_unchecked(self.tape[self.pointer]) });
                    // TODO Any way to make error handling here? Yes, ensure that the number that you are using is not past the maximum ASCII character in the crate
                }
                // TODO Add single print alternative (print only the current character)
                Token::Print => {
                    println!("{}", self.output);
                }
                Token::LoopOpen => i = self.loop_instr(i + 1)?,
                Token::LoopClose => {
                    if self.tape[self.pointer] == 0 {
                        return Ok(i);
                    } else {
                        i = start_index;
                        continue;
                    }
                }
                Token::Debug => {
                    println!(
                        "DEBUG: *{}={} [tape: {:?}]",
                        self.pointer, self.tape[self.pointer], self.tape
                    )
                }
                Token::Unknown(ch) => match ch {
                    '!' => {
                        return Err(format!(
                            "Unexpected input/output separator, only one allowed per script"
                        ))
                    }
                    _ => {}
                },
                _ => {
                    return Err(format!(
                        "Unexpected or unimplemented token was received: {:?}",
                        token
                    ))
                }
            }
            i += 1;
        }
    }
}
