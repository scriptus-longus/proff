
mod lexer;
mod parser;

use parser::Term;
use parser::Type;

fn type_to_str(t: &Type) -> String {
  let mut ret = String::new();

  match t {
    Type::Unit => ret.push_str("U"),
    Type::Empty => ret.push_str("0"),
    Type::Arrow(l, r) => {
      let left = type_to_str(l);
      ret.push_str(&left);
      ret.push_str(" -> ");
      let right = type_to_str(r);
      ret.push_str(&right);
    }
  }

  ret
}

fn term_to_str(t: &Term, depth: u32) -> String{
  let mut ret = String::new();

  match t {
    Term::Lam(v, x, t) => {
      //let var = VAR_NAMES[*v as usize]; 
      let var = ((*v as u8) + b'a') as char;
      ret.push_str("(");
      ret.push_str(&format!("\\{}.", var));
      ret.push_str(&term_to_str(x, depth+1));
      ret.push_str(")    [");
      let type_str = type_to_str(t);
      ret.push_str(&type_str);
      ret.push_str("]");
      ret
    },
    Term::Appl(x, y) => {
      ret.push_str(&term_to_str(x, depth));
      ret.push_str(" ");
      ret.push_str(&term_to_str(y, depth));
      ret
    },
    Term::Var(x) => {
      //ret.push(VAR_NAMES[*x as usize]);
      ret.push(((*x as u8) + b'a') as char);
      ret
    },
  }
}

fn check_valid_expr(t: &Term, depth: u32) -> bool {
  match t {
    Term::Lam(_, x, _) => {
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
    Term::Lam(_, s, _) => {
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
      if let Term::Lam(v, body, _) = &mut **k {
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
    Term::Lam(_, x, _) =>  {
      beta_reduce_once(x, depth+1)
    }
  }
}


fn main() {
  let mut lx = lexer::lex_text(String::from("(\\a : A -> A. a t)"));
  for x in &lx {
    println!("{:?}", x);
  }
  lx.reverse();

  let mut ast = parser::parse_term(&mut lx);

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
