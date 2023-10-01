use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_pdf_test1() {
        let df = 4.0;
        let x = 2.0;

        let pdf = Chi2::pdf(x, df);

        assert_approx_eq!(0.18393972058762567, pdf);
    }

    #[test]
    pub fn chi_sqaured_pdf_test2() {
        let df = 7.0;
        let x = 3.0;

        let pdf = Chi2::pdf(x, df);

        assert_approx_eq!(0.0925081978836606, pdf);
    }

    #[test]
    pub fn chi_sqaured_pdf_test3() {
        let df = 10.0;
        let x = 4.0;

        let pdf = Chi2::pdf(x, df);

        assert_approx_eq!(0.04511176107965131, pdf);
    }

    #[test]
    pub fn chi_sqaured_pdf_test4() {
        let df = 13.0;
        let x = 5.0;

        let pdf = Chi2::pdf(x, df);

        assert_approx_eq!(0.022013261427054242, pdf);
    }
}
