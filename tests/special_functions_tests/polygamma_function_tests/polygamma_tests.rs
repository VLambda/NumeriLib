use numerilib::special::Polygamma;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn digamma_function_test1() {
        let x = 0.5_f64;

        let digamma = Polygamma::digamma(x);

        assert_approx_eq!(-1.9635100260214235, digamma);
    }

    #[test]
    pub fn digamma_function_test2() {
        let x = 0.1_f64;

        let digamma = Polygamma::digamma(x);

        assert_approx_eq!(-10.423744443973368, digamma);
    }

    #[test]
    pub fn digamma_function_test3() {
        let x = 0.2_f64;

        let digamma = Polygamma::digamma(x);

        assert_approx_eq!(-5.289037189298927, digamma);
    }

    #[test]
    pub fn digamma_function_test4() {
        let x = 0.4_f64;

        let digamma = Polygamma::digamma(x);

        assert_approx_eq!(-2.561384544585116, digamma);
    }
}
