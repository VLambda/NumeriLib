use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_pdf_test1() {
        let df = 5_f64;
        let x = 2_f64;

        let pdf = Chi::pdf(x, df);

        assert_approx_eq!(0.5759036428144354, pdf);
    }

    #[test]
    fn chi_pdf_test2() {
        let df = 1_f64;
        let x = 3_f64;

        let pdf = Chi::pdf(x, df);

        assert_approx_eq!(0.008863696823913959, pdf);
    }

    #[test]
    fn chi_pdf_test3() {
        let df = 2_f64;
        let x = 4_f64;

        let pdf = Chi::pdf(x, df);

        assert_approx_eq!(0.0013418505116170687, pdf);
    }

    #[test]
    fn chi_pdf_test4() {
        let df = 3_f64;
        let x = 5_f64;

        let pdf = Chi::pdf(x, df);

        assert_approx_eq!(7.433597573730466e-5, pdf);
    }
}
