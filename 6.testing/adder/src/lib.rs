#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {
  assert!(false);
  assert_eq!("Hello", "world");
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}

#[test]
fn it_adds_correctly() {
  assert_eq!(4, add_two(2));
}
