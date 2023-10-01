use numerilib::stats::distr::Hypergeometric;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn hypergeometric_kurtosis_test1() {
        let population = 15_f64;
        let success = 10_f64;
        let draws = 5_f64;

        let kurtosis = Hypergeometric::kurtosis(population, success, draws);

        assert_approx_eq!(-0.25384615384615383, kurtosis);
    }

    #[test]
    fn hypergeometric_kurtosis_test2() {
        let population = 70_f64;
        let success = 43_f64;
        let draws = 9_f64;

        let kurtosis = Hypergeometric::kurtosis(population, success, draws);

        assert_approx_eq!(-0.16287437283352507, kurtosis);
    }

    #[test]
    fn hypergeometric_kurtosis_test3() {
        let population = 101_f64;
        let success = 73_f64;
        let draws = 17_f64;

        let kurtosis = Hypergeometric::kurtosis(population, success, draws);

        assert_approx_eq!(-0.07071947390286076, kurtosis);
    }

    #[test]
    fn hypergeometric_kurtosis_test4() {
        let population = 13_f64;
        let success = 3_f64;
        let draws = 12_f64;

        let kurtosis = Hypergeometric::kurtosis(population, success, draws);

        assert_approx_eq!(-0.36666666666666664, kurtosis);
    }
}
