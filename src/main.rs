pub mod descending_order;
pub mod count_sheep;

fn main() {
  //println!("{}!", descending_order::descending_order(123));
  //println!("{}!", descending_order::descending_order(1253));
  //println!("{}!", descending_order::descending_order(25123));
  println!("{}   ", count_sheep::count_sheep(&[true]));
  println!("{}   ", count_sheep::count_sheep(&[true, false, true,true]));
  println!("{}   ", count_sheep::count_sheep(&[false]));
}
