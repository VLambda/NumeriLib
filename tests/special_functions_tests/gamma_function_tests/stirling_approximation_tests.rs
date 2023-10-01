use numerilib::special::Gamma;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn stirling_approx_function_test1() {
        let x = 1_f64;

        let gamma = Gamma::stirling(x);

        assert_approx_eq!(0.9221370088957891, gamma);
    }

    #[test]
    pub fn stirling_approx_function_test2() {
        let x = 2_f64;

        let gamma = Gamma::stirling(x);

        assert_approx_eq!(1.9190043514889832, gamma);
    }

    #[test]
    pub fn stirling_approx_function_test3() {
        let x = 3_f64;

        let gamma = Gamma::stirling(x);

        assert_approx_eq!(5.836209591345864, gamma);
    }

    #[test]
    pub fn stirling_approx_function_test4() {
        let x = 4_f64;

        let gamma = Gamma::stirling(x);

        assert_approx_eq!(23.506175132893294, gamma);
    }
}
