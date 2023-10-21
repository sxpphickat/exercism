pub fn reverse(input: &str) -> String {
  //  todo!("Write a function to reverse {input}");
    let mut rev = String::new();
    for c in input.chars().rev() {
        rev.push(c);
    }
    rev
}
