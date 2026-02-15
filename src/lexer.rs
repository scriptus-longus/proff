
#[derive(Debug, PartialEq)]
pub enum Tokens {
  Lambda,
  Var(u32),
  Type(u32),
  Bopen,
  Bclose,
  Delim,
  Arrow,
  Dot
}

pub fn lex_text(mut text: String) -> Vec<Tokens> {
  let mut buf = vec![];

  while !text.is_empty() {
    let c = text.remove(0);

    match c {
      '(' => buf.push(Tokens::Bopen),
      ')' => buf.push(Tokens::Bclose),
      '.' => buf.push(Tokens::Dot),
      ':' => buf.push(Tokens::Delim),
      '\\' => buf.push(Tokens::Lambda),
      '-' => {
        if text.chars().nth(0).unwrap() == '>' {
          buf.remove(0);
          buf.push(Tokens::Arrow)
        }
      }
      _ => {
        if !c.is_whitespace() {
          if c.is_lowercase() {
            let var_num = ((c as u8) - b'a') as u32;
            buf.push(Tokens::Var(var_num))
          } else if c.is_uppercase() {
            let var_num = ((c as u8) - b'A') as u32;
            buf.push(Tokens::Type(var_num))
          }
        }
      }
    }
  }
  buf
}
