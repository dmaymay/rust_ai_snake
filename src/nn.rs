use ::rand::Rng;

#[derive(Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    // Create a new matrix
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn print(&self) {
        // Calculate maximum width for each column
        let mut max_widths: Vec<usize> = vec![0; self.cols];
        for j in 0..self.cols {
            let mut max_width = 0;
            for i in 0..self.rows {
                let width = format!("{:.3}", self.data[i][j]).len();
                if width > max_width {
                    max_width = width;
                }
            }
            max_widths[j] = max_width;
        }

        // Print each value formatted to the max width of its column
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{:width$.3} ", self.data[i][j], width = max_widths[j]);
            }
            println!();
        }
    }

    // Element-wise addition
    pub fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    // Element-wise subtraction
    pub fn subtract(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }

    // Matrix multiplication
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            println!("Matrix multiplication dimension mismatch:");
            println!("Matrix A (self):");
            self.print();
            println!("Matrix B (other):");
            other.print();
            panic!(
                "Dimensions of Matrix A (cols) and Matrix B (rows) must match for multiplication."
            );
        }
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    // Element-wise (Hadamard) multiplication
    pub fn hadamard(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            println!("Matrix hadamard multiplication dimension mismatch:");
            println!("Matrix A (self):");
            self.print();
            println!("Matrix B (other):");
            other.print();
            panic!("Dimensions of Matrix A and Matrix B must match for hadamard multiplication.");
        }
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        result
    }

    // Scalar multiplication
    pub fn scalar_multiply(&self, scalar: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * scalar;
            }
        }
        result
    }

    // Transpose
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    // Apply a function element-wise to the matrix
    pub fn apply<F>(&self, func: F) -> Matrix
    where
        F: Fn(f64) -> f64,
    {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = func(self.data[i][j]);
            }
        }
        result
    }

    pub fn from_array_to_row(data: &[f64]) -> Self {
        Self {
            data: vec![data.to_vec()],
            rows: 1,
            cols: data.len(),
        }
    }

    pub fn from_array_to_column(data: &[f64]) -> Self {
        Self {
            data: data.iter().map(|&value| vec![value]).collect(),
            rows: data.len(),
            cols: 1,
        }
    }

    pub fn random(rows: usize, cols: usize, min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let data = (0..rows)
            .map(|_| {
                (0..cols)
                    .map(|_| rng.gen::<f64>() * (max - min) + min)
                    .collect()
            })
            .collect();
        Self { data, rows, cols }
    }

    pub fn norm(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                sum += self.data[i][j] * self.data[i][j];
            }
        }
        sum.sqrt()
    }
}

use std::ops::Index;

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub struct Layer {
    weights: Matrix,
    biases: Matrix,
    activation: bool,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize, activation: bool) -> Self {
        // Randomly initialize weights and biases with small values.
        let variance = 2.0 / input_size as f64;
        let std_dev = variance.sqrt();

        let weights = Matrix::random(input_size, output_size, -std_dev, std_dev);
        let biases = Matrix::random(1, output_size, 0.0, 0.0);
        Self {
            weights,
            biases,
            activation,
        }
    }

    pub fn forward(&self, input: &Matrix) -> Matrix {
        let z = input.multiply(&self.weights).add(&self.biases);
        if self.activation {
            z.apply(|x| if x > 0.0 { x } else { 0.01 * x })
        } else {
            z
        }
    }
}

pub struct NeuralNetwork {
    layers: Vec<Layer>, // Layers of the neural network
    learning_rate: f64, // Learning rate for optimization
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        // Here, we'll initialize one hidden layer and one output layer.
        let input_layer = Layer::new(input_size, hidden_size, true);
        let hidden_layer = Layer::new(hidden_size, output_size, false);
        Self {
            layers: vec![input_layer, hidden_layer],
            learning_rate: 0.001, // Some default value; can be adjusted
        }
    }

    pub fn forward(&self, state: &[i32; 12]) -> [f64; 4] {
        let state_f64: Vec<f64> = state.iter().map(|&x| x as f64).collect();
        let mut input = Matrix::from_array_to_row(&state_f64);

        for layer in &self.layers {
            input = layer.forward(&input);
        }
        assert_eq!(input.rows, 1);
        assert_eq!(input.cols, 4);
        // Convert the final matrix to an array of size 4
        [
            input[0][0] as f64,
            input[0][1] as f64,
            input[0][2] as f64,
            input[0][3] as f64,
        ]
    }

    pub fn backward(
        &mut self,
        state: &[i32; 12],
        target_qvalues: [f64; 4],
        predicted_qvalues: [f64; 4],
    ) {
  
        // Gradient of MSE loss with respect to predicted Q-values
        let mut error = Matrix::from_array_to_column(&[
            2.0 * (predicted_qvalues[0] - target_qvalues[0]) as f64,
            2.0 * (predicted_qvalues[1] - target_qvalues[1]) as f64,
            2.0 * (predicted_qvalues[2] - target_qvalues[2]) as f64,
            2.0 * (predicted_qvalues[3] - target_qvalues[3]) as f64,
        ]);

        let mut derivatives: Vec<Matrix> = Vec::new();
        let mut activations: Vec<Matrix> = Vec::new();
        let state_f64: Vec<f64> = state.iter().map(|&x| x as f64).collect();

        activations.push(Matrix::from_array_to_row(&state_f64));

        // Derivative computation and storage (using ReLU derivative)

        let mut input = activations[0].clone();
        for layer in &self.layers {
            let z = input.multiply(&layer.weights).add(&layer.biases);
            let a = z.apply(|x| if x > 0.0 { x } else { 0.01 * x });
            activations.push(a.clone());
            let da = z.apply(|x| if x > 0.0 { 1.0 } else { 0.01 });
            derivatives.push(da.transpose());
            input = a;
        }

        // Step 2: Backward pass through the layers
        for (idx, layer) in self.layers.iter_mut().enumerate().rev() {
            let derror_dweights = error.hadamard(&derivatives[idx]);
            let mut gradient_weights = activations[idx]
                .transpose()
                .multiply(&derror_dweights.transpose());
            let mut gradient_biases = derror_dweights.transpose();
            // Step 3: Weight Update using Gradient Descent

            // Gradient clipping
            let threshold: f64 = 10.0;
            if gradient_weights.norm() > threshold {
                gradient_weights =
                    gradient_weights.scalar_multiply(threshold / gradient_weights.norm());
            }
            if gradient_biases.norm() > threshold {
                gradient_biases =
                    gradient_biases.scalar_multiply(threshold / gradient_biases.norm());
            }

            layer.weights = layer
                .weights
                .subtract(&gradient_weights.scalar_multiply(self.learning_rate));

            layer.biases = layer
                .biases
                .subtract(&gradient_biases.scalar_multiply(self.learning_rate));

            // Update error for the next iteration (if there's another layer)
            error = layer.weights.multiply(&derror_dweights);
        }
    }
}
