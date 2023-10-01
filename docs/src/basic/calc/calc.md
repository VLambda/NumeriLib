# Calculus/Basic Functions

- Some Basic Functions in Mathematica

***

## Derivatives:

- Calculates the Derivative of a function at a given point.

### Parameters:

- `f`: A function that takes a single `f64` argument and returns an `f64`. This is the function for which the derivative will be calculated.
- `x`: The point at which the derivative will be calculated.

### Returns:

The calculated derivative of the function at the given point.

### Equation:

\\[ f'\left( x \right)=\lim_{h \to 0} \frac{f\left( x+h \right)-f\left( x \right)}{h} \\]

### Example:

```,norun,rust                                                 
use mathematica::Functions;

fn main() {
    let function = |x: f64| x.powi(2);
    let x = 2;

    let derivative = Functions::derivative(function, x);

    println!("The Derivative of x^2 as x=2 is: {}", derivative);
}                     
```

### Factorials:

- Calculates Factorials with the Definition and the Gamma Function

\\[ a!=a\times \left( a-1 \right)\times \left( a-2 \right)\times \cdot \cdot \cdot \times 1 \\]

#### Example:

```,norun,rust                                     
use ferrate::special::Functions;        
                                        
let n = 6_f64;                          
let factorial = Functions::factorial(n);
assert_eq!(factorial, 720_f64);         
```                                     

### Summation:

- Calculates Summations

\\[ \sum_{i=m}^{n}a_{i}=a_{m}+a_{m+1}+a_{m+2}+\cdot \cdot \cdot +a_{n-1}+a_{n} \\]

#### Example #1: Constant

```,norun,rust                                                          
use ferrate::special::Functions;                             
                                                             
let start = 0;                                               
let limit = 9;                                               
let function = |x: f64| 3_f64;                               
                                                             
let summation = Functions::summation(start, limit, function);
                                                             
assert_eq!(summation, 30_f64);                               
```         

#### Example #2: Function

```,norun,rust                                                          
use ferrate::special::Functions;                             
                                                             
let start = 4.5;                                             
let limit = 100;                                             
let function = |x: f64| 1_f64 / x;                           
                                                             
let summation = Functions::summation(start, limit, function);
                                                             
assert_eq!(summation, 3.1040441843062854);                   
```      

### Product:

- Calculates Product Notation

\\[ \prod_{i=m}^{n}a_{i}=a_{m}\times a_{m+1}\times a_{m+2}\times \cdot \cdot \cdot \times a_{n-1}\times a_{n} \\]

#### Example #1: Constant

```,norun,rust                                                          
use ferrate::special::Functions;                             
                                                             
let start = 2_f64;                                           
let limit = 7_f64;                                           
let f = |x: f64| 3_f64;                                      
                                                             
let product_series = Functions::product(start, limit, f);    
assert_eq!(product_series, 729_f64);                         
```                                                           

#### Example #2: Function

```,norun,rust                                                          
use ferrate::special::Functions;                             
                                                             
let start = 1_f64;                                           
let limit = 6_f64;                                           
let f = |x: f64| x.powi(2);                                  
                                                             
let product_series = Functions::product(start, limit, f);    
assert_eq!(product_series, 518400_f64);                      
```    

### Newton's Method:

- Calculates Roots using Newton's Method

\\[ x_{n+1}=x_{n}-\frac{f\left( x_{n} \right)}{f'\left( x_{n} \right)} \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Functions;             
use ferrate::special::Beta;                  
                                             
let x = 1.5_f64;                             
let function = |x: f64| x.powi(2) - 2_f64;   
let newton = Functions::newmet(x, function); 
                                             
assert_eq!(newton, std::f64::consts::SQRT_2);
```       

### Pochhammer's Function/Rising Factorial:

- Calculates Rising Factorials using its Gamma Definitions

\\[ x^{\overline{n}}=\frac{\Gamma\left( x+n \right)}{\Gamma\left( x \right)} \\]

#### Example:

```,norun,rust                                    
use ferrate::special::Functions;       
                                       
let x = 2_f64;                         
let n = 3_f64;                         
                                       
let poch = Functions::pochhammer(x, n);
                                       
assert_eq!(poch, 24_f64)               
```      

### Falling Factorial:

- Calculates Falling Factorials using its Gamma Definition

\\[ x^{\underline{n}}=\frac{\Gamma\left( x+1 \right)}{\Gamma\left( x-n+1 \right)} \\]

#### Example:

```,norun,rust                                       
use ferrate::special::Functions;          
                                          
let x = 3_f64;                            
let n = 2_f64;                            
                                          
let fall = Functions::fallfactorial(x, n);
                                          
assert_eq!(fall, 6_f64)                   
```                                       