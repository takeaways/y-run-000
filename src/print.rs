// export = pub
pub fn run(){
  println!("Hello from print");
  
  // Basic Formatting
  println!("Number: {} is ..{}",1,"b");

  // Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}","brad","mass","code");

  // Named Arguments
  println!("{name} likes to play {activity}", name="geonil", activity="hello");

  // placeholder traits
  println!("binary: {:b} Hex:{:x} Octal: {:o}",10,10,10);

  // placeholder for debug trait
  println!("{:?}",(12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}",10+10);
}