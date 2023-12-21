use std::fmt::format;

fn main() {
   let data = "internal global";
   let s1 = data.to_string();

   let s2 = String::from(" world");
   let s3 = s1 + &s2;
   
   let s = format!("{}{}", s2, s3);
   println!("{}", s);
   println!("{}", s3);
   let s4 = &s3[0..4];
   println!("{s4}");

   for b in "Зд".bytes() {
      println!("{b}");
  }

}
