use numerilib::special::Probability;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn combination_test1() {
        let n = 5_f64;
        let k = 2_f64;

        let combination = Probability::combination(n, k);

        assert_approx_eq!(10_f64, combination);
    }

    #[test]
    pub fn combination_test2() {
        let n = 10_f64;
        let k = 3_f64;

        let combination = Probability::combination(n, k);

        assert_approx_eq!(120_f64, combination);
    }

    #[test]
    pub fn combination_test3() {
        let n = 15_f64;
        let k = 4_f64;

        let combination = Probability::combination(n, k);

        assert_approx_eq!(1365_f64, combination);
    }

    #[test]
    pub fn combination_test4() {
        let n = 20_f64;
        let k = 5_f64;

        let combination = Probability::combination(n, k);

        assert_approx_eq!(15504_f64, combination);
    }
}
