
pub fn sum_array(array : &[i32]) -> i32 {
  //println!("Sum item {}", array[6]);
  return array.iter().fold(0, |sum, x| sum + x);
}

#[cfg(test)]
mod test {
use super::sum_array;

#[test]
fn sum_array_test() {

    println!("Hello, world!");
    let array = [1, 2, 5, 6];
    let sum = sum_array(&array);
    println!("Sum {}", sum);

}


}
