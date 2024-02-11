fn fizzbuzz(num: u64) -> String {
    String::from("1")
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;
    /// 1を渡すと文字列'1'を返す
    #[test]
    fn passing_one_returns_the_string_one() {
        assert_eq!(fizzbuzz(1), String::from("1"));
    }
}