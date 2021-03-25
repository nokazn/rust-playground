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

fn fizz_buzz10(n: u8) {
  let res = (1..=n)
    .map(|x: u8| -> String {
      match (x % 3, x % 5) {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
        _ => x.to_string(),
      }
    })
    .collect::<Vec<String>>()
    .join("\n");
  println!("{}", res);
}

use std::ops::Rem;

fn fizz_buzz11(n: u32) {
  fn fz<T>(x: T, div_3: T, dib_5: T, zero: T) -> String
  where
    T: Rem<T, Output = T> + Eq + Copy + ToString,
  {
    match (x % div_3 == zero, x % dib_5 == zero) {
      (true, true) => format!("FizzBuzz"),
      (true, _) => format!("Fizz"),
      (_, true) => format!("Buzz"),
      _ => x.to_string(),
    }
  }

  (1..=n)
    .map(|x| fz(x, 3, 5, 0))
    .for_each(|x| println!("{}", x))
}

mod mod1 {
  struct FizzBuzz<T> {
    div_3: T,
    div_5: T,
    zero: T,
  }

  impl<T> FizzBuzz<T> {
    fn new(div_3: T, div_5: T, zero: T) -> FizzBuzz<T> {
      FizzBuzz { div_3, div_5, zero }
    }
  }

  trait ToFzStr<T> {
    fn to_str(&self, x: T) -> String;
  }

  impl ToFzStr<u32> for FizzBuzz<u32> {
    fn to_str(&self, x: u32) -> String {
      match (x % self.div_3 == self.zero, x % self.div_5 == self.zero) {
        (true, true) => format!("FizzBuzz"),
        (true, _) => format!("Fizz"),
        (_, true) => format!("Buzz"),
        _ => x.to_string(),
      }
    }
  }

  pub fn fizz_buzz(n: u32) -> String {
    let fizz_buzzer = FizzBuzz::new(3, 5, 0);
    return (1..=n)
      .map(|x| fizz_buzzer.to_str(x))
      .collect::<Vec<String>>()
      .join("\n");
  }
}

#[test]
fn one_to_thirty() {
  const FIZZ_BUZZ_30: &str = "1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16
17
Fizz
19
Buzz
Fizz
22
23
Fizz
Buzz
26
Fizz
28
29
FizzBuzz";

  // assert_eq!(fizz_buzz1(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz2(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz3(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz4(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz5(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz6(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz7(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz8(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz9(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz10(30), FIZZ_BUZZ_30);
  // assert_eq!(fizz_buzz11(30), FIZZ_BUZZ_30);
  assert_eq!(mod1::fizz_buzz(30), FIZZ_BUZZ_30);
}

fn main() {
  fizz_buzz1(30);
  fizz_buzz2(30);
  fizz_buzz3(30);
  fizz_buzz4(30);
  fizz_buzz5(30);
  fizz_buzz6(30);
  fizz_buzz7(30);
  fizz_buzz8(30);
  fizz_buzz9(30);
  fizz_buzz10(30);
  fizz_buzz11(30);
  println!("{}", mod1::fizz_buzz(30));
}
