pub mod descending_order;
pub mod count_sheep;

fn main() {
  //println!("{}!", descending_order::descending_order(123));
  //println!("{}!", descending_order::descending_order(1253));
  //println!("{}!", descending_order::descending_order(25123));
  //println!("{}   ", count_sheep::count_sheep(&[true]));
  //println!("{}   ", count_sheep::count_sheep(&[true, false, true,true]));
  //println!("{}   ", count_sheep::count_sheep(&[false]));
    assert_eq!(count_sheep::count_sheep(&[false]), 0);
    assert_eq!(greet("Jim"),   "Hello, Jim!");
    assert_eq!(greet("Johnny"), "Hello, my love!");
    assert_eq!(string_to_number("12"), 12);
    assert_eq!(string_to_number("1234"), 1234);
    assert_eq!(string_to_number("-7"), -7);
    assert_eq!(even_or_odd(2), "Even");
    assert_eq!(even_or_odd(1), "Odd");

}

fn greet(input : &str) -> String {
    if input == "Johnny" {
        return "Hello, my love!".to_string();
    };
    return format!("Hello, {}!", input);
}

fn string_to_number(s: &str) -> i32 {
    let number: Vec<_> = s.chars().collect();
    number.into_iter().collect::<String>().parse::<i32>().unwrap()
}

fn even_or_odd(i: i32) -> &'static str {
    static EVEN: &'static str = "Even";
    static ODD: &'static str = "Odd";
    if i % 2 == 0{
        return EVEN;
    }else{
        return ODD;
    }
}
