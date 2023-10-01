use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_mean_test1() {
        let d2 = 8.0;

        let mean = Fisher::mean(d2);

        assert_approx_eq!(4_f64 / 3_f64, mean);
    }

    #[test]
    pub fn fisher_mean_test2() {
        let d2 = 4.0;

        let mean = Fisher::mean(d2);

        assert_approx_eq!(2_f64, mean);
    }

    #[test]
    pub fn fisher_mean_test3() {
        let d2 = 10.0;

        let mean = Fisher::mean(d2);

        assert_approx_eq!(1.25, mean);
    }

    #[test]
    pub fn fisher_mean_test4() {
        let d2 = 76.0;

        let mean = Fisher::mean(d2);

        assert_eq!(1.027027027027027, mean);
    }
}
