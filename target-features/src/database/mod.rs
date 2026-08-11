use crate::TargetFeatures;

mod generated;
pub use generated::*;

#[derive(Copy, Clone)]
pub(crate) struct FeatureData {
    pub(crate) name: &'static str,
    pub(crate) features: TargetFeatures,
}

impl FeatureData {
    pub(crate) const fn new(name: &'static str, features: TargetFeatures) -> Self {
        Self { name, features }
    }
}
