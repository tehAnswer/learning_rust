use std::thread;

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
    println!("{} is eating.", self.name);
    thread::sleep_ms(1000);
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

  println!("\nSecuential:");
  println!("-------------\n");

  for p in &philos {
    p.eat()
  }

  thread::sleep_ms(500);

  println!("\nMultithread:");
  println!("-------------\n");

  let handles: Vec<_> = philos.into_iter().map(|philo| {
    thread::spawn(move || {
      philo.eat();
    })
  }).collect();

  for h in handles {
    h.join().unwrap();
  }
}
