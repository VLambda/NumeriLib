# Polygamma

- The polygamma functions, denoted as (\\( \psi^{n}\left( z \right) \\)), are mathematical functions that are extensions of the digamma function (\\( \psi\left( z \right) \\)), also known as the psi or di-gamma function. The digamma function is the logarithmic derivative of the gamma function (\\( \Gamma\left( z \right) \\)), which is a widely used function in mathematics and has applications in various fields.

***

### Digamma:

Calculates the Digamma function with its series representation. The Digamma function is the logarithmic derivative of the gamma function (\\( \Gamma\left( z \right) \\)).

#### Parameters:

- `z`: The number for which the Digamma function will be calculated.

#### Returns:

The Digamma function of the given number.

#### Equation:

\\[ \psi\left( z \right)=\frac{d }{dz}\Gamma\left( z \right)=\frac{\Gamma'\left( z \right)}{\Gamma\left( z \right)} \\]

#### Example:

```,norun,rust                                      
use ferrate::special::Polygamma;         

fn main() {                               
    let z = 2_f64;                        
                                          
    let digamma = Polygamma::digamma(z);  
                                          
    println!("The Digamma Function of {} is: {}", z, digamma);
}
```

Executes as:

```,norun,rust
>>> The Digamma Function of 2 is: 0.42278438084235914
```

### Polygamma:

Calculates the Polygamma function with its series representation. The Polygamma function is the (\\( m \\))-th derivative of the Digamma function (\\( \psi\left( z \right) \\)).

#### Parameters:

- `degree`: The degree of the Polygamma function. This is the (\\( m \\)) in (\\( \psi^{m}\left( z \right) \\)).
- `z`: The number for which the Polygamma function will be calculated.

#### Returns:

The Polygamma function of the given number.

#### Equation:

\\[ \psi^{\left( m \right)}\left( z \right):=\frac{d^{m}}{dz^{m}}\psi\left( z \right)=\left( -1 \right)^{\left( m+1 \right)}m!\sum_{k=0}^{\infty }\frac{1}{\left( z+k \right)^{m+1}}\qquad m\gt 1 \\]

\\[ \psi^{-1}\left( z \right)=\int\psi\left( z \right)=\text{ln}\Gamma\left( z \right) \\]

#### Example:

```,norun,rust                                             
use ferrate::special::Polygamma;                
               
fn main() {                                  
    let degree = 0;                                 
    let z = 5_f64;                                  
                                                
    let polygamma = Polygamma::polygamma(degree, z);

    println!("The Polygamma Function of {} at degree {} is: {}", z, degree, polygamma);                                                
}
```

Executes as:

```,norun,rust
>>> The Polygamma Function of 5 at degree 0 is: 1.5061177964312848
```                                             