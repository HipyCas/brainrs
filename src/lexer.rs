// TODO Add character/instruction for printing tape (maybe #?)
// TODO Add character/instruction for printing pointer/position (maybe @(at, sounds awesome)?)
// ? Can you actually increase the value over u8? Isn't that made impossible by the check against std::u8::MAX before increasing it (remove the may in the comment for Increase token)

/// Identifier for each brainf**k instruction
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
  /// Advance pointer position. This corresponds to the `>` character.
  Advance,
  /// Reduce pointer position. This corresponds to the `<` character.
  Return,
  /// Increase value of data (inside tape) in pointer position. This corresponds to the `+` character.
  ///
  /// This can raise an error, as the value may be increased further than what u8 allows.
  Increase,
  /// Decrease value of data (inside tape) in pointer position. This corresponds to the `-` character.
  ///
  /// This can raise an error, as the value can be tried to be brought below 0.
  Decrease,
  /// Push an ascii character created from the value in the current position to the output. This corresponds to the `.` character.
  PushOutput,
  /// Get input from the input (only file input supported yet with `!` separator). This corresponds to the `,` character.
  GetInput,
  /// Print the output as it is in the moment the program reaches here. This corresponds to the `%` character.
  Print,
  /// Indicates the start of a loop. This corresponds to the `[` character.
  LoopOpen,
  /// Indicates the end of a loop. This corresponds to the `]` character.
  LoopClose,
  // IOSeparator,
  /// Print information about the program's state as `*<pointer_index>=<pointer_value> [tape = <tape>]`. This corresponds to the `#` character.
  Debug,
  /// Represents an unknown token/character and wraps the specific character
  Unknown(char),
}

#[derive(Debug)]
pub struct Lexer {
  //? Maybe Peekable instead?
  pub tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(chars: &str) -> Lexer {
    let mut v: Vec<Token> = vec![];
    for ch in chars.chars() {
      v.push(match ch {
        '>' => Token::Advance,
        '<' => Token::Return,
        '+' => Token::Increase,
        '-' => Token::Decrease,
        '.' => Token::PushOutput,
        ',' => Token::GetInput,
        '%' => Token::Print,
        '[' => Token::LoopOpen,
        ']' => Token::LoopClose,
        '#' => Token::Debug,
        // '!' => Token::IOSeparator),
        _ => Token::Unknown(ch),
      })
    }
    Lexer { tokens: v }
  }

  /*
  pub fn from_stdin() -> io::Result<Lexer<std::str::Chars<'static>>> {
    let mut string = String::new();

    io::stdin().read_to_string(&mut string)?;
    let iterator = string.chars();
    Ok(Lexer {
      input: clone(iterator),
    })
  }
  */
}

// TODO To implement as iterator, instead of having a function that returns the tokens, have the new method save the tokens into a field
/*
impl Iterator for Lexer<'_> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {

  }
}
*/
