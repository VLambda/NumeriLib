# Definite Integration

- Functions for Definite Integration in Mathematica

***

### Right Endpoint Riemann Sum:

The Right Endpoint method to calculate a definite integral. The Riemann Sum is an approximation of the area under a curve using rectangles. The Right Endpoint method uses the right endpoint of each rectangle (interval) to calculate the height of the rectangle. The height of the rectangle is then multiplied by the width of the rectangle to calculate the area of the rectangle. The area of each rectangle is then summed to calculate the area under the curve.

#### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `intervals`: The number of the intervals for the Riemann sum.

#### Returns:

The calculated definite integral using the Right Endpoint method.

#### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx\approx \Delta x\sum_{i=1}^{n}f\left( a + x_{i}\Delta x \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

#### Example:
```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let intervals = 6_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

    println!("The Integral of x^2 at [0,6] using Right Endpoints is: {}", integral)
}
```

Execute as:

```,norun,rust
>>> The Integral of x^2 at [0,6] using Right Endpoints is: 91
```

### Left Endpoint Riemann Sum:

The Left Endpoint method to calculate a definite integral. The Riemann Sum is an approximation of the area under a curve using rectangles. The Left Endpoint method uses the left endpoint of each rectangle (interval) to calculate the height of the rectangle. The height of the rectangle is then multiplied by the width of the rectangle to calculate the area of the rectangle. The area of each rectangle is then summed to calculate the area under the curve.

#### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `intervals`: The number of the intervals for the Riemann sum.

#### Returns:

The calculated definite integral using the Left Endpoint method.

#### Equation:
\\[ \int_{a}^{b}f\left( x \right)dx\approx \Delta x\sum_{i=0}^{n-1}f\left( a + x_{i}\Delta x \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

#### Example:

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let intervals = 6_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

    println!("The Integral of x^2 at [0,6] using Left Endpoints is: {}", integral)
}
```

Execute as:

```,norun,rust
>>> The Integral of x^2 at [0,6] using Left Endpoints is: 55
```

### Midpoint Riemann Sum:

The Midpoint method to calculate a definite integral. The Riemann Sum is an approximation of the area under a curve using rectangles. The Midpoint method uses the midpoint of each rectangle (interval) to calculate the height of the rectangle. The height of the rectangle is then multiplied by the width of the rectangle to calculate the area of the rectangle. The area of each rectangle is then summed to calculate the area under the curve.

#### Parameters

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `intervals`: The number of the intervals for the Riemann sum.

#### Returns

The calculated definite integral using the Midpoint method.

#### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx\approx \Delta x\sum_{i=0}^{n}f\left( a + x_{i + \frac{1}{2}}\Delta x^{2} \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

#### Example

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let intervals = 6_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

    println!("The Integral of x^2 at [0,6] using Midpoints is: {}", integral)
}
```

Execute as:

```,norun,rust
>>> The Integral of x^2 at [0,6] using Midpoints is: 71.5
```

### Trapezoid Rule:

The Trapezoid method to calculate a definite integral. The Trapezoid Rule is an approximation of the area under a curve using trapezoids. The Trapezoid Rule uses the left and right endpoints of each interval to calculate the height of the trapezoid. The height of the trapezoid is then multiplied by the width of the trapezoid to calculate the area of the trapezoid. The area of each trapezoid is then summed to calculate the area under the curve.

#### Parameters

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `intervals`: The number of the intervals for the Riemann sum.

#### Returns

The calculated definite integral using the Trapezoidal rule.

#### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx \approx \frac{1}{2}\Delta x\sum_{i=0}^{n-1}f\left( a + x_{i} \Delta x \right) + f\left( a + x_{i+1} \Delta x \right) \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

#### Example

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let intervals = 6_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

    println!("The Integral of x^2 at [0,6] using the Trapezoid Rule is: {}", integral)
}
```

Execute as:

```,norun,rust
>>> The Integral of x^2 at [0,6] using the Trapezoid Rule is: 73
```

### Simpson's Rule:

Uses the Composite Simpson's 1/3rd Rule to calculate a definite integral. Simpson's Rule is an approximation of the area under a curve using parabolas. Simpson's Rule uses the left and right endpoints of each interval to calculate the height of the parabola. The height of the parabola is then multiplied by the width of the parabola to calculate the area of the parabola. The area of each parabola is then summed to calculate the area under the curve.

#### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `intervals`: The number of the intervals for the Riemann sum.

#### Returns:

The calculated definite integral using Simpson's Rule.

#### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx \approx \frac{1}{3} \Delta x \left[ f\left( a \right) + 4 \sum_{i=0}^{\frac{n}{2}} f\left(a + x_{2i-1} \frac{\Delta x}{2} \right) + 2 \sum_{i=1}^{\frac{n}{2} - 1} f\left( a + x_{2i} \frac{\Delta x}{2} \right) + f\left( b \right) \right] \\]
\\[ \Delta x=\frac{b-a}{\underbrace{n}_\text{Intervals}} \\]

#### Example:

```,norun,rust
use mathematica::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let intervals = 6_f64;
    let function = |x: f64| x.powi(2);

    let integral = Functions::simpson(function, lower_bound, upper_bound, intervals);

    println!("The Integral of x^2 at [0,6] using Simpson's Rule is: {}", integral)
}
```

Execute as:

```,norun,rust
>>> The Integral of x^2 at [0,6] using Simpson's Rule is: 72
```

### Boole's Rule:

Uses Boole's Rule to calculate a definite integral. Boole's Rule is an approximation of the area under a curve using quartic polynomials. Boole's Rule uses the left and right endpoints of each interval to calculate the height of the quartic polynomial. The height of the quartic polynomial is then multiplied by the width of the quartic polynomial to calculate the area of the quartic polynomial. The area of each quartic polynomial is then summed to calculate the area under the curve.

#### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.

#### Returns:

The calculated definite integral using Boole's Rule.

#### Equation:

\\[ \int_{a}^{b}f\left( x \right)dx \to \int_{x_{0}}^{x_{4}}f\left( x \right)dx \\]

\\[ \Delta x = \frac{b-a}{4} \\]

<style>
table {
  overflow: hidden;
}
</style>

|            Intervals             |
|:--------------------------------:|
|        \\( x_{0} = a \\)         |
|  \\( x_{1} = a + {\Delta x} \\)  |
| \\( x_{2} = a + {2 \Delta x} \\) |
| \\( x_{3} = a + {3 \Delta x} \\) |
|        \\( x_{4} = b \\)         |

\\[ \int_{x_{0}}^{x_{4}}f\left( x \right)dx\approx \frac{2\Delta x}{45}\left[ 7f\left( x_{0} \right) + 32f\left( x_{1} \right) + 12f\left( x_2 \right) + 32f\left( x_{3} \right) + 7f\left( x_{4} \right) \right] \\]

#### Example:

```,norun,rust
use numerilib::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let function = |x: f64| x.powi(2);
    
    let integral = Functions::boole_rule(function, lower_bound, upper_bound);
    
    println!("The Integral of x^2 at [0,6] using Boole's Rule is: {}", integral)
}
```

Execute as:

```,norun,rust
>>> The Integral of x^2 at [0,6] using Boole's Rule is: 72
```

### Adaptive Quadrature:

Uses Adaptive Quadrature to calculate a definite integral. Adaptive Quadrature is an approximation of the area under a curve using Simpson's Rule. Adaptive Quadrature uses Simpson's Rule to calculate the area under the curve. If the error is greater than the tolerance, the interval is split in half and the area under the curve is calculated again. This process is repeated until the error is less than the tolerance.

#### Parameters:

- `function`: A function that takes a single `f64` argument and returns an `f64`. This is the function to be integrated.
- `lower_limit`: The lower limit of integration.
- `upper_limit`: The upper limit of integration.
- `tolerance`: The level of precision (ie: `1e-6`) that is passed.

#### Returns:

The calculated definite integral using Adaptive Quadrature.

#### Procedure:

Adaptive Quadrature is Described as the following:

```norun,rust
1. Integrate the function using Simpson's Rule.
   I = ∫[a,b]f(x)dx
   c = (a + b) / 2
   Q = ∫[a,c]f(x)dx 
   R = ∫[c,b]f(x)dx
   I ≈ Q + R

2. If the error is greater than the tolerance
   split the interval in half and integrate again.
   
   ε = |Q - R|
   If ε > tolerance, then:
   
    a. Split the intervals in half.
       d = (a + c) / 2
       e = (c + b) / 2
   
    b. Integrate the function using Simpson's Rule.
         M1 = ∫[a,d]f(x)dx + ∫[d,b]f(x)dx
         M2 = ∫[a,e]f(x)dx + ∫[e,b]f(x)dx
         I ≈ M1 + M2
   
3. Repeat until the error is less than the tolerance.
   In which, return the value.
```

#### Example:

```,norun,rust
use numerilib::Functions;

fn main() {
    let lower_bound = 0_f64;
    let upper_bound = 6_f64;
    let tolerance = 1e-12;
    let function = |x: f64| x.powi(2);
    
    let integral = Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerance);
    
    println!("The Integral of x^2 at [0,6] using Adaptive Quadrature is: {}", integral)
}
```

Execute as:

```,norun,rust
>>> The Integral of x^2 at [0,6] using Adaptive Quadrature is: 72
```