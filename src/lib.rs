pub fn fizz_buzz(number: u64) ->  String {
    match number {
        i if i % (3*5) == 0 => String::from("FizzBuzz"),
        i if i % (5) == 0 => String::from("Buzz"),
        i if i % (3) == 0 => String::from("Fizz"),
        i => i.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _1_is_1() {
        assert_eq!(fizz_buzz(1), "1");
    }
    #[test]
    fn _2_is_2() {
        assert_eq!(fizz_buzz(2), "2");
    }
    #[test]
    fn _3_is_fizz() {
        assert_eq!(fizz_buzz(3), "Fizz");
    }
    #[test]
    fn _5_is_buzz() {
        assert_eq!(fizz_buzz(5), "Buzz");
    }
    #[test]
    fn _15_is_fizzbuzz() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
    }
}
