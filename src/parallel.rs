use rayon::prelude::*;

fn check_password(password: &str) -> bool {
    println!("check_password {}", password);
    if password == "1" {
        return true;
    }
    return false;
}

fn check_passwords(passwords: Vec<&str>) -> bool {
    println!("size {}", passwords.len());
    passwords.par_iter().find_any(|&&x| check_password(x));
    return true;
}

#[cfg(test)]
#[test]
fn find_any_test() {
    let passwords = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
    println!("size {}", passwords.len());
    let result = passwords.par_iter().find_any(|&&x| check_password(x));
    println!("result {:?}", result);
}