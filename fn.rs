fn main() {
  let x: i16 = 1;

  let y = {
    let x: i16 = 3;
    x + 1
  };

  println!("y is {}", y)
}