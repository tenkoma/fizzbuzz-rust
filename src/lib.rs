fn fizzbuzz(num: u64) -> String {
    match num {
        2 => String::from("2"),
        _ => String::from("1")
    }
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;
    /// 1を渡すと文字列'1'を返す
    #[test]
    fn passing_one_returns_the_string_one() {
        assert_eq!(fizzbuzz(1), String::from("1"));
    }

    /// 2を渡すと文字列'2'を返す
    #[test]
    fn passing_two_returns_the_string_two() {
        assert_eq!(fizzbuzz(2), String::from("2"));
    }
}