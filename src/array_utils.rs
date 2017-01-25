

pub fn sum_array(array : &[i32]) -> i32 {
  return array.iter().fold(0, |sum, x| sum + x);
}

#[cfg(test)]

#[test]
fn sum_array_test() {
    let array = [1, 2, 5, 6];
    let sum = sum_array(&array);
    println!("Sum {}", sum);
}

#[test]
fn concatenate_array_string() {
    let array = ["Alice", "Bob"];
    let concat = array.iter().fold("".to_string(), |sum, value| format!("{} {}", sum, value));
    println!("array {}", concat);
}
