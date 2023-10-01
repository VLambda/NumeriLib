use numerilib::special::Probability;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn permutation_test1() {
        let n = 5_f64;
        let k = 2_f64;

        let permutation = Probability::permutation(n, k);

        assert_approx_eq!(20_f64, permutation);
    }

    #[test]
    pub fn permutation_test2() {
        let n = 10_f64;
        let k = 3_f64;

        let permutation = Probability::permutation(n, k);

        assert_approx_eq!(720_f64, permutation);
    }

    #[test]
    pub fn permutation_test3() {
        let n = 15_f64;
        let k = 4_f64;

        let permutation = Probability::permutation(n, k);

        assert_approx_eq!(32760_f64, permutation);
    }

    #[test]
    pub fn permutation_test4() {
        let n = 20_f64;
        let k = 5_f64;

        let permutation = Probability::permutation(n, k);

        assert_approx_eq!(1860480_f64, permutation);
    }
}
