use std::collections::HashMap;

use crate::parser::Type;
use crate::parser::Term;

pub struct Context {
  vars: HashMap<u32, Type>,
}


impl Context {
  pub fn new() -> Self {
    Context {vars:  HashMap::new()}
  }

  pub fn obtain_var(&self, v: u32) -> Option<Type> {
    match self.vars.get(&v) {
      Some(t) => Some(t.clone()),
      None => None,
    }
  }

  pub fn add_var(&mut self, v: u32, t: &Type) {
    println!("HI");
    let t_clone = t.clone();
    self.vars.insert(v, t_clone);
  }
}

pub fn type_check(ast: &Term, ctx: &mut Context) -> Result<Type, &'static str> {
  match ast {
    Term::Var(x) => {
      println!("Var: {}", x);
      match ctx.obtain_var(*x) {
        Some(x) => Ok(x),
        None => Err("Variable not found in context"),
      }
    },
    Term::Appl(x, y) => {
      let f_type = type_check(x, ctx);
      let arg_type = type_check(y, ctx).unwrap();
      match f_type {
        Ok(Type::Arrow(x, y)) => {
          if *x == arg_type {
            Ok(*y.clone())
          } else {
            Err("Could not apply, types do not match")
          }
        },
        _ => Err("Could not apply, types do not match"),
      }
      /*let f_type = type_check(x, ctx);
      let arg_type = type_check(y, ctx).unwrap();

      println!("FTYPE: {:?}", f_type);
      println!("arg_type: {:?}", arg_type);

      Ok(Type::Unit)*/
    },
    Term::Lam(arg, fun, t)  => {
      ctx.add_var(*arg, t);
      let body_t = match type_check(fun, ctx) {
        Ok(t) => t,
        x => return x,
      };
      
      let ty = *t.clone();
      Ok(Type::Arrow(Box::new(ty), Box::new(body_t)))
    }
  }
}
