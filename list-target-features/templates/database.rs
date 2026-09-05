// This file is @generated.

use super::FeatureData;
use crate::TargetFeatures;

{% for architecture in architectures -%}
#[cfg(target_arch = "{{ architecture.module_name }}")]
pub(crate) const FEATURE_WORDS: usize = {{ architecture.feature_words }};
{% endfor %}
#[cfg(not(any(
{%- for architecture in architectures %}
    target_arch = "{{ architecture.module_name }}",
{%- endfor %}
)))]
pub(crate) const FEATURE_WORDS: usize = 0;

{% for architecture in architectures -%}
#[cfg(any(doc, target_arch = "{{ architecture.module_name }}"))]
#[rustfmt::skip]
pub mod {{ architecture.module_name }} {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
{%- for feature in architecture.features %}
        {{ feature.id }},
{%- endfor %}
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "{{ architecture.module_name }}")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "{{ architecture.module_name }}")))]
            {
                TargetFeatures::empty()
            }
        } };
    }

{%- for feature in architecture.features %}
    #[doc = {{ feature.description_literal }}]
    pub const {{ feature.id }}: TargetFeatures = feature_set!({% for implied in feature.closure %}{{ implied }}{% if !loop.last %}, {% endif %}{% endfor %});
{% endfor %}

    #[cfg(target_arch = "{{ architecture.module_name }}")]
    pub(crate) const FEATURES: &[FeatureData] = &[
{%- for feature in architecture.features %}
        FeatureData::new({{ feature.name_literal }}, {{ feature.id }}),
{%- endfor %}
    ];

    #[cfg(target_arch = "{{ architecture.module_name }}")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
{%- for feature in architecture.features %}
        #[cfg(target_feature = {{ feature.name_literal }})]
        let features = features.with({{ feature.id }});
{%- endfor %}
        features
    }

}
{% endfor %}

{% for architecture in architectures -%}
#[cfg(target_arch = "{{ architecture.module_name }}")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
{%- for feature in architecture.features %}
    ({{ feature.name_literal }}) => { $crate::{{ architecture.module_name }}::{{ feature.id }} };
{%- endfor %}
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
{% endfor %}

#[cfg(not(any(
{%- for architecture in architectures %}
    target_arch = "{{ architecture.module_name }}",
{%- endfor %}
)))]
#[doc(hidden)]
#[macro_export]
macro_rules! __target_feature {
    ($feature:tt) => {
        compile_error!(concat!("target features are unavailable for this architecture: ", stringify!($feature)))
    };
}

/// Constructs a [`TargetFeatures`] value from target feature names.
///
/// Each name identifies a target feature.
/// Multiple names are combined using [`TargetFeatures::with`].
///
/// ```
/// # #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// const REQUIRED: target_features::TargetFeatures =
///     target_features::target_features!("avx", "fma", "bmi2");
/// ```
#[macro_export]
macro_rules! target_features {
    ($($feature:tt),* $(,)?) => { {
        let features = $crate::TargetFeatures::empty();
        $(let features = features.with($crate::__target_feature!($feature));)*
        features
    } };
}

#[allow(unused_variables)]
pub(crate) const fn enabled_for_target() -> TargetFeatures {
    let features = TargetFeatures::empty();
{%- for architecture in architectures %}
    #[cfg(target_arch = "{{ architecture.module_name }}")]
    let features = {{ architecture.module_name }}::enabled_for_target();
{%- endfor %}
    features
}

#[allow(unused_variables)]
pub(crate) fn features() -> &'static [FeatureData] {
    let features: &'static [FeatureData] = &[];
{%- for architecture in architectures %}
    #[cfg(target_arch = "{{ architecture.module_name }}")]
    let features = {{ architecture.module_name }}::FEATURES;
{%- endfor %}
    features
}
