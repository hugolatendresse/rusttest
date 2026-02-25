pub fn fibonacci(n: usize) -> usize {
    fn fib_helper(from: (usize, usize), n: usize) -> usize {
        if n == 0 {
            from.0
        } else {
            fib_helper((from.1, from.0 + from.1), n - 1)
        }
    }
    fib_helper((0, 1), n)
}
