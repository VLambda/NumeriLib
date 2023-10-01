use crate::extra::Extra;
use crate::func::integration::Integration;

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
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let function = |x: f64| x.powi(2);
    ///     let x = 2;
    ///
    ///     let derivative = Functions::derivative(function, x);
    ///
    ///     println!("The Derivative of x^2 at x=2 is: {}", derivative);
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
    /// - `intervals`: The number of intervals for the Riemann sum.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Right Endpoint method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let intervals = 6_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn right_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        Integration::right_riemann(function, lower_limit, upper_limit, intervals)
    }

    ///The Left Endpoint method to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `intervals`: The number of intervals for the Riemann sum.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Left Endpoint method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let intervals = 6_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn left_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        Integration::left_riemann(function, lower_limit, upper_limit, intervals)
    }

    /// The Midpoint method to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `intervals`: The number of intervals for the Riemann sum.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Midpoint method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let intervals = 6_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    ///
    pub fn midpoint_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        Integration::midpoint_riemann(function, lower_limit, upper_limit, intervals)
    }

    /// The Trapezoid method to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `intervals`: The number of intervals for the Trapezoidal rule.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using the Trapezoidal rule.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let intervals = 6_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn trapezoid<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        Integration::trapezoid(function, lower_limit, upper_limit, intervals)
    }

    /// Uses the Composite Simpson's 1/3rd Rule to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `intervals`: The number of intervals for the Simpson's Rule.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using Simpson's Rule.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let intervals = 6_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::simpson(function, lower_bound, upper_bound, intervals);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn simpson<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        Integration::simpson(function, lower_limit, upper_limit, intervals)
    }

    /// Uses Boole's Rule to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using Boole's Rule.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::boole_rule(function, lower_bound, upper_bound);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn boole_rule<F: Fn(f64) -> f64>(function: F, lower_limit: f64, upper_limit: f64) -> f64 {
        Integration::boole_rule(function, lower_limit, upper_limit)
    }

    /// Uses Adaptive Quadrature to calculate a definite integral.
    ///
    /// # Parameters
    ///
    /// - `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
    /// - `lower_limit`: The lower limit of integration.
    /// - `upper_limit`: The upper limit of integration.
    /// - `tolerance`: The level of precision (ie: `1e-6`) that is passed.
    ///
    /// # Returns
    ///
    /// The calculated definite integral using Adaptive Quadrature.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let tolerance = 1e-12;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerance);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn adaptive_quadrature<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        tolerance: f64,
    ) -> f64 {
        Integration::adaptive_quadrature(function, lower_limit, upper_limit, tolerance)
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
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let start = 0_f64;
    ///     let limit = 9_f64;
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
    /// use numerilib::Functions;
    ///
    /// fn main() {
    ///     let start = 4.5;
    ///     let limit = 100_f64;
    ///     let function = |x: f64| 1_f64 / x;
    ///
    ///     let summation = Functions::summation(start, limit, function);
    ///
    ///     println!("The summation of the function 1/x from [4.5,100] is: {}", summation);
    /// }
    /// ```
    /// <hr/>
    pub fn summation<F: Fn(f64) -> f64>(start: f64, limit: f64, f: F) -> f64 {
        let mut result = 0.0;
        let mut x = start;

        while x <= limit {
            result += f(x);
            x += 1.0; // You can adjust the step size if needed
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
    /// use numerilib::Functions;
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
    /// use numerilib::Functions;
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
        let mut x = start;

        while x <= limit {
            result *= f(x);
            x += 1.0; // You can adjust the step size if needed
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
    /// use numerilib::Functions;
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
    pub fn newmet<F: Fn(f64) -> f64>(mut guess: f64, func: F) -> f64 {
        let mut iteration = 0;
        while iteration <= 200 {
            let value = func(guess);
            let derivative_value = Functions::derivative(&func, guess);

            if derivative_value.abs() < Extra::EPSILON2 || value.abs() < Extra::EPSILON2 {
                break;
            }

            guess -= value / derivative_value;
            iteration += 1;
        }

        guess
    }
}
