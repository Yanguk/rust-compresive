#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    // 모든 공백을 무시합니다. 2자리 미만 숫자는 무시합니다.
    //
    // 오른쪽에서 왼쪽으로 이동하며 2번째 자리마다 숫자를 2배 증가시킵니다. 예를 들어 1234에서 3과 1에 각각 2를 곱합니다.
    //
    // 두배로 만든 숫자가 2자리라면 각 자리 숫자를 더합니다. 예를 들어, 7은 두배로 만들면 14이므로 5가 됩니다.
    //
    // 모든 자리의 숫자를 더합니다.
    //
    // 합계의 끝자리가 0인 경우 유효한 신용카드 번호입니다.
    let number_iter = cc_number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    if number_iter.len() < 2 {
        return false;
    }

    let number_sum: u32 = number_iter
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, digit)| {
            if index % 2 == 1 {
                let d = digit * 2;
                if d > 9 { d / 10 + d % 10 } else { d }
            } else {
                digit
            }
        })
        .sum();

    number_sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
