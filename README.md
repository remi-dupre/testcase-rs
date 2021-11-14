Generic parser for competitive programming
==========================================

This is a generic parser for competitive programming, it can be used to read
structured data line-by-line or though derive macro in a higher level fashion.


Functional API
--------------

You can take any `BufRead` input and parse each lines into a structured output.
Output can be strings, numerics or any combination of tuple, sized array or
Vec.

```rust
use testcase::prelude::*;

let input = "1 2\n3 4\n";

assert_eq!(
    input.as_bytes().parse_line::<(i32, i32)>().unwrap(),
    (1, 2)
);
```

Derive API
----------

You can define complex layouts using the `TestCase` derive in combination of
`parse_testcase`, each `#[testcase(line)]` attribute symbolizes a newline in
the input.

```rust
use testcase::prelude::*;
use testcase_derive::TestCase;

#[derive(Debug, TestCase)]
struct Input {
    #[testcase(line)]
    n: usize,
    #[testcase(lines = "n")]
    table: Vec<[u32; 2]>,
}

let input = "2\n1 2\n3 4\n";

let testcase: Input = input.as_bytes()
    .parse_testcase()
    .unwrap();

assert_eq!(
    testcase.table,
    vec![
        [1, 2],
        [3, 4]
    ],
);
```


Domain-specific APIs
--------------------

### Facebook Hackercup

The `#[hackercup]` will generate your main function for a single instance, it
will automatically iterate over all problems and parse inputs using the
`TestCase` trait.

```rust
use testcase_derive::{hackercup, TestCase};

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _k: usize,
    #[testcase(line)]
    text: String,
}

fn solve_problem(input: &Input) -> u64 {
    todo!()
}

// Tests can automatically be generated if you specify sample input and output:
// #[hackercup(input = "test_sample.in", output = "test_sample.out")]
#[hackercup]
fn solve(input: Input) -> u64 {
    solve_problem(&input)
}
```
