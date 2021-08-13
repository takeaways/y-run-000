pub fn string_fn(){
  let mut hello = String::from("Hello---------------- ");
  println!("{}",hello);

  // length
  println!("Length : {}", hello.len());

  hello.push('W');
  hello.push_str("orld");

  // Capacity in bytes
  println!("Capacity : {}", hello.capacity());

  // Check is empty
  println!("Is Empty: {}", hello.is_empty());

  // Contains
  println!("Contains 'World' {} ", hello.contains("orld"));

  // Replace
  println!("Replace: {}", hello.replace("World","Geonil"));

  // Loop through string by whitespace
  for word in hello.split_whitespace(){
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  
  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());





  println!("result : --------- {}",hello);  
}
pub fn run(){
  println!("this is inside of string")
}