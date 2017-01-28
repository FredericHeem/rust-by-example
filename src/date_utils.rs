#[cfg(test)]
use chrono::*;

#[test]
fn date_utc() {
    let utc: DateTime<UTC> = UTC::now();
    println!("UTC::now() {}", utc);
}
