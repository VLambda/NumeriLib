use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_skewness_test1() {
        let df = 5.0;

        let skewness = Chi::skewness(df);

        assert_approx_eq!(0.3542422252176183, skewness);
    }

    #[test]
    fn chi_skewness_test2() {
        let df = 7.0;

        let skewness = Chi::skewness(df);

        assert_approx_eq!(0.29062187526173416, skewness);
    }

    #[test]
    fn chi_skewness_test3() {
        let df = 9.0;

        let skewness = Chi::skewness(df);

        assert_approx_eq!(0.2518525758595361, skewness);
    }

    #[test]
    fn chi_skewness_test4() {
        let df = 11.0;

        let skewness = Chi::skewness(df);

        assert_approx_eq!(0.22520272403516955, skewness);
    }
}
