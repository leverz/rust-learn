use std::fmt::{ Display, Formatter, Result };

struct Matrix (f32, f32, f32, f32);

impl Display for Matrix {
  fn fmt (&self, f: &mut Formatter) -> Result {
    writeln!(f, "( {} {} )", self.0, self.1)?;
    writeln!(f, "( {} {} )", self.2, self.3)
  }
}

fn transpose (m: &Matrix) -> Matrix {
  Matrix(m.0, m.2, m.1, m.3)
}

fn main () {
  let m = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("Matrix:");
  println!("{}", m);
  println!("Transpose:");
  println!("{}", transpose(&m));
}
