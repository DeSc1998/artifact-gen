mod artifact;
mod stat;
mod statgrowth;

use artifact::*;

fn main() {
  for _ in 0..20 {
    let mut a = generate_arifact(5);
    println!("{}", a);
    a.add_levels(20);
    println!("{}", a);
    println!("----------------------------");
  }
}
