mod data;

use burn::tensor::backend::NdArray;
use burn::tensor::Tensor;
use burn::optim::Adam;
use burn::module::Module;
use burn::nn::Linear;
use burn::nn::loss::MSELoss;
use burn::config::Config;
use data::generate_data;

#[derive(Module, Debug)]
struct LinearModel {
    layer: Linear<f32>,
}

impl LinearModel {
    fn new() -> Self {
        Self {
            layer: Linear::new(1, 1), // 1 input, 1 output
        }
    }

    fn forward(&self, x: Tensor<NdArray, 2>) -> Tensor<NdArray, 2> {
        self.layer.forward(x)
    }
}

fn main() {

        train_model();
        evaluate_model(&model);

}

fn train_model() {
    let data = generate_data(100);
    let mut model = LinearModel::new();
    let mut optimizer = Adam::new(0.01);

    for epoch in 0..100 {
        let x: Vec<f32> = data.iter().map(|(xi, _)| *xi).collect();
        let y: Vec<f32> = data.iter().map(|(_, yi)| *yi).collect();

        let x_tensor = Tensor::<NdArray, 2>::from_data(&x);
        let y_tensor = Tensor::<NdArray, 2>::from_data(&y);

        let output = model.forward(x_tensor.clone());
        let loss = MSELoss::new(output, y_tensor.clone());

        optimizer.step(&mut model, &loss);

        if epoch % 10 == 0 {
            println!("Epoch {}: Loss = {:?}", epoch, loss);
        }
    }
}

