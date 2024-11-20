fn factorial(num: u64) -> u64 {
    // n! = n x (n-1) * (n-2) * (n-3) * 3 * 2 * 1
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion

    let mut sum = 1;

    for curr in 1..num {
        println!("current sum: {sum}");
        println!("calculating: {sum} * ({curr} + 1)");

        sum = sum * (curr + 1);
    }

    sum
}

fn main() {
    // You can optionally experiment here.
    let factorial = factorial(5);

    println!("factorial: {factorial}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
