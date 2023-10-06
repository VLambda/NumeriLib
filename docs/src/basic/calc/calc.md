# Calculus/Basic Functions

- Some Basic Functions in Mathematica

***

### Derivatives:

Calculates the Derivative of a function at a given point: using the limit definition of a derivative. The Derivative of a function is the slope of the tangent line to the function at a given point.

#### Parameters:

- `func`: A function that takes a single `f64` argument and returns an `f64`. This is the function for which the derivative will be calculated.
- `x`: The point at which the derivative will be calculated.

#### Returns:

The calculated derivative of the function at the given point.

#### Equation:

\\[ f'\left( x \right)=\lim_{h \to 0} \frac{f\left( x+h \right)-f\left( x \right)}{h} \\]

#### Example:

```,norun,rust                                                 
use mathematica::Functions;

fn main() {
    let function = |x: f64| x.powi(2);
    let x = 2;

    let derivative = Functions::derivative(function, x);

    println!("The Derivative of x^2 as x=2 is: {}", derivative);
}                     
```

Execute as:

```,norun,rust
>>> The Derivative of x^2 as x=2 is: 4
```

### Summation:

Calculates Summations of a function. The Summation of a function is the sum of all the values of the function from a given start to a given limit.

#### Parameters:

- `start`: The starting value of the summation.
- `limit`: The ending value of the summation.
- `func`: A function that takes a single `f64` argument and returns an `f64`. This is the function for which the summation will be calculated.

#### Returns:

The summation of the function from the given start to the given limit.

#### Equation:

\\[ \sum_{i=m}^{n}a_{i}=a_{m}+a_{m+1}+a_{m+2}+\cdot \cdot \cdot +a_{n-1}+a_{n} \\]

#### Example #1: Constant

```,norun,rust                                                          
use ferrate::special::Functions;                             
                                                             
let start = 0;                                               
let limit = 9;                                               
let function = |x: f64| 3_f64;                               
                                                             
let summation = Functions::summation(start, limit, function);
                                                             
println!("The Summation of the constant, 3 from 0 to 9 is: {}", summation);
```         

Execute as:

```,norun,rust
>>> The Summation of the constant, 3 from 0 to 9 is: 30
```

#### Example #2: Function

```,norun,rust                                                          
use ferrate::special::Functions;                             
                                                             
let start = 4.5;                                             
let limit = 100;                                             
let function = |x: f64| 1_f64 / x;                           
                                                             
let summation = Functions::summation(start, limit, function);
                                                             
println!("The Summation of 1/x from 4.5 to 100 is: {}", summation);
```

Execute as:

```,norun,rust
>>> The Summation of 1/x from 4.5 to 100 is: 3.104044184318839
```

### Product:
Calculates the Product (a.k.a. Capital Pi Notation) of a function. The Product of a function is the product of all the values of the function from a given start to a given limit.

#### Parameters:

- `start`: The starting value of the product.
- `limit`: The ending value of the product.
- `func`: A function that takes a single `f64` argument and returns an `f64`. This is the function for which the product will be calculated.

#### Returns:

The product of the function from the given start to the given limit.

#### Equation:

\\[ \prod_{i=m}^{n}a_{i}=a_{m}\times a_{m+1}\times a_{m+2}\times \cdot \cdot \cdot \times a_{n-1}\times a_{n} \\]

#### Example #1: Constant

```,norun,rust                                                          
use ferrate::special::Functions;                             

fn main() {                                                             
    let start = 2_f64;                                           
    let limit = 7_f64;                                           
    let f = |x: f64| 3_f64;                                      
                                                                 
    let product_series = Functions::product(start, limit, f);    
    
    println!("The Product of the constant, 3 from 2 to 7 is: {}", product_series);
}
```  

Execute as:

```,norun,rust
>>> The Product of the constant, 3 from 2 to 7 is: 2187
```

#### Example #2: Function

```,norun,rust                                                          
use ferrate::special::Functions;                             
     
fn main() {                                                        
    let start = 1_f64;                                           
    let limit = 6_f64;                                           
    let f = |x: f64| x.powi(2);                                  
                                                                 
    let product_series = Functions::product(start, limit, f);    
    
    println!("The Product of x^2 from 1 to 6 is: {}", product_series);
}
```    

Execute as:

```,norun,rust
>>> The Product of x^2 from 1 to 6 is: 518400
```

### Newton's Method:

Calculates the root of a function using Newton's Method. Newton's Method is an iterative method for finding the roots of a function. It is based on the idea that a continuous and differentiable function can be approximated by a straight line tangent to it.

#### Parameters:

- `x`: The initial guess for the root of the function.
- `func`: A function that takes a single `f64` argument and returns an `f64`. This is the function for which the root will be calculated.

#### Returns:

The root of the function.

#### Equation:

\\[ x_{n+1}=x_{n}-\frac{f\left( x_{n} \right)}{f'\left( x_{n} \right)} \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Functions;             

fn main() {                                   
    let x = 1.5_f64;                             
    let function = |x: f64| x.powi(2) - 2_f64;   
    let newton = Functions::newmet(x, function); 
                                                 
    println!("The Root of x^2 - 2 is: {}", newton);
}                     
```

Execute as:

```,norun,rust
>>> The Root of x^2 - 2 is: 1.414213562373095
```