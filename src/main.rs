use std::{
  fs,
  path::PathBuf,
};

#[cfg(not(target_arch = "wasm32"))]
use std::io::{self, Read};
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use yatima::{
  core,
  hashspace,
  parse,
  repl,
};

#[cfg(target_arch = "wasm32")]
use yatima::{
  core,
  hashspace,
  parse,
};

#[derive(Debug, StructOpt)]
#[structopt(about = "A programming language for the decentralized web")]
enum Cli {
  Save {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
  },
  Show {
    input: String,
  },
  Parse {
    #[structopt(parse(from_os_str))]
    optPath: Option<PathBuf>,
  },
  Run {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
  },
  Repl,
}
//

#[cfg(not(target_arch = "wasm32"))]
fn main() -> io::Result<()> {
  let command = Cli::from_args();
  match command {
    Cli::Repl => repl::main().unwrap(),
    Cli::Parse { optPath } => {
      match optPath {
        Some(path) => {
          let env = parse::package::PackageEnv::new(path);
          let (_, p, ..) = parse::package::parse_file(env);
          println!("Package parsed:\n{}", p);
        }
        None => {
          let mut source = String::new();
          let stdin = io::stdin();
          let mut handle = stdin.lock();
          handle.read_to_string(&mut source)?;
          let (_, p, ..) = parse::package::parse_text(source.as_str(), None);
          println!("Package parsed:\n{}", p);
        }
      }
    }
    Cli::Run { input } => {
      let env = parse::package::PackageEnv::new(input.clone());
      let (_, p, defs, refs) = parse::package::parse_file(env);
      let (def_link, _) = refs.get("main").expect(&format!(
        "No `main` expression in package {} from file {:?}",
        p.name, input
      ));
      let def = defs.get(def_link).expect("Unknown link for `main` expression");
      let dag = core::dag::DAG::from_term(def.to_owned().term);
      let red = core::eval::norm(&defs, dag);
      println!("{}", red);
    }
    Cli::Save { input } => {
      let string = fs::read_to_string(input).unwrap();
      let expr = hashexpr::parse(&string).unwrap().1;
      let link = hashspace::put(expr);
      println!("Saved as {}", link)
    }
    Cli::Show { input } => {
      let link = hashexpr::link::Link::parse(&input).expect("valid link").1;
      println!("link {:?} {}", link, link);
      let expr = hashspace::get(link).expect("unknown link");
      println!("{}", expr)
    }
  }
  Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
  let command = Cli::from_args();
  match command {
    Cli::Repl => println!("repl not working yet"),
      // repl::main().unwrap(),
    Cli::Parse { input } => {
      let env = parse::package::PackageEnv::new(input);
      let (_, p, ..) = parse::package::parse_file(env);
      println!("Package parsed:\n{}", p);
    }
    Cli::Run { input } => {
      let env = parse::package::PackageEnv::new(input.clone());
      let (_, p, defs, refs) = parse::package::parse_file(env);
      let (def_link, _) = refs.get("main").expect(&format!(
        "No `main` expression in package {} from file {:?}",
        p.name, input
      ));
      let def = defs.get(def_link).expect("Unknown link for `main` expression");
      let dag = core::dag::DAG::from_term(def.to_owned().term);
      let red = core::eval::norm(&defs, dag);
      println!("{}", red);
    }
    Cli::Save { input } => {
      let string = fs::read_to_string(input).unwrap();
      let expr = hashexpr::parse(&string).unwrap().1;
      let link = hashspace::put(expr);
      println!("Saved as {}", link)
    }
    Cli::Show { input } => {
      let link = hashexpr::link::Link::parse(&input).expect("valid link").1;
      println!("link {:?} {}", link, link);
      let expr = hashspace::get(link).expect("unknown link");
      println!("{}", expr)
    }
  }
}
