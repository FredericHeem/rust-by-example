use rayon::prelude::*;

fn check_password(password: &str,  key_pair: &mut String) -> bool {
    println!("check_password {}", password);
    if password == "1" {
        println!("Gold ");
        key_pair.push_str("Gold");
        println!("key pair {}", key_pair);
        return true;
    }
    return false;
}

fn check_passwords(passwords: &Vec<&str>, mut key_pair: String) -> bool {
    println!("size {}", passwords.len());
    passwords.par_iter().find_any(|&&x| check_password(x, &mut key_pair));
    //key_pair = String::from("AAAAA");
    return true;
}

#[cfg(test)]
#[test]
fn find_any_test() {
    let passwords = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
    println!("size {}", passwords.len());
    let mut key_pair = String::from("");
    let result = check_passwords(&passwords, &mut key_pair);
    //passwords.par_iter().find_any(|&&x| check_password(x));
    println!("result {:?}, key pair {:?}", result, key_pair);
}