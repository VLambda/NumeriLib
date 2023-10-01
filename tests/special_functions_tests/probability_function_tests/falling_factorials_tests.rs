use numerilib::special::Probability;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn falling_factorial_test1() {
        let x = 5_f64;
        let n = 2_f64;

        let falling_factorial = Probability::falling_factorial(x, n);

        assert_approx_eq!(20_f64, falling_factorial);
    }

    #[test]
    pub fn falling_factorial_test2() {
        let x = 10_f64;
        let n = 3_f64;

        let falling_factorial = Probability::falling_factorial(x, n);

        assert_approx_eq!(720_f64, falling_factorial);
    }

    #[test]
    pub fn falling_factorial_test3() {
        let x = 15_f64;
        let n = 4_f64;

        let falling_factorial = Probability::falling_factorial(x, n);

        assert_approx_eq!(32760_f64, falling_factorial);
    }

    #[test]
    pub fn falling_factorial_test4() {
        let x = 20_f64;
        let n = 5_f64;

        let falling_factorial = Probability::falling_factorial(x, n);

        assert_approx_eq!(1860480_f64, falling_factorial, 1e-4);
    }
}
