# Definite Integration

- Functions for Definite Integration in Mathematica

***

## Right Endpoint Riemann Sum:
- The Right Endpoint method to calculate a definite integral.

### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `interval_size`: The size of the intervals for the Riemann sum.

### Returns:

The calculated definite integral using the Right Endpoint method.

### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx\approx \Delta x\sum_{i=1}^{n}f\left( a + x_{i}\Delta x \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

### Example:
```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let interval_size = 1_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::right_riemann(function, lower_bound, upper_bound, interval_size);

    println!("The Integral of x^2 at [0,6] is: {}", integral)
}
```

## Left Endpoint Riemann Sum:

- The Left Endpoint method to calculate a definite integral.

### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `interval_size`: The size of the subintervals for the Riemann sum.

### Returns:

The calculated definite integral using the Left Endpoint method.

### Equation:
\\[ \int_{a}^{b}f\left( x \right)dx\approx \Delta x\sum_{i=0}^{n-1}f\left( a + x_{i}\Delta x \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

### Example:

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let interval_size = 1_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::left_riemann(function, lower_bound, upper_bound, interval_size);

    println!("The Integral of x^2 at [0,6] is: {}", integral)
}
```

## Midpoint Riemann Sum:

- The Midpoint method to calculate a definite integral.

### Parameters

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `interval_size`: The size of the subintervals for the Riemann sum.

### Returns

The calculated definite integral using the Midpoint method.

### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx\approx \Delta x\sum_{i=0}^{n}f\left( a + x_{i + \frac{1}{2}}\Delta x^{2} \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

### Example

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let interval_size = 1_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, interval_size);

    println!("The Integral of x^2 at [0,6] is: {}", integral)
}
```

## Trapezoid Rule:

- The Trapezoid method to calculate a definite integral.

### Parameters

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `interval_size`: The size of the subintervals for the Trapezoidal rule.

### Returns

The calculated definite integral using the Trapezoidal rule.

### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx \approx \frac{1}{2}\Delta x\sum_{i=0}^{n-1}f\left( a + x_{i} \Delta x \right) + f\left( a + x_{i+1} \Delta x \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

### Example

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let interval_size = 1_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::trapezoid(function, lower_bound, upper_bound, interval_size);

    println!("The Integral of x^2 at [0,6] is: {}", integral)
}
```

## Simpson's Rule:

- Uses Simpson's Rule to calculate a definite integral.

### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `interval_size`: The size of the subintervals for the Simpson's Rule.

### Returns:

- The calculated definite integral using Simpson's Rule.

### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx \approx \frac{1}{3} \Delta x \left[ f\left( a \right) + 4 \sum_{i=0}^{\frac{n}{2}} f\left(a + x_{2i-1} \frac{\Delta x}{2} \right) + 2 \sum_{i=1}^{\frac{n}{2} - 1} f\left( a + x_{2i} \frac{\Delta x}{2} \right) + f\left( b \right) \right] \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

### Example:

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let interval_size = 1_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::simpson(function, lower_bound, upper_bound, interval_size);

    println!("The Integral of x^2 at [0,6] is: {}", integral)
}
```