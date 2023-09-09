use crate::extra::Extra;
use crate::func::integration::Integration;
use crate::special::Gamma;

/// A module containing Regular Mathematics Functions.
pub struct Functions;

impl Functions {
    /// Uses the definition of a derivative to calculate the derivative of a function at a specific point of a given function.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that takes a single `f64` argument and returns an `f64`. This is the function for which the derivative will be calculated.
    /// - `x`: The point at which the derivative will be calculated.
    ///
    /// # Returns
    ///
    /// The calculated derivative of the function at the given point.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use mathematica::Functions;
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
    pub fn derivative<F: Fn(f64) -> f64>(f: F, x: impl Into<f64> + Copy) -> f64 {
        let h = 1e-7;
        let definition = ((f(x.into() + h)) - f(x.into())) / h;

        if definition.fract().abs() < Extra::EPSILON1 {
            definition.round()
        } else {
            definition
        }
    }

    /// The Right Endpoint method to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `interval_size`: The size of the subintervals for the Riemann sum.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Right Endpoint method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::right_riemann(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn right_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::right_riemann(function, lower_limit, upper_limit, interval_size)
    }

    /// The Left Endpoint method to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `interval_size`: The size of the subintervals for the Riemann sum.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Left Endpoint method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::left_riemann(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn left_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::left_riemann(function, lower_limit, upper_limit, interval_size)
    }

    /// The Midpoint method to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `interval_size`: The size of the subintervals for the Riemann sum.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Midpoint method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    ///
    pub fn midpoint_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::midpoint_riemann(function, lower_limit, upper_limit, interval_size)
    }

    /// The Trapezoid method to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `interval_size`: The size of the subintervals for the Trapezoidal rule.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Trapezoidal rule.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::trapezoid(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn trapezoid<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::trapezoid(function, lower_limit, upper_limit, interval_size)
    }

    /// Uses Simpson's Rule to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `interval_size`: The size of the subintervals for the Simpson's Rule.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using Simpson's Rule.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::simpson(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn simpson<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::simpson(function, lower_limit, upper_limit, interval_size)
    }

    /// Calculates a Factorial by using Lanczos's Gamma Function Approximation.
    ///
    /// # Parameters
    ///
    /// - `n`: The value for which the factorial is calculated.
    ///
    /// # Returns
    ///
    /// The factorial of the given value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
    ///
    /// fn main() {
    ///     let n = 6_f64;
    ///     let factorial = Functions::factorial(n);
    ///
    ///     println!("6! is: {}", factorial);
    /// }
    /// ```
    /// <hr/>
    pub fn factorial(n: f64) -> f64 {
        if n.floor() == n {
            (1..=n as u64).map(|i| i as f64).product()
        } else {
            Gamma::lanczos(n + 1_f64)
        }
    }

    /// Summations in Rust.
    ///
    /// # Parameters
    ///
    /// - `start`: The starting value for the summation.
    /// - `limit`: The ending value for the summation.
    /// - `f`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be summed.
    ///
    /// # Returns
    ///
    /// The sum of applying the function to each value in the range `[start, limit]`.
    ///
    /// # Example #1: Constant
    ///
    /// ```rust
    /// use mathematica::Functions;
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
    ///
    /// # Example #2: Function
    ///
    /// ```rust
    /// use mathematica::Functions;
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

    /// Calculates the product of a function. a.k.a Capital Pi Notation.
    ///
    /// # Parameters
    ///
    /// - `start`: The starting value for the product.
    /// - `limit`: The ending value for the product.
    /// - `f`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be multiplied in the product.
    ///
    /// # Returns
    ///
    /// The product of applying the function to each value in the range `[start, limit]`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
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
    ///
    /// # Example #2: Function
    ///
    /// ```rust
    /// use mathematica::Functions;
    ///
    /// fn main() {
    ///     let start = 3_f64;
    ///     let limit = 7_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let summation = Functions::product(start, limit, function);
    ///
    ///     println!("The Product Series of the Function x^2 from [3,7] is: {}", summation);
    /// }
    /// ```
    /// <hr/>
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
    ///
    /// # Parameters
    ///
    /// - `guess`: An initial guess for the root.
    /// - `func`: A function that takes a single `f64` argument and returns an `f64`. This is the function for which the root is found.
    ///
    /// # Returns
    ///
    /// The approximate root of the function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
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

            if derivative_value.abs() < Extra::EPSILON2 {
                break;
            }

            result -= value / derivative_value;

            if value.abs() < Extra::EPSILON2 {
                break;
            }

            iteration += 1;
        }

        result
    }

    /// Pochhammer's Function in Rust.
    ///
    /// # Parameters
    ///
    /// - `x`: The base value.
    /// - `n`: The exponent value.
    ///
    /// # Returns
    ///
    /// The value of Pochhammer's function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
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

    /// Falling Factorials in Rust.
    ///
    /// # Parameters
    ///
    /// - `x`: The base of the falling factorial.
    /// - `n`: The number of falling factorial terms.
    ///
    /// # Returns
    ///
    /// The value of the falling factorial.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::Functions;
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
    /// <hr/>                                       
    pub fn fallfactorial(x: f64, n: f64) -> f64 {
        Gamma::lanczos(x + 1_f64) / Gamma::lanczos(x - n + 1_f64)
    }
}
