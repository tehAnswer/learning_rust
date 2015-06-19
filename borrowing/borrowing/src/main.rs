fn main() {
  let mut tupac :String = "2Pac".to_string();
  let mut makaveli :String = "Makaveli".to_string();

  swap(&mut tupac, &mut makaveli);

  println!("2pac: {}, makaveli: {}", tupac, makaveli);
 
  say_hello(&tupac);
  change_name(&mut tupac, &makaveli);
  say_hello(&tupac);


  say_hello(&tupac); // 2Pac said: Hello.
}

fn say_hello(name: &String) {
  println!("{} said: Hello.", *name);
}

fn change_name(old_name: &mut String, new_name: &String) {
  *old_name = new_name.to_string();
}

fn swap(name1: &mut String, name2: &mut String) {
  let tmp :String = name1.to_string();
  *name1 = name2.to_string();
  *name2 = tmp;
}

