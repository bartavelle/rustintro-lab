struct FizzBuzzIterator {
    // fill here
}

impl FizzBuzzIterator {
    fn new() -> Self {
        todo!()
    }
}

impl Iterator for FizzBuzzIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn fizzbuzz() {
        assert_eq!(
            super::FizzBuzzIterator::new().take(30).collect::<Vec<_>>(),
            [
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "Fizz", "22", "23",
                "Fizz", "Buzz", "26", "Fizz", "28", "29", "FizzBuzz"
            ]
        )
    }
}
