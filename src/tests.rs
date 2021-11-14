use testcase::prelude::*;
use testcase_derive::TestCase;

#[test]
fn test_inline_only() {
    #[derive(Debug, TestCase, Eq, PartialEq)]
    struct Test {
        #[testcase(line)]
        n: usize,
        #[testcase(line)]
        s: String,
        #[testcase(line)]
        m: i32,
    }

    let test: Test = "18\ndan\n42\n".as_bytes().parse_testcase().unwrap();

    assert_eq!(
        test,
        Test {
            n: 18,
            s: "dan".to_string(),
            m: 42,
        }
    )
}

#[test]
fn test_inline_tuple() {
    #[derive(Debug, TestCase, Eq, PartialEq)]
    struct Test {
        #[testcase(line)]
        n: usize,
        s: String,
        #[testcase(line)]
        x: u8,
        y: u32,
    }

    let test: Test = "18 dan\n5 6\n".as_bytes().parse_testcase().unwrap();

    assert_eq!(
        test,
        Test {
            n: 18,
            s: "dan".to_string(),
            x: 5,
            y: 6,
        }
    )
}

#[test]
fn test_lines() {
    #[derive(Debug, TestCase, Eq, PartialEq)]
    struct Test {
        #[testcase(line)]
        n: usize,
        #[testcase(lines = "n")]
        edges: Vec<(u64, u64)>,
    }

    let test: Test = "3\n1 2\n3 4\n5 6\n".as_bytes().parse_testcase().unwrap();

    assert_eq!(
        test,
        Test {
            n: 3,
            edges: vec![(1, 2), (3, 4), (5, 6)]
        }
    )
}
