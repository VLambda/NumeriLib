use numerilib::stats::distr::Gaussian;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn gaussian_pdf_test1() {
        let x_value = 0.5;
        let mean = 0.0;
        let sd = 1.0;

        let normalpdf = Gaussian::pdf(x_value, mean, sd);

        assert_approx_eq!(0.3520653267642995, normalpdf);
    }

    #[test]
    pub fn gaussian_pdf_test2() {
        let x_value = 1.96;
        let mean = 0.0;
        let sd = 1.0;

        let normalpdf = Gaussian::pdf(x_value, mean, sd);

        assert_approx_eq!(0.05844094433345148, normalpdf);
    }

    #[test]
    pub fn gaussian_pdf_test3() {
        let x_value = 0.5;
        let mean = 8.0;
        let sd = 1.0;

        let normalpdf = Gaussian::pdf(x_value, mean, sd);

        assert_approx_eq!(2.4343205330290136e-13, normalpdf);
    }

    #[test]
    pub fn gaussian_pdf_test4() {
        let x_value = 0.5;
        let mean = 0.0;
        let sd = 8.0;

        let normalpdf = Gaussian::pdf(x_value, mean, sd);

        assert_approx_eq!(0.04977048208586083, normalpdf);
    }
}
