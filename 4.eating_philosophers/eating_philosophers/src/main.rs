use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
  name: String,
  left: usize,
  right: usize
}

struct Table {
  forks: Vec<Mutex<()>>
}

impl Table {
  fn new() -> Table {
      Table {
        forks: vec![
          Mutex::new(()),
          Mutex::new(()),
          Mutex::new(()),
          Mutex::new(()),
          Mutex::new(())
        ]
      }
  }
}

impl Philosopher {
  fn new(name: &str, left: usize, right: usize) -> Philosopher {
    Philosopher {
      name: name.to_string(),
      left: left,
      right: right
    }
  }
  fn eat(&self, table: &Table) {
    let _left = table.forks[self.left].lock().unwrap();
    let _right = table.forks[self.right].lock().unwrap();

    println!("{} is eating.", self.name);
    thread::sleep_ms(1000);
    println!("{} is done eating.", self.name);
  }
}

fn main () {
  let philos = vec! [
    Philosopher::new("Baruch Spinoza", 0, 1),
    Philosopher::new("Gilles Deleuze", 1, 2),
    Philosopher::new("Karl Marx", 2, 3),
    Philosopher::new("Friedrich Nietzsche", 3, 4),
    Philosopher::new("El Chanclas", 0, 4) // el chanclas es zurdo.
  ];

  let table = Arc::new(Table::new());

  println!("\nMultithread:");
  println!("-------------\n");

  let handles: Vec<_> = philos.into_iter().map(|philo| {
    let table = table.clone();
    thread::spawn(move || {
      philo.eat(&table);
    })
  }).collect();

  for h in handles {
    h.join().unwrap();
  }
}
