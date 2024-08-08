/// Burn AI Model Adapter Module
///
/// This module provides an adapter for the Burn-based AI model,
/// implementing the AiModelPort trait to integrate with the rest of the application.

use crate::domain::ai_model::{AiModel, AIPrediction};
use crate::ports::ai_model_port::AiModelPort;
use burn::backend::NdArrayBackend;

/// Adapter for the Burn-based AI model.
///
/// This struct wraps the AiModel and provides an implementation of the AiModelPort trait,
/// allowing it to be used interchangeably with other AI model implementations in the application.
pub struct BurnAiModelAdapter {
    /// The underlying Burn-based AI model.
    model: AiModel<NdArrayBackend>,
}

impl BurnAiModelAdapter {
    /// Creates a new instance of the BurnAiModelAdapter.
    ///
    /// This method initializes a new AI model with predefined input and output sizes.
    ///
    /// # Returns
    ///
    /// A new `BurnAiModelAdapter` instance.
    pub fn new() -> Self {
        // TODO: Adjust input_size and output_size as needed for your specific use case
        let model = AiModel::new(10, 1);
        BurnAiModelAdapter { model }
    }
}

impl AiModelPort for BurnAiModelAdapter {
    /// Makes a prediction using the AI model.
    ///
    /// # Arguments
    ///
    /// * `input` - A slice of i32 values representing the input data.
    ///
    /// # Returns
    ///
    /// A vector of f32 values representing the model's prediction.
    fn predict(&self, input: &[i32]) -> AIPrediction {
        // Convert input from i32 to f32
        let input_f32: Vec<f32> = input.iter().map(|&x| x as f32).collect();
        
        // Use the underlying model to make a prediction
        self.model.predict(&input_f32)
    }

    /// Loads a pretrained model from a file.
    ///
    /// # Arguments
    ///
    /// * `model_path` - A string slice that holds the path to the pretrained model file.
    ///
    /// # Returns
    ///
    /// A Result containing either the loaded BurnAiModelAdapter or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the model cannot be loaded.
    fn load_pretrained(model_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let model = AiModel::load_pretrained(model_path)?;
        Ok(BurnAiModelAdapter { model })
    }
}
