use target_features::TargetFeatures;

const EMPTY: TargetFeatures = TargetFeatures::empty();
const _: () = assert!(TargetFeatures::enabled_for_target().contains(EMPTY));

#[test]
fn empty_contains_itself() {
    assert!(EMPTY.contains(TargetFeatures::empty()));
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    #[cfg(target_arch = "x86")]
    use target_features::x86 as features;
    #[cfg(target_arch = "x86_64")]
    use target_features::x86_64 as features;
    use target_features::TargetFeatures;

    const REQUIRED: TargetFeatures = features::AVX.with(features::FMA).with(features::BMI2);
    const FROM_NAMES: TargetFeatures = target_features::target_features!("avx", "fma", "bmi2",);

    const _: () = assert!(REQUIRED.contains(features::AVX));
    const _: () = assert!(REQUIRED.contains(features::FMA));
    const _: () = assert!(REQUIRED.contains(features::BMI2));
    const _: () = assert!(features::AVX2.contains(features::AVX));
    const _: () = assert!(REQUIRED.with(features::FMA).contains(REQUIRED));
    const _: () = assert!(REQUIRED.contains(REQUIRED.with(features::FMA)));
    const _: () = assert!(FROM_NAMES.contains(REQUIRED));
    const _: () = assert!(REQUIRED.contains(FROM_NAMES));
    const _: () = assert!(target_features::target_features!().contains(TargetFeatures::empty()));

    #[test]
    fn implication_closed_composition() {
        assert!(features::AVX2.contains(features::AVX));
        assert_eq!(REQUIRED.with(features::FMA), REQUIRED);
        assert_eq!(FROM_NAMES, REQUIRED);
        assert_eq!(
            target_features::target_features!("sse4.1"),
            features::SSE4_1
        );
        assert_eq!(
            features::AVX.with(features::FMA),
            features::FMA.with(features::AVX)
        );
    }

    #[test]
    fn debug_uses_concise_basis() {
        assert_eq!(
            format!("{REQUIRED:?}"),
            "TargetFeatures([\"bmi2\", \"fma\"])"
        );
    }

    #[test]
    fn enabled_for_target_contains_architecture_baseline() {
        let enabled_for_target = TargetFeatures::enabled_for_target();
        #[cfg(target_arch = "x86_64")]
        assert!(enabled_for_target.contains(features::SSE2));
        #[cfg(target_arch = "x86")]
        assert!(enabled_for_target.contains(TargetFeatures::empty()));
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
#[test]
fn tied_features_have_equal_values() {
    #[cfg(target_arch = "aarch64")]
    use target_features::aarch64 as features;
    #[cfg(target_arch = "arm64ec")]
    use target_features::arm64ec as features;

    assert_eq!(features::PACA, features::PACG);
    assert_eq!(
        format!("{:?}", features::PACG),
        "TargetFeatures([\"paca\"])"
    );
}
