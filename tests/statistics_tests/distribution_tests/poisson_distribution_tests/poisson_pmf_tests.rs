use numerilib::stats::distr::Poisson;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn poisson_pmf_test1() {
        let k = 10_f64;
        let lambda = 8_f64;

        let pmf = Poisson::pmf(k, lambda);

        assert_approx_eq!(0.09926153383153559, pmf);
    }

    #[test]
    fn poisson_pmf_test2() {
        let k = 8_f64;
        let lambda = 10_f64;

        let pmf = Poisson::pmf(k, lambda);

        assert_approx_eq!(0.11259903214901998, pmf);
    }

    #[test]
    fn poisson_pmf_test3() {
        let k = 72_f64;
        let lambda = 6_f64;

        let pmf = Poisson::pmf(k, lambda);

        assert_approx_eq!(4.306527803003941e-51, pmf);
    }

    #[test]
    fn poisson_pmf_test4() {
        let k = 2_f64;
        let lambda = 1_f64;

        let pmf = Poisson::pmf(k, lambda);

        assert_approx_eq!(0.18393972058572117, pmf);
    }
}
