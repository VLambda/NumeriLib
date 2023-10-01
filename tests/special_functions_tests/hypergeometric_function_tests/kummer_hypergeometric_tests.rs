use numerilib::special::Hypergeometric;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn kummer_hypergeometric_function_test1() {
        let a = 1_f64;
        let b = 2_f64;
        let x = 0.5_f64;

        let hypergeometric = Hypergeometric::kummer(a, b, x);

        assert_approx_eq!(1.2974425413997148, hypergeometric);
    }

    #[test]
    pub fn kummer_hypergeometric_function_test2() {
        let a = 5_f64;
        let b = 6_f64;
        let x = 0.1_f64;

        let hypergeometric = Hypergeometric::kummer(a, b, x);

        assert_approx_eq!(1.08701128569255, hypergeometric);
    }

    #[test]
    pub fn kummer_hypergeometric_function_test3() {
        let a = 8_f64;
        let b = 9_f64;
        let x = 0.2_f64;

        let hypergeometric = Hypergeometric::kummer(a, b, x);

        assert_approx_eq!(1.1947936123980913, hypergeometric);
    }

    #[test]
    pub fn kummer_hypergeometric_function_test4() {
        let a = 70_f64;
        let b = 40_f64;
        let x = std::f64::consts::LN_2;

        let hypergeometric = Hypergeometric::kummer(a, b, x);

        assert_approx_eq!(3.3384995279960794, hypergeometric);
    }
}
