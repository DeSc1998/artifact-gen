mod artifact;
mod stat;
mod statgrowth;

#[warn(unused_imports)]
use artifact::*;

fn usage() {
  let prog_anme = &std::env::args().take(1).collect::<Vec<String>>()[0];
  println!("{} <count>", prog_anme)
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let input = &args[1..];

  match input[0].parse::<u32>() {
    Ok(val) => {
      for _ in 0..val {
        println!("{}", generate_arifact(5));
      }
    },
    Err(error) => {
      eprintln!("ERROR: {}", error);
      usage()
    },
  }

  // for _ in 0..20 {
  //   let mut a = generate_arifact(5);
  //   println!("{}", a);
  //   a.add_levels(20);
  //   println!("{}", a);
  //   println!("----------------------------");
  // }
}
