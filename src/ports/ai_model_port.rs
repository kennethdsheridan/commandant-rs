use crate::domain::ai_model::{AIPrediction};

pub trait AIModelPort{
    fn predict(&self, input: Vec<f32>) -> AIPrediction;
    fn load_pretrained() -> Result<Self, burn::Error> where Self: Sized;

}
