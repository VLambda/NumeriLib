use numerilib::special::Probability;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn pochhammer_symbol_test1() {
        let x = 5_f64;
        let n = 2_f64;

        let pochhammer_symbol = Probability::pochhammer(x, n);

        assert_approx_eq!(30_f64, pochhammer_symbol);
    }

    #[test]
    pub fn pochhammer_symbol_test2() {
        let x = 10_f64;
        let n = 3_f64;

        let pochhammer_symbol = Probability::pochhammer(x, n);

        assert_approx_eq!(1320_f64, pochhammer_symbol);
    }

    #[test]
    pub fn pochhammer_symbol_test3() {
        let x = 15_f64;
        let n = 4_f64;

        let pochhammer_symbol = Probability::pochhammer(x, n);

        assert_approx_eq!(73440_f64, pochhammer_symbol);
    }

    #[test]
    pub fn pochhammer_symbol_test4() {
        let x = 20_f64;
        let n = 5_f64;

        let pochhammer_symbol = Probability::pochhammer(x, n);

        assert_approx_eq!(5100480_f64, pochhammer_symbol, 1e-4);
    }
}
