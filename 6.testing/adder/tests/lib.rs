extern crate adder;

#[test]
fn it_adds() {
  assert_eq!(4, adder::add_two(2));
}