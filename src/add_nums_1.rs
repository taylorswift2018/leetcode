pub fn add_digits(mut num: i32) -> i32 {
    let mut root = 0;
    while num > 0 {
        root += num % 10;
        num = num / 10;

        if num == 0 && root > 9 {
            num = root;
            root = 0;
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::add_digits;

    #[test]
    fn it_works() {
        assert_eq!(add_digits(123), 6);
    }
}
