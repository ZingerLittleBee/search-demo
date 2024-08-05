use std::path::Path;

use ndarray::{Array1, ArrayView1};
use ort::{CoreMLExecutionProvider, ExecutionProvider, GraphOptimizationLevel, Session};
use tracing::warn;

pub(crate) struct ONNXModelConfig {
    pub num_intra_thread: usize,
    pub optimization_level: GraphOptimizationLevel,
}

impl Default for ONNXModelConfig {
    fn default() -> Self {
        Self {
            num_intra_thread: 16,
            optimization_level: GraphOptimizationLevel::Level3,
        }
    }
}

pub(crate) fn load_onnx_model(
    model_path: impl AsRef<Path>,
    config: Option<ONNXModelConfig>,
) -> anyhow::Result<Session> {
    let builder = Session::builder()?;

    let coreml = CoreMLExecutionProvider::default();

    if let Err(e) = coreml.register(&builder) {
        warn!("failed to register CoreMLExecutionProvider: {e}");
    }

    let config = config.unwrap_or(Default::default());

    let session = builder
        .with_intra_threads(config.num_intra_thread)?
        .with_optimization_level(config.optimization_level)?
        .commit_from_file(model_path)?;

    Ok(session)
}

fn l2_norm(x: ArrayView1<f32>) -> f32 {
    x.dot(&x).sqrt()
}

pub fn normalize(mut x: Array1<f32>) -> Array1<f32> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e / norm);
    x
}
