use nalgebra::{DMatrix};
use std::f64::consts;

/**
    * Activation Functions
    *
    * Activation functions are used to introduce non-linearity in the neural network.
    * The primary purpose of having nonlinear components in the neural network (fNN)
    * is to allow it to approximate nonlinear functions. Without activation functions,
    * fNN will always be linear, no matter how deep it is.
    *
    * The forward activation takes in Z -> the result of transforming an input
    * through some layer. It returns A, the activated version of this.
    *
    * The backwards function takes in dLdA, the derivative of loss with respect to
    * the output of the layer. This signifies however much our loss changes based on
    * change in the output.
    * 
    * By multiplying dLdA with dAdZ, we get dLdZ, the change in loss with respect to 
    * the input. This is then passed to the layer.
    *
    * Currently, the following activation functions are implemented:
    * 1. Identity - f(x) = x
    * 2. ReLU - f(x) = max(0, x)
    * 3. Sigmoid f(z) = 1/(1 + e^-z)
    * 
    *
**/


// Define a trait for Activation Functions
pub trait ActivationFunction {
    fn forward(&mut self, Z: &DMatrix<f64>) -> DMatrix<f64>;
    fn backward(&self, dLdA: &DMatrix<f64>) -> DMatrix<f64>;
}

// Identity Activation Function
pub struct Identity {
    A : DMatrix<f64>
}

impl Identity {
    pub fn new() -> Self {
        Identity {
            A : DMatrix::zeros(0, 0)
        }
    }

    pub fn forward(&mut self, Z : &DMatrix<f64>) -> DMatrix<f64> {
        self.A = Z.clone(); // Identity(Z) = Z
        return self.A.clone();
    }

    pub fn backward(&self, dLdA : &DMatrix<f64>) -> DMatrix<f64> {
        let dLdZ = dLdA; // Derivative of Identity is 1, so dLdZ = dLdA * 1 = dLdA
        return dLdZ.clone();
    }
}

// ReLU Activation Function
pub struct ReLU {
    A : DMatrix<f64>
}

impl ReLU {
    pub fn new() -> Self {
        ReLU {
            A : DMatrix::zeros(0, 0)
        }
    }

    pub fn forward(&mut self, Z : &DMatrix<f64>) -> DMatrix<f64> {
        self.A = Z.map(|x| x.max(0.0)); // ReLU(Z) = max(0, Z)
        return self.A.clone();
    }

    pub fn backward(&self, dLdA : &DMatrix<f64>) -> DMatrix<f64> {
        // Assert that forward pass is  called before backward pass
        // to ensure that self.A is set to the correct value.
        assert!(!self.A.is_empty(), "Forward pass not called before backward pass");

        // Derivative of ReLU is 1 if x > 0, 0 otherwise
        let dAdZ = self.A.map(|x| if x > 0.0 { 1.0 } else { 0.0 });
        dLdA.component_mul(&dAdZ) // dLdZ = dLdA * dA/dZ
    }
}

// Sigmoid Activation Function
pub struct Sigmoid {
    A : DMatrix<f64>
}

impl Sigmoid {
    pub fn new() -> Self {
        Sigmoid{
            A : DMatrix::zeros(0, 0)
        }
    }
    pub fn forward(&mut self, Z : &DMatrix<f64>) -> DMatrix<f64>{
        self.A = Z.map(|x| 1.0/(1.0 + consts::E.powf(x)));
        return self.A.clone();
    }
    pub fn backward(&self, dLdA : &DMatrix<f64>) -> DMatrix<f64>{
        let dAdZ = self.A.map(|x| x * (1.0 - x));
        return dLdA.component_mul(&dAdZ);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_identity_forward() {
        let mut identity = Identity::new();
        let Z = DMatrix::from_row_slice(2, 2, &[1.0, 2.0,
                                                3.0, 4.0]);
        let A = identity.forward(&Z);
        assert_abs_diff_eq!(A, Z, epsilon = 1e-12);
    }

    #[test]
    fn test_identity_backward() {
        let identity = Identity::new();
        let dLdA = DMatrix::from_row_slice(2, 2, &[1.0, 2.0,
                                                    3.0, 4.0]);
        let dLdZ = identity.backward(&dLdA);
        assert_abs_diff_eq!(dLdZ, dLdA, epsilon = 1e-12);
    }

    #[test]
    fn test_relu_forward() {
        let mut relu = ReLU::new();
        let Z = DMatrix::from_row_slice(2, 3, &[0.0378, 0.3022, -1.6123,
                                                -2.5186, -1.9395, 1.4077]);
        let A = relu.forward(&Z);
        let expected = DMatrix::from_row_slice(2, 3, &[0.0378, 0.3022,
                                                       0.0, 0.0, 0.0, 1.4077]);
        assert_abs_diff_eq!(A, expected, epsilon = 1e-12);
    }

    #[test]
    fn test_relu_backward() {
        let mut relu = ReLU::new();
        let Z = DMatrix::from_row_slice(2, 3, &[0.0378, 0.3022, -1.6123,
                                                -2.5186, -1.9395, 1.4077]);
        let _ = relu.forward(&Z);
        let dLdA = DMatrix::from_row_slice(2, 3, &[1.0, 2.0, 3.0,
                                                   4.0, 5.0, 6.0]); // Mock dLdA
        let dLdZ = relu.backward(&dLdA);
        let expected = DMatrix::from_row_slice(2, 3, &[1.0, 2.0, 0.0,
                                                       0.0, 0.0, 6.0]);
        assert_abs_diff_eq!(dLdZ, expected, epsilon = 1e-12);
    }

    fn test_sigmoid_forward(){
        let mut sigmoid = Sigmoid::new();
        let Z = DMatrix::from_row_slice(4, 2, &[-4.0, -3.0,
                                                -2.0, -1.0,
                                                0.0, 1.0,
                                                2.0, 3.0]);
        let A = sigmoid.forward(&Z);
        let expected = DMatrix::from_row_slice(4, 2, &[0.018, 0.0474,
                                                       0.1192, 0.2689,
                                                       0.5, 0.7311,
                                                       0.8808, 0.9526]);
        assert_abs_diff_eq!(A, expected, epsilon = 1e-12);
    }
    fn test_sigmoid_backward(){
        let mut sigmoid = Sigmoid::new();
        let Z = DMatrix::from_row_slice(4, 2, &[-4.0, -3.0,
                                                -2.0, -1.0,
                                                0.0, 1.0,
                                                2.0, 3.0]);
        let _ = sigmoid.forward(&Z);
        let dLdA = DMatrix::from_row_slice(4, 2, &[1.0, 1.0,
                                                   1.0, 1.0,
                                                   1.0, 1.0,
                                                   1.0, 1.0,]);
        let dLdZ = sigmoid.backward(&dLdA);
        let expected = DMatrix::from_row_slice(4, 2, &[0.0177, 0.0452,
                                                       0.105, 0.1966,
                                                       0.25, 0.1966,
                                                       0.105, 0.0452]);
        assert_abs_diff_eq!(dLdZ, expected, epsilon = 1e-12);
        
    }

}
