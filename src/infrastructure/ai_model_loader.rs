use crate::adapters::burn_ai_model::BurnAIModel;
use std::sync::Once;

pub struct AIModelLoader;

// Use Once for thread-safe lazy initialization
static INIT: Once = Once::new();
static mut MODEL: Option<BurnAIModel> = None;

impl AIModelLoader {
    pub fn load() -> Result<&'static BurnAIModel, burn::Error> {
        INIT.call_once(|| {
            // This is safe because we're ensuring single-threaded access with Once
            unsafe {
                MODEL = Some(BurnAIModel::load_pretrained().expect("Failed to load AI model"));
            }
        });

        // This is safe because we're ensuring single-threaded access with Once
        unsafe { MODEL.as_ref().ok_or(burn::Error::Other("Model not initialized".into())) }
    }
}
