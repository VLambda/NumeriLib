use crate::extra::Extra;
use crate::special::Gamma;

const EPSILON: f64 = 1e-15;

pub struct Functions;

impl Functions {
    /// Uses the definition of a derivative to calculate the derivative of a function at a specific point of a given function. <br>
    /// Learn more about Derivatives and Differentiation at: <a href="https://wikipedia.org/wiki/Derivative" target="_blank">Wikipedia Derivative</a> <br>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use ferrate::Functions;
    ///     
    /// fn main() {
    ///     let function = |x: f64| x.powi(2);
    ///     let x = 2;
    ///
    ///     let derivative = Functions::derivative(function, x);
    ///
    ///     println!("The Derivative of x^2 as x=2 is: {}", derivative);
    /// }
    /// ```
    /// <hr/>
    ///
    pub fn derivative<F: Fn(f64) -> f64>(f: F, x: impl Into<f64> + Copy) -> f64 {
        let h = 1e-7;
        let definition = ((f(x.into() + h)) - f(x.into())) / h;

        if definition.fract().abs() < 1e-7 {
            definition.round()
        } else {
            definition
        }
    }
    /// Uses Simpson's 1/3rd Rule to calculate the integral.<br>
    /// Learn more about Integrals at: <a href="https://wikipedia.org/wiki/Integral" target="_blank">Wikipedia Integral</a> <br>
    /// Learn more about Simpson's 1/3rd Rule at: <a href="https://wikipedia.org/wiki/Simpson's_rule" target="_blank">Wikipedia Simpson's Rule</a> <br>
    /// <hr/>
    ///
    /// # Example
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::integral(lower_bound, upper_bound, function);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    ///
    pub fn integral<F: Fn(f64) -> f64>(lower_bound: f64, upper_bound: f64, f: F) -> f64 {
        let lower = lower_bound;
        let upper = upper_bound;

        let n = 100000;
        let h = (upper - lower) / n as f64;

        let mut sum = f(lower) + f(upper);

        for i in 1..n {
            let x = lower + i as f64 * h;
            sum += if i % 2 == 0 { 2.0 * f(x) } else { 4.0 * f(x) };
        }

        let r1 = (h / 3.0) * sum;

        let approx = Extra::round(r1);
        approx
    }
    /// Calculates a Factorial by using Lanczos's Gamma Function Approximation. <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Factorial" target="_blank">Wikipedia Factorials</a> <br>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let n = 6_f64;
    ///     let factorial = Functions::factorial(n);
    ///     
    ///     println!("6! is: {}", factorial);
    /// }
    /// ```
    /// <hr/>
    ///
    pub fn factorial(n: f64) -> f64 {
        if n.floor() == n {
            (1..=n as u64).map(|i| i as f64).product()
        } else {
            Gamma::lanczos(n + 1_f64)
        }
    }
    /// Summations in Rust. <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Summation" target="_blank">Wikipedia Summation</a> <br>
    /// <hr/>
    ///
    /// # Example #1: Constant
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 0;
    ///     let limit = 9;
    ///     let function = |x: f64| 3_f64;
    ///
    ///     let summation = Functions::summation(start, limit, function);
    ///
    ///     println!("The summation of the constant 3 from [0,9] is: {}", summation);
    /// }
    /// ```
    /// <hr/>
    ///
    /// # Example #2: Function
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 4.5;
    ///     let limit = 100;
    ///     let function = |x: f64| 1_f64 / x;
    ///
    ///     let summation = Functions::summation(start, limit, function);
    ///
    ///     println!("The summation of the function 1/x from [4.5,100] is: {}", summation);
    /// }
    /// ```
    /// <hr/>
    pub fn summation(
        start: impl Into<f64> + Copy,
        limit: impl Into<f64> + Copy,
        f: fn(f64) -> f64,
    ) -> f64 {
        let start = start.into().round() as i64;
        let limit = limit.into().round() as i64;
        let mut result = 0.0;

        for i in start..=limit {
            result += f(i as f64);
        }

        result
    }
    /// Calculates the product of a function. a.k.a Capital Pi Notation. <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Product_(mathematics)#Product_of_a_sequence" target="_blank">Wikipedia Capital Pi Notation</a> <br>
    /// <hr/>
    ///
    /// # Example #1: Constant
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 2_f64;
    ///     let limit = 7_f64;
    ///     let f = |x: f64| 3_f64;
    ///
    ///     let product_series = Functions::product(start, limit, f);
    ///     
    ///     println!("The Product Series of the Constant 3 from [2,7] is: {}", product_series);
    /// }
    /// ```
    /// <hr/>
    ///
    /// # Example #2: Function
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 1_f64;
    ///     let limit = 6_f64;
    ///     let f = |x: f64| x.powi(2);
    ///
    ///     let product_series = Functions::product(start, limit, f);
    ///     
    ///     println!("The product series of the function x^2 from [1,6] is: {}", product_series);
    /// }
    /// ```
    /// <hr/>
    ///
    pub fn product<F: Fn(f64) -> f64>(start: f64, limit: f64, f: F) -> f64 {
        let mut result = 1.0;
        let start = start.round() as i64;
        let limit = limit.round() as i64;

        for i in start..=limit {
            result *= f(i as f64);
        }
        result
    }
    /// A rust implementation of the Newton–Raphson method for finding roots.
    /// Learn more at: <a href="https://wikipedia.org/wiki/Newton's_method" target="_blank">Wikipedia Newton–Raphson method</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let x = 1.5_f64;
    ///     let function = |x: f64| x.powi(2) - 2_f64;
    ///     let newton = Functions::newmet(x, function);
    ///
    ///     println!("Using Newton's Method we can approximate the root of x^2-2 with a guess of 1.5 as: {}", newton);
    /// }
    /// ```
    /// <hr/>
    pub fn newmet<F: Fn(f64) -> f64>(guess: f64, func: F) -> f64 {
        let mut iteration = 0;
        let mut result = guess;
        while iteration < 200 {
            let value = func(result);
            let derivative_value = Functions::derivative(&func, result);

            if derivative_value.abs() < EPSILON {
                break;
            }

            result -= value / derivative_value;

            if value.abs() < EPSILON {
                break;
            }

            iteration += 1;
        }

        result
    }
    /// Pochhammer's Function in Rust <br>
    /// Learn more at: <a href="https://en.wikipedia.org/wiki/Falling_and_rising_factorials" target="_blank">Wikipedia Falling and Rising Factorials</a> <br>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let x = 2_f64;
    ///     let n = 3_f64;
    ///     let poch = Functions::pochhammer(x, n);
    ///
    ///     println!("The Rising Factorial of 2^n n=3 is: {}", poch);
    /// }
    /// ```
    /// <hr/>
    pub fn pochhammer(x: f64, n: f64) -> f64 {
        Gamma::lanczos(x + n) / Gamma::lanczos(x)
    }
    /// Falling Factorials in Rust <br>
    /// Learn more at: <a href="https://en.wikipedia.org/wiki/Falling_and_rising_factorials" target="_blank">Wikipedia Falling and Rising Factorials</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```                                       
    /// use ferrate::Functions;          
    ///
    /// fn main() {                                          
    ///     let x = 3_f64;                            
    ///     let n = 2_f64;                            
    ///                                           
    ///     let fall = Functions::fallfactorial(x, n);   
    ///                                           
    ///     println!("The Falling Factorial of 3^n where n=2 is: {}", fall);
    /// }               
    /// ```                                       
    pub fn fallfactorial(x: f64, n: f64) -> f64 {
        Gamma::lanczos(x + 1_f64) / Gamma::lanczos(x - n + 1_f64)
    }
}
