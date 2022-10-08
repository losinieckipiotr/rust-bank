pub fn is_valid_card_number(digits_str: &str) -> bool {
  let mut alt = false;

  let digits: Vec<i32> = digits_str
    .chars()
    .map(|character| {
      match character {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("invalid character")
      }
    })
    .collect();

  let mut sum: i32 = 0;
  let mut i = (digits.len() - 1) as i32;
  while i >= 0 {
    let mut temp = digits[i as usize];
    if alt {
      temp *= 2;
      if temp > 9 {
        temp -= 9; // same as add to digits eg. 1+6 = 7, 16-9 = 7
      }
    }
    sum += temp;
    alt = !alt;
    i -= 1;
  }

  return sum % 10 == 0;
}

mod tests {
  #[test]
  fn should_return_true() {
    assert_eq!(crate::luhn::is_valid_card_number("4000000130085832"), true)
  }
}
