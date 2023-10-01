use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_pdf_test1() {
        let d1 = 4.0;
        let d2 = 8.0;
        let x = 0.5;

        let pdf = Fisher::pdf(x, d1, d2);

        assert_approx_eq!(0.655360000005586, pdf);
    }

    #[test]
    pub fn fisher_pdf_test2() {
        let d1 = 8.0;
        let d2 = 4.0;
        let x = 0.5;

        let pdf = Fisher::pdf(x, d1, d2);

        assert_approx_eq!(0.625, pdf);
    }

    #[test]
    pub fn fisher_pdf_test3() {
        let d1 = 7.0;
        let d2 = 10.0;
        let x = 0.5;

        let pdf = Fisher::pdf(x, d1, d2);

        assert_approx_eq!(0.6963662983253275, pdf);
    }

    #[test]
    pub fn fisher_pdf_test4() {
        let d1 = 16.0;
        let d2 = 2.0;
        let x = 0.5;

        let pdf = Fisher::pdf(x, d1, d2);

        assert_approx_eq!(0.536870912002969, pdf);
    }
}
