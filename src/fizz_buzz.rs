fn fizz_buzz1(n: u8) {
  let mut x = 1;
  while x <= n {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 3 == 0 {
      println!("Fizz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x);
    }
    x += 1;
  }
}

fn fizz_buzz2(n: u8) {
  for x in 1..=n {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 3 == 0 {
      println!("Fizz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x);
    }
  }
}

fn fizz_buzz3(n: u8) {
  for x in 1..=n {
    match x % 15 {
      0 => println!("FizzBuzz"),
      3 | 6 | 9 | 12 => println!("Fizz"),
      5 | 10 => println!("Buzz"),
      _ => println!("{}", x),
    }
  }
}

fn fizz_buzz4(n: u8) {
  for x in 1..=n {
    match x {
      e if e % 15 == 0 => println!("FizzBuzz"),
      e if e % 3 == 0 => println!("Fizz"),
      e if e % 5 == 0 => println!("Buzz"),
      e => println!("{}", e),
    }
  }
}

fn fizz_buzz5(n: u8) {
  for x in 1..=n {
    match (x % 3, x % 5) {
      (0, 0) => println!("FizzBuzz"),
      (0, _) => println!("Fizz"),
      (_, 0) => println!("Buzz"),
      _ => println!("{}", x),
    }
  }
}

fn fizz_buzz6(n: u8) {
  for x in 1..=n {
    let s: String = match (x % 3, x % 5) {
      (0, 0) => "FizzBuzz".to_string(),
      (0, _) => "Fizz".to_string(),
      (_, 0) => "Buzz".to_string(),
      _ => x.to_string(),
    };
    println!("{}", s);
  }
}

fn fizz_buzz7(n: u8) {
  for x in 1..=n {
    let tmp;
    let s = match (x % 3, x % 5) {
      (0, 0) => "FizzBuzz",
      (0, _) => "Fizz",
      (_, 0) => "Buzz",
      _ => {
        tmp = x.to_string();
        &tmp
      }
    };
    println!("{}", s);
  }
}

fn fizz_buzz8(n: u8) {
  let fz = |x: u8| match (x % 3, x % 5) {
    (0, 0) => format!("FizzBuzz"),
    (0, _) => format!("Fizz"),
    (_, 0) => format!("Buzz"),
    _ => x.to_string(),
  };
  (1..=n).map(fz).for_each(|x| println!("{}", x));
}

fn fizz_buzz9(n: u8) {
  let res = (1..=n).fold(String::from(""), |prev, x| match (x % 3, x % 5) {
    (0, 0) => format!("{}FizzBuzz\n", prev),
    (0, _) => format!("{}Fizz\n", prev),
    (_, 0) => format!("{}Buzz\n", prev),
    _ => format!("{}{}\n", prev, x),
  });
  println!("{}", res)
}

fn main() {
  fizz_buzz1(50);
  fizz_buzz2(50);
  fizz_buzz3(50);
  fizz_buzz4(50);
  fizz_buzz5(50);
  fizz_buzz6(50);
  fizz_buzz7(50);
  fizz_buzz8(50);
  fizz_buzz9(50);
}
