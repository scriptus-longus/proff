
mod lexer;
mod parser;

static VAR_NAMES: [char; 21] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u'];

use parser::Term;

/*#[derive(Clone)]
enum Term {
  Var(u32),
  Lam(u32, Box<Term>),
  Appl(Box<Term>, Box<Term>),
}*/

fn term_to_str(t: &Term, depth: u32) -> String{
  let mut ret = String::new();

  match t {
    Term::Lam(v, x) => {
      let var = VAR_NAMES[*v as usize]; 
      ret.push_str(&format!("\\{}.", var));
      ret.push_str(&term_to_str(x, depth+1));
      ret
    },
    Term::Appl(x, y) => {
      ret.push_str(&term_to_str(x, depth));
      ret.push_str(" ");
      ret.push_str(&term_to_str(y, depth));
      ret
    },
    Term::Var(x) => {
      ret.push(VAR_NAMES[*x as usize]);
      ret
    },
  }
}

fn check_valid_expr(t: &Term, depth: u32) -> bool {
  match t {
    Term::Lam(_, x) => {
      check_valid_expr(x, depth+1)
    },
    Term::Appl(x, y) => {
      check_valid_expr(x, depth) && check_valid_expr(y, depth)
    },
    Term::Var(_) => {
      true
    },
  }
}

fn subs_vars (f: &mut Term, x: u32, y: &Term) {
  // Term{ perform A B [x := y]
  match f {
    Term::Var(t) => {
      if *t == x {
        *f = y.clone();
      }     
    },
    Term::Lam(_, s) => {
      subs_vars(s, x, y)
    }
    Term::Appl(r, l) => {
      subs_vars(r, x, y);
      subs_vars(l, x, y);
    }
  }
}

fn beta_reduce_once(t: &mut Term, depth: u32) -> bool {
  match t {
    Term::Appl(k, y) => {
      if let Term::Lam(v, body) = &mut **k {
        let mut body = (**body).clone();
        let arg = y.clone();

        subs_vars(&mut body, *v, &arg);
        *t = body;
        true
      } else {
        beta_reduce_once(k, depth+1) || beta_reduce_once(y, depth+1)
      }
    },
    Term::Var(_) => {
      false
    },
    Term::Lam(_, x) =>  {
      beta_reduce_once(x, depth+1)
    }
  }
}

fn main() {
  let mut ast = Term::Appl(
                  Box::new(Term::Lam(
                    0,
                    Box::new(Term::Appl(
                      Box::new(Term::Var(0)),
                      Box::new(Term::Var(3))
                      ))
                    )),

                  Box::new(Term::Var(1))
                );

 
  let mut lx = lexer::lex_text(String::from("(\\a . a) b"));
  lx.reverse();
  for x in &lx {
    print!("{:?} ", x);
  }

  println!();

  parser::parse_term(&mut lx);

  if !check_valid_expr(&ast, 0) {
    println!("Syntax error: Lambda is not well formed");
    return;
  }

  let mut lambda_str = term_to_str(&ast, 0);

  println!("Lambda: {}", lambda_str);

  beta_reduce_once(&mut ast, 0);

  lambda_str = term_to_str(&ast, 0);

  println!("Lambda: {}", lambda_str);

}
