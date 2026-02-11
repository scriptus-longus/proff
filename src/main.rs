
static VAR_NAMES: [char; 21] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u'];

enum Term {
  Var(u32),
  Lam(Box<Term>),
  Appl(Box<Term>, Box<Term>),
}

fn term_to_str(t: &Term, depth: u32) -> String{
  let mut ret = String::new();

  match t {
    Term::Lam(x) => {
      let var = VAR_NAMES[depth as usize]; 
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
    Term::Lam(x) => {
      check_valid_expr(x, depth+1)
    },
    Term::Appl(x, y) => {
      check_valid_expr(x, depth) && check_valid_expr(y, depth)
    },
    Term::Var(x) => {
      if *x < depth {
        true
      } else {
        false
      }
    },
  }
}

fn main() {
  let ast = Term::Lam(
              Box::new(Term::Lam(
                Box::new(Term::Appl(
                  Box::new(Term::Var(0)),
                  Box::new(Term::Var(1))
                ))
              ))
            );

  if !check_valid_expr(&ast, 0) {
    println!("Syntax error: Lambda is not well formed");
    return;
  }

  let lambda_str = term_to_str(&ast, 0);

  println!("Lambda: {}", lambda_str);
}
