use numerilib::stats::distr::Hypergeometric;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn hypergeometric_pmf_test1() {
        let population = 15_f64;
        let success = 10_f64;
        let draws = 5_f64;
        let observed = 3_f64;

        let pmf = Hypergeometric::pmf(population, success, draws, observed);

        assert_approx_eq!(0.3996003996003996, pmf);
    }

    #[test]
    fn hypergeometric_pmf_test2() {
        let population = 70_f64;
        let success = 43_f64;
        let draws = 9_f64;
        let observed = 7_f64;

        let pmf = Hypergeometric::pmf(population, success, draws, observed);

        assert_approx_eq!(0.17392050323034178, pmf);
    }

    #[test]
    fn hypergeometric_pmf_test3() {
        let population = 101_f64;
        let success = 73_f64;
        let draws = 17_f64;
        let observed = 9_f64;

        let pmf = Hypergeometric::pmf(population, success, draws, observed);

        assert_approx_eq!(0.03773652902263438, pmf);
    }

    #[test]
    fn hypergeometric_pmf_test4() {
        let population = 13_f64;
        let success = 3_f64;
        let draws = 12_f64;
        let observed = 1_f64;

        let pmf = Hypergeometric::pmf(population, success, draws, observed);

        assert_approx_eq!(0_f64, pmf);
    }
}
