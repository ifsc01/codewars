// https://www.codewars.com/kata/summy/rust

fn summy(strng: &str) -> i32 {
  strng.split(' ').map(|n| {
    n.parse::<i32>().unwrap()
  }).sum()
}

#[test]
fn sample_tests() {
    assert_eq!(summy("1 2 3"), 6);
    assert_eq!(summy("1 2 3 4"), 10);
    assert_eq!(summy("1 2 3 4 5"), 15);
    assert_eq!(summy("10 10"), 20);
    assert_eq!(summy("0 0"), 0);
}