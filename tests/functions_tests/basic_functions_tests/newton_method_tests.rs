use numerilib::Functions;

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn test1() {
        let function = |x: f64| x.powi(2) - 2_f64;
        let guess = 1.5;
        let root = Functions::newmet(guess, function);

        assert_approx_eq!(1.4142135623730951, root);
    }

    #[test]
    pub fn test2() {
        let function = |x: f64| x.powi(2) - x - 1_f64;
        let guess = -0.5;
        let root = Functions::newmet(guess, function);

        assert_approx_eq!(-0.6180339887498949, root);
    }
}
