#![allow(unused_variables)]

use std::error::Error;
use std::fmt;

use crate::{
  term::{
    Term,
    Link,
  },
  core::{
    dag::{
      Branch,
      BranchTag,
      Leaf,
      LeafTag,
      ParentCell,
      Single,
      SingleTag,
      DAG,
    },
    eval::{
      whnf,
      subst
    },
    uses::*
  },
};

type PreCtx = Vec<(String, DAG)>;
type Ctx    = Vec<(String, Uses, DAG)>;

#[inline]
pub fn add_ctx(ctx: &mut Ctx, ctx2: Ctx) {
  for i in 0..ctx.len() {
    ctx[0].1 = Uses::add(ctx[0].1, ctx2[0].1)
  }
}

#[inline]
pub fn mul_ctx(uses: Uses, ctx: &mut Ctx) {
  for mut bind in ctx {
    bind.1 = Uses::mul(bind.1, uses)
  }
}

#[derive(Debug)]
pub enum CheckError {
  GenericError(String),
}

impl fmt::Display for CheckError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CheckError::GenericError(msg) => write!(f,"{}",msg),
    }
  }
}

impl Error for CheckError {
  fn description(&self) -> &str {
    match self {
      CheckError::GenericError(msg) => &msg,
    }
  }
}

pub fn equal(a: DAG, b: DAG, dep: u64) -> bool {
  true
}

pub fn check(mut pre: PreCtx, uses: Uses, term: &Term, mut typ: DAG) -> Result<Ctx, CheckError> {
  match &term {
    Term::Lam(_, name, term_body) => {
      // A potential problem is that `term` might also mutate, which is an unwanted side-effect,
      // since `term` and `typ` might be coupled. To deal with this we will need to make copies
      // of `term` in the self rule
      whnf(typ);
      match typ {
        DAG::Branch(link) => {
          let Branch { tag, var, left: dom, right: img, .. } = unsafe { &*link.as_ptr() };
          match tag {
            BranchTag::All(lam_uses) => {
              // Annotate the depth of the node that binds var
              match var {
                None => panic!("Malformed DAG"),
                Some(link) => unsafe {
                  (*link.as_ptr()).depth = pre.len() as u64;
                }
              }
              // Add the domain of the function to the precontext
              pre.push((name.to_string(), *dom));
              let mut ctx = check(pre, Uses::Once, &**term_body, *img)?;
              let (_, inf_uses, _) = ctx
                .pop()
                .ok_or(CheckError::GenericError(String::from("Empty context")))?;
              if Uses::gth(inf_uses, *lam_uses) {
                Err(CheckError::GenericError(String::from("Quantity mismatch.")))
              }
              else {
                mul_ctx(uses, &mut ctx);
                Ok(ctx)
              }
            }
            _ => Err(CheckError::GenericError(String::from("The type of a lambda must be a forall."))),
          }
        },
        _ => Err(CheckError::GenericError(String::from("The type of a lambda must be a forall."))),
      }
    },
      
    Term::Dat(_, dat_body) => {
      whnf(typ);
      match typ {
        DAG::Single(link) => {
          let Single { tag, var, body: slf_body, .. } = unsafe { &*link.as_ptr() };
          match tag {
            SingleTag::Slf => {
              // If the self type does not bind any variable, then check the term's body
              // against the self type's body directly. Otherwise, subsitute the variable by the term
              let typ_body = match var {
                None => *slf_body,
                Some(var_link) => unsafe {
                  match (*var_link.as_ptr()).parents {
                    None => *slf_body,
                    Some(_) => {
                      let mut term_dag = DAG::from_open_term(pre.len() as u64, &term);
                      term_dag.uproot();
                      subst(link, term_dag)
                    }
                  }
                }
              };
              let ctx = check(pre, uses, dat_body, typ_body)?;
              Ok(ctx)
            }
            _ => Err(CheckError::GenericError(String::from("The type of data must be a self."))),
          }
        },
        _ => Err(CheckError::GenericError(String::from("The type of data must be a self."))),
      }
    },

    _ => {
      let depth = pre.len();
      let (ctx, infer_typ) = infer(pre, uses, term)?;
      if equal(typ, infer_typ, depth as u64) {
        Ok(ctx)
      }
      else {
        Err(CheckError::GenericError(String::from("Type mismatch.")))
      }
    },
  }
}

pub fn infer(mut pre: PreCtx, uses: Uses, term: &Term) -> Result<(Ctx, DAG), CheckError> {
  match term {
    Term::Var(_, _, _) => {
      panic!("TODO")
    }
    Term::Ref(_, _, _, _) => {
      panic!("TODO")
    }
    Term::Lam(_, _, _) => {
      panic!("TODO")
    }
    Term::App(_, _, _) => {
      panic!("TODO")
    }
    Term::Cse(_, _) => {
      panic!("TODO")
    }
    Term::All(_, _, _, _, _) => {
      panic!("TODO")
    }
    Term::Slf(_, _, _) => {
      panic!("TODO")
    }
    Term::Let(_, _, _, _, _, _, _) => {
      panic!("TODO")
    }
    Term::Typ(_) => {
      panic!("TODO")
    }
    Term::Ann(_, _, _) => {
      panic!("TODO")
    }
    Term::Lit(_, _) => {
      panic!("TODO")
    }
    Term::LTy(_, _) => {
      panic!("TODO")
    }
    Term::Opr(_, _) => {
      panic!("TODO")
    }
    _ => {
      panic!("TODO")
    }
  }
}
