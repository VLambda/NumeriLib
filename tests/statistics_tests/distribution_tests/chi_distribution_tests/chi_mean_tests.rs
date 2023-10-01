use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_mean_test1() {
        let df = 5.0;

        let mean = Chi::mean(df);

        assert_approx_eq!(2.1276921621376355, mean);
    }

    #[test]
    fn chi_mean_test2() {
        let df = 7.0;

        let mean = Chi::mean(df);

        assert_approx_eq!(2.5532305945667493, mean);
    }

    #[test]
    fn chi_mean_test3() {
        let df = 9.0;

        let mean = Chi::mean(df);

        assert_approx_eq!(2.9179778223632513, mean);
    }

    #[test]
    fn chi_mean_test4() {
        let df = 11.0;

        let mean = Chi::mean(df);

        assert_approx_eq!(3.2421975804045124, mean);
    }
}
