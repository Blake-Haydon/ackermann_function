/// Ackermann Function as defined in [Wikipedia](https://en.wikipedia.org/wiki/Ackermann_function).
/// The function is only valid for non-negative integers.
fn ackermann(m: u32, n: u32) -> u32 {
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) => ackermann(m - 1, 1),
        (m, n) => ackermann(m - 1, ackermann(m, n - 1)),
    }
}

fn main() {
    // Define m and n vars for the ackermann function
    let m = 2;
    let n = 3;

    // Time how long it takes to calculate the ackermann function with (4, 1)
    let start = std::time::Instant::now();
    let result = ackermann(m, n);
    let end = start.elapsed();
    let millis = end.as_millis();

    // Print result
    println!("ackermann(m={m}, n={n}) is equal to {result} in {millis} ms");
}

// Unit tests
#[cfg(test)]
mod test {
    use super::ackermann;

    #[test]
    /// Tests for when m = 0.
    fn m_0() {
        for n in 0..5 {
            assert_eq!(ackermann(0, n), n + 1);
        }
    }

    #[test]
    /// Tests for when m = 1.
    fn m_1() {
        for n in 0..5 {
            assert_eq!(ackermann(1, n), n + 2);
        }
    }

    #[test]
    /// Tests for when m = 2.
    fn m_2() {
        for n in 0..5 {
            assert_eq!(ackermann(2, n), 2 * n + 3);
        }
    }

    #[test]
    /// Tests for when m = 3.
    fn m_3() {
        for n in 0..5 {
            assert_eq!(ackermann(3, n), 2u32.pow(n + 3) - 3);
        }
    }

    #[test]
    /// Tests for when m = 4.
    fn m_4() {
        assert_eq!(ackermann(4, 0), 13);
        assert_eq!(ackermann(4, 1), 65533);
    }

    #[test]
    /// Tests for when m = 5.
    fn m_5() {
        assert_eq!(ackermann(5, 0), 65533);
    }
}
