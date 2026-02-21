use crate::lexer;

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
  Empty,
  Unit,
  Arrow(Box<Type>, Box<Type>)
}

#[derive(Clone)]
pub enum Term {
  Var(u32),
  Lam(u32, Box<Term>, Box<Type>),
  Appl(Box<Term>, Box<Term>),
}


/* Grammar

Term ::= Lambda | Application

Lambda ::= '\' Var ':' Type '.' Term

Type ::= Atomic_type
        | Atomic_type '->' Type

Atomic_type ::= type | '(' type ')'

Application ::= Atom Atom | Atom


Atom ::= Var | '(', Term ')'
*/


pub fn parse_term(buf: &mut Vec<lexer::Tokens>) -> Term {
  let c = buf.last();

  match c {
    Some(lexer::Tokens::Lambda) => parse_lambda(buf),
    None => panic!("Error: Term could not be parsed"),
    Some(_) => parse_application(buf),
  }
}

pub fn parse_lambda(buf: &mut Vec<lexer::Tokens>) -> Term {
  if Some(lexer::Tokens::Lambda) != buf.pop() {
    panic!("Expeced \\ Lambda");
  } 

  let var = match buf.pop() {
    Some(lexer::Tokens::Var(x)) => x,
    _ => panic!("Expected Variable after lambda beginning"),
  };

  if Some(lexer::Tokens::Delim) != buf.pop() {
    panic!("Not type signature for lambda input found"); 
  }

  let in_type = parse_type(buf);

  if Some(lexer::Tokens::Dot) != buf.pop() {
    panic!("Expected . Delim");
  }

  let body = parse_term(buf);

  Term::Lam(var, Box::new(body), Box::new(in_type))
}

pub fn parse_type(buf: &mut Vec<lexer::Tokens>) -> Type {
  let mut t = parse_atomic_type(buf);

  while matches!(buf.last(), Some(lexer::Tokens::Arrow) | Some(lexer::Tokens::Bopen)) {
    buf.pop();
    println!("LAST TT {:?}", buf.last());
    let to = parse_atomic_type(buf);
    t = Type::Arrow(Box::new(t), Box::new(to))
  }

  t
}

pub fn parse_atomic_type(buf: &mut Vec<lexer::Tokens>) -> Type {
  match buf.pop() {
    Some(lexer::Tokens::Type(x)) => {
      if x == 0 {
        Type::Empty 
      } else if x == 1 {
        Type::Unit
      } else {
        panic!("Could not find type");
      }
    }
    Some(lexer::Tokens::Bopen) => {
      buf.remove(0);
      parse_type(buf)
    }
    _ => panic!("Expected type"),
  }
}


pub fn parse_application(buf: &mut Vec<lexer::Tokens>) -> Term {
  let mut term = parse_atom(buf);

  while matches!(buf.last(), Some(lexer::Tokens::Var(_)) | Some(lexer::Tokens::Bopen)) {
    let arg = parse_atom(buf);
    term = Term::Appl(Box::new(term), Box::new(arg));
  }

  term
}

pub fn parse_atom(buf: &mut Vec<lexer::Tokens>) -> Term {
  match buf.pop() {
    Some(lexer::Tokens::Var(x)) => Term::Var(x),
    Some(lexer::Tokens::Bopen) => {
      let term = parse_term(buf);
      buf.pop();
      term
    },
    _ => panic!("Unexpected Atom"),
  }
}
