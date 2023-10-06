# Gamma Functions

- All the Gamma Functions
***

### Stirling's Approximation:

Uses the Definition of Stirling's Approximation. Stirling's Approximation is an approximation for factorials.

#### Parameters:

- `n`: The number for which the factorial will be approximated via Stirling's Approximation.

#### Returns:

The approximation of the factorial of the given number.

#### Equation:

\\[ n!\sim \sqrt{2\pi n}\left( \frac{n}{e} \right)^{n} \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Gamma;                 

fn main() {                               
    let n = 2_f64;                               
    let stirling = Gamma::stirling(n);           
                                             
    println!("The Stirling Approximation of {}! is: {}", n, stirling);
}
```

Executes as:

```,norun,rust
>>> The Stirling Approximation of 2! is: 1.9190043514880554
```

### Log Gamma Lanczos:

The Log Gamma Function is calculated via the Lanczos Approximation. The Lanczos Approximation is an algorithm uses precomputed coefficients to perform calculations for the Gamma and Log-Gamma Function with fixed precision.

#### Parameters:

- `n`: The number for which the Log Gamma Function will be calculated.

#### Returns:

The Log Gamma Function of the given number.

#### Procedure:

Lanczos Approximation for the Log Gamma Function goes as follows:

```norun,rust
1. The Constants are defined:

const G: f64 = 5f64;

const P: [f64; 7] = [
    1.000000000189712,
    76.18009172948503,
    -86.50532032927205,
    24.01409824118972,
    -1.2317395783752254,
    0.0012086577526594748,
    -0.00000539702438713199,
];

2. A loop calculates for the Final Constant:

let A(z) = P[0] + ∑[k=1; P.len()] P[k] / (z + k)];

3. The Log Gamma Function is calculated:

lnΓ(z) = ln(√(2π)) + ln(A(z)) - (z + G - 0.5) * ln(z + G - 0.5) * (z + 0.5);
```
    
#### Example:

```,norun,rust                                       
use ferrate::special::Gamma;              

fn main() {                                          
    let n = 6_f64;                            
    let lanczos_ln = Gamma::lanczosln(n);     
                                          
    println!("The Log Gamma Function of {} is: {}", n, lanczos_ln);
}
```

Executes as:

```,norun,rust
>>> The Log Gamma Function of 6 is: 4.787491742782046
```

### Gamma Function Lanczos:

The Gamma function is a generalization of the factorial function to non-integer and complex numbers. It is defined for all non-positive integers. The Gamma Function is calculated via the Lanczos Approximation. The Lanczos Approximation is an algorithm uses precomputed coefficients to perform calculations for the Gamma and Log-Gamma Function with fixed precision.

#### Parameters:

- `n`: The number for which the Gamma Function will be calculated.

#### Returns:

The Gamma Function of the given number.

#### Equation:

\\[ \Gamma\left( x \right)=\left( x-1 \right)!\\]
\\[ \Gamma\left( x \right)=\int_{0}^{\infty }t^{x-1}e^{-t}dt\\]
\\[ \Gamma\left( x \right)=e^{ln\Gamma\left( x \right)} \\]

#### Example:

```,norun,rust                             
use ferrate::special::Gamma;    

fn main() {                               
    let n = 6_f64;                               
    let lanczos = Gamma::lanczos(n);           
                                             
    println!("The Gamma Function of {} is: {}", n, lanczos);
}
```                     
Executes as:

```,norun,rust                          
>>> The Gamma Function of 6 is: 120
```

### Incomplete Gamma Function (Lower)

Uses the Series Definition of the Lower Incomplete Gamma Function

#### Parameters:

- `a`: The value for which the Lower Incomplete Gamma Function will be calculated.
- `x`: The integral bound at which the Lower Incomplete Gamma Function will be calculated.

#### Returns:

The Lower Incomplete Gamma Function of the given number.

#### Equation:

\\[ \gamma\left( s,x \right)=\int_{0}^{x }t^{s-1}e^{-t}dt \\]
\\[ \gamma\left( s,x \right)=x^{s}\sum_{k=0}^{\infty }\frac{\left( -1 \right)^{k}x^{k}}{\left( s+k \right)k!} \\]

#### Example:

```,norun,rust                                   
use ferrate::special::Gamma;               
                
fn main() {                                  
    let a = 3_f64;                            
    let x = 1_f64;                            
                                              
    let gamma = Gamma::incgamma(a, x);        
                                              
    println!("The Lower Incomplete Gamma Function of {} and {} is: {}", a, x, gamma);
}
```
Executes as:

```,norun,rust
>>> The Lower Incomplete Gamma Function of 3 and 1 is: 0.16060279414278839
```

### Incomplete Gamma Function (Upper)

Uses the Definition of the Upper Incomplete Gamma Function.

#### Parameters:

- `a`: The value for which the Upper Incomplete Gamma Function will be calculated.
- `x`: The integral bound at which the Upper Incomplete Gamma Function will be calculated.

#### Returns:

The Upper Incomplete Gamma Function of the given number.

#### Equation:

\\[ \Gamma\left( s,x \right)=\int_{x}^{\infty }t^{s-1}e^{-t}dt \\]
\\[ \Gamma\left( s,x \right)+\gamma\left( s,x \right)=\Gamma\left( x \right) \\]
\\[ \Gamma\left( s,x \right)=\Gamma\left( x \right)-\gamma\left( s,x \right) \\]

#### Example:

```,norun,rust                                       
use ferrate::special::Gamma;              
             
fn main() {                                  
    let a = 3_f64;                            
    let x = 1_f64;                            
                                              
    let gamma = Gamma::incgammac(a, x);       
                                              
    println!("The Upper Incomplete Gamma Function of {} and {} is: {}", a, x, gamma);
}
```
Executes as:

```,norun,rust
>>> The Upper Incomplete Gamma Function of 3 and 1 is: 1.8393972058572117
```    

### The Regularized Incomplete Gamma Function

Uses the Definition of the Regularized Incomplete Gamma Function.

#### Parameters:

- `a`: The value for which the Regularized Incomplete Gamma Function will be calculated.
- `x`: The integral bound at which the Regularized Incomplete Gamma Function will be calculated.

#### Returns:

The Regularized Incomplete Gamma Function of the given number.

#### Equation:

\\[ \text{P}\left( s,x \right)=\frac{\gamma\left( s,x \right)}{\Gamma\left( x \right)} \\]

#### Example:

```,norun,rust                                            
use ferrate::special::Gamma;                   
                  
fn main() {                                     
    let a = 3_f64;                               
    let x = 1_f64;                               
                                                 
    let gamma = Gamma::reggamma(a, x);           
                                                 
    println!("The Regularized Incomplete Gamma Function of {} and {} is: {}", a, x, gamma);
}
```

Executes as:

```,norun,rust
>>> The Regularized Incomplete Gamma Function of 3 and 1 is: 0.052653017343711174
```

### CDF for Poisson Random Variables

Uses the Definition of the CDF for Poisson Random Variables.

#### Parameters:

- `a`: The value for which the CDF for Poisson Random Variables will be calculated.
- `x`: The integral bound at which the CDF for Poisson Random Variables will be calculated.

#### Returns:

The CDF for Poisson Random Variables of the given number.

#### Equation:

\\[ \text{Q}\left( s,x \right)=\frac{\Gamma\left( s,x \right)}{\Gamma\left( x \right)} \\]
\\[ \text{Q}\left( s,x \right)=1-\text{P}\left( s,x \right) \\]

#### Example:

```,norun,rust                                           
use ferrate::special::Gamma;                  
                           
fn main() {                                      
    let a = 3_f64;                               
    let x = 1_f64;                               
                                                 
    let gamma = Gamma::reggammac(a, x);          
                                                 
    println!("The CDF for Poisson Random Variables of {} and {} is: {}", a, x, gamma);
}
```

Executes as:

```,norun,rust
>>> The CDF for Poisson Random Variables of 3 and 1 is: 0.9473469826562888
```                                           