///! Extention traits for the standart input method to easily read testcase
///! structures.
use crate::testcase::{BlockTestCase, InlineTestCase, TestCase};
use crate::Error;
use std::io::BufRead;

pub trait ParseLine {
    fn parse_line<T: InlineTestCase>(&mut self) -> Result<T, Error>;
}

impl<R: BufRead> ParseLine for R {
    /// Consume a single line from the input to build a testcase.
    ///
    /// # Example
    ///
    /// ```
    /// use testcase::prelude::*;
    ///
    /// let input = "1 2\n3 4\n";
    ///
    /// assert_eq!(
    ///     input.as_bytes().parse_line::<(i32, i32)>().unwrap(),
    ///     (1, 2)
    /// );
    /// ```
    fn parse_line<T: InlineTestCase>(&mut self) -> Result<T, Error> {
        let mut buf = String::new();
        self.read_line(&mut buf)?;
        T::parse_line(buf.trim())
    }
}

pub trait ParseBlock {
    /// Consume a given number of lines from the input to build the testcase.
    ///
    /// # Example
    ///
    /// ```
    /// use testcase::prelude::*;
    ///
    /// let input = "1 2\n3 4\n5 6\n";
    ///
    /// let testcase: Vec<Vec<i32>> = input.as_bytes()
    ///     .parse_lines(2)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     testcase,
    ///     vec![
    ///         vec![1, 2],
    ///         vec![3, 4]
    ///     ],
    /// );
    ///
    /// ```
    fn parse_lines<T: BlockTestCase>(&mut self, n: usize) -> Result<T, Error>;
}

impl<R: BufRead> ParseBlock for R {
    fn parse_lines<T: BlockTestCase>(&mut self, n: usize) -> Result<T, Error> {
        T::parse_n(self, n)
    }
}

pub trait Parse {
    /// Consume the input freely until the testcase is ready.
    ///
    /// # Example
    ///
    /// ```
    /// use testcase::prelude::*;
    /// use testcase_derive::TestCase;
    ///
    /// #[derive(Debug, TestCase)]
    /// struct Input {
    ///     #[testcase(lines = "2")]
    ///     table: Vec<[u32; 2]>,
    /// }
    ///
    /// let input = "1 2\n3 4\n";
    ///
    /// let testcase: Input = input.as_bytes()
    ///     .parse_testcase()
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     testcase.table,
    ///     vec![
    ///         [1, 2],
    ///         [3, 4]
    ///     ],
    /// );
    ///
    /// ```
    fn parse_testcase<T: TestCase>(&mut self) -> Result<T, Error>;
}

impl<R: BufRead> Parse for R {
    fn parse_testcase<T: TestCase>(&mut self) -> Result<T, Error> {
        T::parse(self)
    }
}
