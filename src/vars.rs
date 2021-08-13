pub fn vars(){
  let name ="Geonil";
  let mut age = 20; // const = let , let = let mut
  println!("My name is {} and I am {}",name, age);

  age = 9;
  println!("My name is {} and I am {}",name, age);
  
  // Define constant
  const ID:i32 = 100;
  println!("ID:{}",ID);

  // Assign multiple vars
  let (my_name, my_age) = ("geonil", 30);
  println!("i am {} and {}",my_name,my_age);


}