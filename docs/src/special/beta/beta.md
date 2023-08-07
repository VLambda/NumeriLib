# Beta Functions

- The beta function is closely related to the gamma function (\\( \Gamma\left( z \right) \\)) and has various applications in mathematics, statistics, and other fields. 

***

## Ferrate Functions:

### Log Beta:

- Uses the defintion of the Log Beta function in terms of the Log Gamma Function

\\[ \text{ln}B\left( z_{1},z_{2} \right)=\text{ln}\Gamma\left( z_{1} \right)+\text{ln}\Gamma\left( z_{2} \right)-\text{ln}\Gamma\left( z_{1}+z_{2} \right) \\]

#### Example:

```,norun,rust                                     
use ferrate::special::Beta;             
                                        
let z1 = 1_f64;                         
let z2 = 2_f64;                         
let lnbeta = Beta::lnbeta(z1, z2);      
                                        
assert_eq!(lnbeta, -0.6931471805616405);
```                                     

### Beta Function:

- Calculates the Beta Function

\\[ B\left( z_{1},z_{2} \right)=\int_{0}^{1}t^{z_{1}-1}\left( 1-t \right)^{z_{2}-1}dt=e^{\text{ln}B\left( z_{1},z_{2} \right)} \\]

#### Example:

```,norun,rust                                    
use ferrate::special::Beta;            
                                       
let z1 = 8_f64;                        
let z2 = 7_f64;                        
                                       
let beta = Beta::beta(z1, z2);         
                                       
assert_eq!(beta, 4.162504162405661e-5);
```

### Incomplete Beta Function:

- Uses the Definition of an Incomplete Beta Function

\\[ I_{x}\left( z_{1},z_{2} \right)=\frac{B_{x}\left( z_{1},z_{2} \right)}{B\left( z_{1},z_{2} \right)} \\]

\\[ B_{x}\left( z_{1},z_{2} \right)={B\left( z_{1},z_{2} \right)}\cdot I_{x}\left( z_{1},z_{2} \right) \\]

#### Example:

```,norun,rust                                        
use ferrate::special::Beta;                
                                           
let x = 1_f64 / 7_f64;                     
let z1 = 0.5_f64;                          
let z2 = 3_f64;                            
                                           
let incbeta = Beta::incbeta(x, z1, z2);    
                                           
assert_eq!(incbeta, 0.6870211373344728_f64)
```

### The Regularized Incomplete Beta Function:

- Uses a Series Expansion of the Incomplete Beta Function

\\[ I_{x}\left( z_{1},z_{2} \right)=\frac{B_{x}\left( z_{1},z_{2} \right)}{B\left( z_{1},z_{2} \right)} \\]
\\[ I_{x}\left( z_{1},z_{2} \right)=\frac{x^{z_{1}}\left( 1-x \right)^{z_{2}}}{z_{1}B\left( z_{1},z_{2} \right)}\left[ 1+\sum_{n=0}^{\infty }\frac{B\left( z_{1}+1,n+1 \right)}{B\left( z_{1}+z_{2},n+1 \right)}x^{n+1} \right] \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Beta;                  
                                             
let x = 1_f64 / 7_f64;                       
let z1 = 1_f64 / 2_f64;                      
let z2 = 3_f64;                              
                                             
let regincbeta = Beta::regincbeta(z1, z2, x);
                                             
assert_eq!(regincbeta, 0.6440823162530317);  
```

### The Inverse of the Regularized Incomplete Beta Function:

- Uses Newton's Method to Calculate the Inverse

\\[ I_{x}^{-1}\left( I_{x}\left( z_{1},z_{2} \right) \right)=1 \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Beta;                  
                                             
let z1 = 1_f64;                              
let z2 = 2_f64;                              
let x = 0.590401_f64;                        
                                             
let inverse = Beta::invregincbeta(z1, z2, x);
                                             
assert_eq!(inverse, 0.3600007812492397_f64); 
```                                          