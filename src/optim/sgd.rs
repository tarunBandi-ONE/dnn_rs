use nalgebra::{DMatrix};
use crate::nn::model::NeuralNetwork;


/**
    * Stochastic Gradient Descent (SGD) Optimizer
    *
    * The Stochastic Gradient Descent (SGD) optimizer is a simple yet effective
    * optimization algorithm used to update the parameters of a neural network
    * during the training process. The basic idea behind SGD is to update the
    * parameters in the direction of the negative gradient of the loss function
    * with respect to the parameters. This is done by computing the gradient of
    * the loss with respect to the parameters for each sample in the training
    * data, and then updating the parameters using the average gradient over
    * the entire training data.

**/


pub struct SGD {
    pub model: NeuralNetwork,
    pub lr: f64, // Learning Rate
    pub mu: f64, // Momentum
    pub v_W: Vec<DMatrix<f64>>, // Velocity for weights
    pub v_b: Vec<DMatrix<f64>> // Velocity for biases
}

impl SGD {
    // Constructor for the SGD struct. Creates a new SGD optimizer with
    // the specified learning rate and momentum.
    pub fn new(model: NeuralNetwork, lr: f64, mu: f64) -> Self {
        let mut v_W = Vec::new();
        let mut v_b = Vec::new();
        for i in 0..model.layers.len() {
            v_W.push(DMatrix::zeros(model.layers[i].W.nrows(), model.layers[i].W.ncols()));
            v_b.push(DMatrix::zeros(model.layers[i].b.nrows(), model.layers[i].b.ncols()));
        }
        SGD {
            model: model,
            lr: lr,
            mu: mu,
            v_W: v_W,
            v_b: v_b
        }
    }

    // The update method is used to update the parameters of the neural network
    // using the Stochastic Gradient Descent (SGD) algorithm. The update is done
    // by computing the gradient of the loss with respect to the parameters for
    // each sample in the training data, and then updating the parameters using
    // the average gradient over the entire training data.
    pub fn update(&mut self, x: &DMatrix<f64>, y: &DMatrix<f64>) {

        // Forward pass (compute loss)
        let Z = self.model.forward(&x);
        let _ = self.model.loss.forward(&Z, &y);

        // Backward pass (compute gradients)
        let dLdA = self.model.backward();

        for i in 0..self.model.layers.len() {

            if self.mu == 0.0 {
                // Update the weights and biases using the negative gradient
                // of the loss with respect to the parameters
                let dLdW = self.model.layers[i].dLdW.clone();
                let dLdb = self.model.layers[i].dLdb.clone();
                self.model.layers[i].W -= self.lr * &dLdW;
                self.model.layers[i].b -= self.lr * &dLdb;
            } else {
                // Update the weights and biases using momentum
                let dLdW = self.model.layers[i].dLdW.clone();
                let dLdb = self.model.layers[i].dLdb.clone();
                self.v_W[i] = self.mu * &self.v_W[i] + &dLdW;
                self.v_b[i] = self.mu * &self.v_b[i] + &dLdb;
                self.model.layers[i].W -= self.lr * &self.v_W[i];
                self.model.layers[i].b -= self.lr * &self.v_b[i];
            }
        }
    }
}