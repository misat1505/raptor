use std::io::BufReader;

use crate::common::assert_same_output;

#[test]
fn functions() {
    let text = BufReader::new(
        r#"
        fn add(i64 a, i64 b): i64 {
            return a + b;
        }

        i64 a = add(1, 2);
        println(a as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "3\n");
}

#[test]
fn recursion() {
    let text = BufReader::new(
        r#"
        fn fib(i64 x): i64 {
            if (x == 1 || x == 2) {
                return 1;
            }

            return fib(x - 1) + fib(x - 2);
        }

        i64 x = fib(6);
        println(x as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "8\n");
}

#[test]
fn is_prime() {
    let text = BufReader::new(
        r#"
    fn is_prime(i64 x): bool {
      if (x < 2) {
        return false;
      }

      for (i64 i = 2; i < x / 2; i = i + 1) {
        if (x % i == 0) {
          return false;
        }
      }

      return true;
    }

    bool is_5 = is_prime(5);
    bool is_6 = is_prime(6);
    println(is_5 as str);
    println(is_6 as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "true\nfalse\n");
}

#[test]
fn function_returning_vector() {
    let text = BufReader::new(
        r#"
    fn make_range(i64 n): i64[] {
      i64[] result = [];
      for (i64 i = 0; i < n; i = i + 1) {
        vector_push(&result, i);
      }
      return result;
    }

    i64[] range = make_range(4);
    i64 sum = 0;
    for (i64 i = 0; i < vector_size(&range); i += 1) {
      sum += range[i];
    }

    println(sum as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "6\n");
}

#[test]
fn function_without_explicit_return_and_void_type_ok() {
    let text = BufReader::new(
        r#"
    fn do_nothing(): void {
      i64 x = 5;
    }
    do_nothing();
    i64 marker = 1;

    println(marker as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n");
}

#[test]
fn recursive_function_with_reference_accumulator() {
    let text = BufReader::new(
        r#"
    fn count_down(i64 n, &i64 steps): void {
      if (n <= 0) {
        return;
      }
      steps = steps + 1;
      count_down(n - 1, &steps);
    }

    i64 steps = 0;
    count_down(5, &steps);

    println(steps as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "5\n");
}

#[test]
fn void_function_calling_another_function() {
    let text = BufReader::new(
        r#"
    fn helper(i64 x): i64 {
      return x * 2;
    }

    fn main_logic(&i64 result): void {
      result = helper(21);
    }

    i64 output = 0;
    main_logic(&output);

    println(output as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "42\n");
}

#[test]
fn chained_index_and_function_call() {
    let text = BufReader::new(
        r#"
    fn make_matrix(): i64[][] {
      return [[1, 2], [3, 4]];
    }

    i64 x = make_matrix()[1][0];
    
    println(x as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "3\n");
}

#[test]
fn recursive_function_multiple_params_gcd() {
    let text = BufReader::new(
        r#"
    fn gcd(i64 a, i64 b): i64 {
      if (b == 0) {
        return a;
      }
      return gcd(b, a % b);
    }

    i64 result = gcd(48, 18);

    println(result as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "6\n");
}
