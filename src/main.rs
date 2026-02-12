
static VAR_NAMES: [char; 21] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u'];

#[derive(Clone)]
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
    Term::Var(_) => {
      true
    },
  }
}

fn subs_vars (f: &Term, x: u32, y: &Term) -> Term {
  // Term{ perform A B [x := y]
  match f {
    Term::Var(t) => {
      if *t == x {
        y.clone()
      } else {
        Term::Var(*t)
      }
    },
    Term::Lam(s) => {
      Term::Lam(Box::new(subs_vars(s, x, y)))
    }
    Term::Appl(r, l) => {
      Term::Appl(Box::new(subs_vars(r, x, y)), Box::new(subs_vars(l, x, y)))
    }
  }
}

fn beta_reduce(t: &mut Term, depth: u32) -> Term {
  match t {
    Term::Appl(k, y) => {
      if let Term::Lam(body) = &mut **k {
        println!("HI");
        let body = body.clone();
        let arg = y.clone();

        subs_vars(&body, depth, &arg)
      } else {
        Term::Appl(Box::new(beta_reduce(k, depth+1)), Box::new(beta_reduce(y, depth+1)))
      }
    },
    Term::Var(x) => {
      let v = x.clone();
      Term::Var(v)
    },
    Term::Lam(x) =>  {
      beta_reduce(x, depth+1)
    }
  }
}

fn main() {
  let mut ast = Term::Appl(
                  Box::new(Term::Lam(
                    Box::new(Term::Appl(
                      Box::new(Term::Var(0)),
                      Box::new(Term::Var(3))
                      ))
                    )),

                  Box::new(Term::Var(1))
                );

  if !check_valid_expr(&ast, 0) {
    println!("Syntax error: Lambda is not well formed");
    return;
  }

  let mut lambda_str = term_to_str(&ast, 0);

  println!("Lambda: {}", lambda_str);

  ast = beta_reduce(&mut ast, 0);

  lambda_str = term_to_str(&ast, 0);

  println!("Lambda: {}", lambda_str);

}
