use numerilib::stats::distr::Cauchy;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn cauchy_pdf_test1() {
        let location = 0_f64;
        let scale = 1_f64;
        let x = 2.5;

        let pdf = Cauchy::pdf(location, scale, x);

        assert_approx_eq!(0.05092958178940651, pdf);
    }

    #[test]
    pub fn cauchy_pdf_test2() {
        let location = 2_f64;
        let scale = 4_f64;
        let x = 2.5;

        let pdf = Cauchy::pdf(location, scale, x);

        assert_approx_eq!(5.092958178940651, pdf);
    }

    #[test]
    pub fn cauchy_pdf_test3() {
        let location = 3_f64;
        let scale = 1_f64;
        let x = 2.5;

        let pdf = Cauchy::pdf(location, scale, x);

        assert_approx_eq!(1.2732395447351628, pdf);
    }

    #[test]
    pub fn cauchy_pdf_test4() {
        let location = 9_f64;
        let scale = 5_f64;
        let x = 8_f64;

        let pdf = Cauchy::pdf(location, scale, x);

        assert_approx_eq!(1.591549430918953, pdf);
    }
}
