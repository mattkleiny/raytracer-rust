#![allow(dead_code)]

use math::*;

mod math;

fn main() {
  let a = vec(2., 3., 4.);
  let b = vec(1., -6., 8.);

  print!("a * b = {:?}", a + b);
}