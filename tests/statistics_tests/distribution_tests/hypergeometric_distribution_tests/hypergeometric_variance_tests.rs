use numerilib::stats::distr::Hypergeometric;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn hypergeometric_variance_test1() {
        let population = 15_f64;
        let success = 10_f64;
        let draws = 5_f64;

        let variance = Hypergeometric::variance(population, success, draws);

        assert_approx_eq!(0.7936507936507935, variance);
    }

    #[test]
    fn hypergeometric_variance_test2() {
        let population = 70_f64;
        let success = 43_f64;
        let draws = 9_f64;

        let variance = Hypergeometric::variance(population, success, draws);

        assert_approx_eq!(1.8852085181898848, variance);
    }

    #[test]
    fn hypergeometric_variance_test3() {
        let population = 101_f64;
        let success = 73_f64;
        let draws = 17_f64;

        let variance = Hypergeometric::variance(population, success, draws);

        assert_approx_eq!(2.8613194784825016, variance);
    }

    #[test]
    fn hypergeometric_variance_test4() {
        let population = 13_f64;
        let success = 3_f64;
        let draws = 12_f64;

        let variance = Hypergeometric::variance(population, success, draws);

        assert_approx_eq!(0.1775147928994083, variance);
    }
}
