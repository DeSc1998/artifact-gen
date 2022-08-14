mod artifact;
mod stat;
mod statgrowth;

#[warn(unused_imports)]
use artifact::*;

fn usage() {
  let prog_anme = &std::env::args().take(1).collect::<Vec<String>>()[0];
  println!("{} [<options>] <count>", prog_anme);
  println!("  Options:" );
  println!("   -l           level the artifact to +20");
  println!("   -h, --help   print this help");
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let input = &args[1..];
  let last_value = input.last();
  let should_level = args.iter().find( |&e| e == "-l" );

  if let Some(_) = args.iter().find( |&e| e == "-h" || e == "--help" ) {
    usage();
    return
  }

  if last_value == None {
    eprintln!("ERROR: not enough arguments provided");
    usage();
    return
  }

  match (last_value.unwrap().parse::<u32>(), should_level) {
    (Ok(val), Some(_)) => {
      for i in 0..val {
        let mut a = generate_arifact(5);
        println!("new:\n{}", a);
        a.add_levels(20);
        println!("leveled:\n{}", a);
        if i < val - 1 {
          println!("----------------------");
        }
      }
    },
    (Ok(val), None) => {
      for _ in 0..val {
        println!("{}", generate_arifact(5));
      }
    },
    (Err(error), _) => {
      eprintln!("ERROR: {}", error);
      usage()
    },
  }
}
