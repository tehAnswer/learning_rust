struct Philosopher {
  name: String
}

impl Philosopher {
  fn new(name: &str) -> Philosopher {
    Philosopher {
      name: name.to_string()
    }
  }
  fn eat(&self) {
    println!("{} is done eating.", self.name);
  }
}

fn main () {
  let philos = vec! [
    Philosopher::new("Baruch Spinoza"),
    Philosopher::new("Gilles Deleuze"),
    Philosopher::new("Karl Marx"),
    Philosopher::new("Friedrich Nietzsche"),
    Philosopher::new("El Chanclas")
  ];

  for p in &philos {
    p.eat()
  }
}
