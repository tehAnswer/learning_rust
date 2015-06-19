fn main() {
  let mut x = 5;
  let mut done = false;

  while x % 5 != 0 {
    x += x - 3;
    println!("{}", x);
  }
}
