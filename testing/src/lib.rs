#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;  // dep for test.

pub mod lib {
    /// First line is a short summary describing function.
    ///
    /// The next lines present detailed documentation. Code blocks start with
    /// triple backquotes and have implicit `fn main()` inside
    /// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
    ///
    /// ```
    /// let result = testing::lib::add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(lib::add(2, 3), 5);
    }
}
