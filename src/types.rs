pub fn types(){
  // Default is i32;
  let x = 1;
  println!("{}",x);
  // f64
  let y = 2.5;
  println!("{}",y);

  
  let z: i64 = 4545454545454545;
  println!("{}",z);

  // Find max size
  println!("Max i32: {}",std::i32::MAX);
  println!("Max i64: {}",std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Get boolean from expression
  let is_greater = 100 < 5;


  // Char on quote
  let a1 = 'a';
  let face = '\u{1F600}';


  println!("{:?}",(x,y,z,is_active,is_greater,a1,face));

}