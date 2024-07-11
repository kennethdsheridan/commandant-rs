use crate::domain::ai_model::{AIModel, AIPrediction};

pub struct AIPredictionUseCase<T: AIModel> {
    model: T,
}

impl<T: AIModel> AIPredictionUseCase<T> {
    pub fn new(model: T) -> Self {
        Self { model }
    }

    pub fn execute(&self, input: Vec<f32>) -> AIPrediction {
        self.model.predict(input)
    }
}
