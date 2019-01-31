// TODO: need a rust ide

use std::fmt::{ Display, Formatter, Result };

struct Structure(i32);

impl Display for Structure {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64
}

impl Display for Complex {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{} +{}i", self.real, self.imag)
  }
}

struct List(Vec<i32>);
impl Display for List {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let vec = &self.0;

    write!(f, "[")?;

    for (count, v) in vec.iter().enumerate() {
      if count != 0 { write!(f, ", ")?; }
      write!(f, "{}: {}", count, v)?;
    }

    write!(f, "]")
  }
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "RGB ({}, {}, {}) 0x{:X}{:X}{:X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
  }
}

fn main () {
  let s = Structure(12);
  println!("Display: {}", s);

  let complex = Complex{ real: 3.3, imag: 7.2};

  println!("Display: {}", complex);
  println!("Debug: {:?}", complex);

  let v = List(vec![1, 2, 3]);
  println!("{}", v);

  let c = Color{ red: 128, green: 255, blue: 90 };
  println!("{:?}", c);
  println!("{}", c);
}
