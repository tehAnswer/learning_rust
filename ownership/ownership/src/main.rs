fn main() {
  // IT DOESN'T COMPILE BECAUSE IT VIOLATES THE
  // RUST SECURITY GUIDELINES. 
  let vector = vec![0,1,2];
  let (x1, x2) = (vector, vector);
  println!("{} is 0", x1[0]);
  println!("{} is 0", x2[0]);
}
