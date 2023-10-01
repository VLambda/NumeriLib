use numerilib::special::Probability;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn factorials_test1() {
        let n = 5_f64;

        let factorial = Probability::factorial(n);

        assert_approx_eq!(120_f64, factorial);
    }

    #[test]
    pub fn factorials_test2() {
        let n = 10_f64;

        let factorial = Probability::factorial(n);

        assert_approx_eq!(3628800_f64, factorial);
    }

    #[test]
    pub fn factorials_test3() {
        let n = 7.6_f64;

        let factorial = Probability::factorial(n);

        assert_approx_eq!(17290.248509297868, factorial);
    }

    #[test]
    pub fn factorials_test4() {
        let n = 1.5_f64;

        let factorial = Probability::factorial(n);

        assert_approx_eq!(1.3293403881627572, factorial);
    }
}
