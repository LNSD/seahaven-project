//! Extension of the `serde_yaml` crate providing YAML value merging functionality.

/// Trait extending [`Value`] with merging capabilities.
///
/// Merging follows these rules:
/// - Mappings: Recursively merge matching keys, insert new keys.
/// - Sequences: Append source to target.
/// - Scalars: Overwrite target with source.
///
/// [`Value`]: serde_yaml::Value
pub trait ValueMergeExt: _priv::Sealed {
    /// Merges the current value with another [`Value`].
    ///
    /// The current value is modified in-place according to the merging rules.
    ///
    /// [`Value`]: serde_yaml::Value
    fn merge(&mut self, other: &serde_yaml::Value);

    /// Merges the current value into the target [`Value`].
    ///
    /// The target [`Value`] is modified in-place according to the merging rules.
    ///
    /// [`Value`]: serde_yaml::Value
    fn merge_into(&self, target: &mut serde_yaml::Value);
}

impl ValueMergeExt for serde_yaml::Value {
    fn merge(&mut self, other: &serde_yaml::Value) {
        match (self, other) {
            // If the source and the target are both mappings, merge the source into the target.
            (serde_yaml::Value::Mapping(dst_map), serde_yaml::Value::Mapping(src_map)) => {
                for (key, src_val) in src_map {
                    // If the key is present in both, recursively merge the values;
                    // otherwise, insert the source key into the target.
                    match dst_map.get_mut(key) {
                        Some(dst_val) => dst_val.merge(src_val),
                        None => {
                            dst_map.insert(key.clone(), src_val.clone());
                        }
                    };
                }
            }

            // If the source and the target are both sequences, extend the target with the source.
            (serde_yaml::Value::Sequence(dst_seq), serde_yaml::Value::Sequence(src_seq)) => {
                dst_seq.extend_from_slice(src_seq);
            }

            // Otherwise overwrite the target value with the source value.
            (dst, src) => *dst = src.clone(),
        }
    }

    fn merge_into(&self, target: &mut serde_yaml::Value) {
        target.merge(self);
    }
}

impl _priv::Sealed for serde_yaml::Value {}

/// Trait extending [`Mapping`] with merging capabilities.
///
/// Merging follows these rules:
/// - For matching keys: Recursively merge the values if both are mappings, otherwise overwrite
/// - For new keys: Insert the key-value pair from source
///
/// [`Mapping`]: serde_yaml::Mapping
pub trait MappingMergeExt: _priv::Sealed {
    /// Merges the current mapping with another [`Mapping`].
    ///
    /// The current mapping is modified in-place according to the merging rules.
    ///
    /// [`Mapping`]: serde_yaml::Mapping
    fn merge(&mut self, other: &serde_yaml::Mapping);

    /// Merges the current mapping into the target [`Mapping`].
    ///
    /// The target [`Mapping`] is modified in-place according to the merging rules.
    ///
    /// [`Mapping`]: serde_yaml::Mapping
    fn merge_into(&self, target: &mut serde_yaml::Mapping);
}

impl MappingMergeExt for serde_yaml::Mapping {
    fn merge(&mut self, other: &serde_yaml::Mapping) {
        for (key, src_val) in other {
            match self.get_mut(key) {
                Some(dst_val) => {
                    dst_val.merge(src_val);
                }
                None => {
                    // Insert new key-value pair
                    self.insert(key.clone(), src_val.clone());
                }
            }
        }
    }

    fn merge_into(&self, target: &mut serde_yaml::Mapping) {
        target.merge(self);
    }
}

impl _priv::Sealed for serde_yaml::Mapping {}

/// Trait extending [`Sequence`] with merging capabilities.
///
/// Merging follows these rules:
/// - Append all elements from the source sequence to the target sequence
///
/// [`Sequence`]: serde_yaml::Sequence
pub trait SequenceMergeExt: _priv::Sealed {
    /// Merges the current sequence with another [`Sequence`].
    ///
    /// The current sequence is modified in-place by appending all elements from the source sequence.
    ///
    /// [`Sequence`]: serde_yaml::Sequence
    fn merge(&mut self, other: &serde_yaml::Sequence);

    /// Merges the current sequence into the target [`Sequence`].
    ///
    /// The target [`Sequence`] is modified in-place by appending all elements from the current sequence.
    ///
    /// [`Sequence`]: serde_yaml::Sequence
    fn merge_into(&self, target: &mut serde_yaml::Sequence);
}

impl SequenceMergeExt for serde_yaml::Sequence {
    fn merge(&mut self, other: &serde_yaml::Sequence) {
        self.extend_from_slice(other);
    }

    fn merge_into(&self, target: &mut serde_yaml::Sequence) {
        target.extend_from_slice(self);
    }
}

impl _priv::Sealed for serde_yaml::Sequence {}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use serde_yaml::Value;

    use super::{MappingMergeExt as _, SequenceMergeExt as _, ValueMergeExt as _};

    #[test]
    fn merge_mappings_recursively() {
        //* Given
        let mut target: Value = serde_yaml::from_str(indoc::indoc! {r#"
            a: 1
            b: 2
            nested:
                x: 10
                y: 20
        "#})
        .expect("Failed to parse target YAML");

        let source: Value = serde_yaml::from_str(indoc::indoc! {r#"
            b: 3
            c: 4
            nested:
                y: 30
                z: 40
        "#})
        .expect("Failed to parse source YAML");

        //* When
        target.merge(&source);

        //* Then
        let expected: Value = serde_yaml::from_str(indoc::indoc! {r#"
            a: 1
            b: 3
            c: 4
            nested:
                x: 10
                y: 30
                z: 40
        "#})
        .expect("Failed to parse expected YAML");

        assert_eq!(target, expected);
    }

    #[test]
    fn append_sequences_on_merge() {
        //* Given
        let mut target: Value =
            serde_yaml::from_str("[1, 2, 3]").expect("Failed to parse target YAML");

        let source: Value = serde_yaml::from_str("[4, 5]").expect("Failed to parse source YAML");

        //* When
        target.merge(&source);

        //* Then
        let expected: Value =
            serde_yaml::from_str("[1, 2, 3, 4, 5]").expect("Failed to parse expected YAML");

        assert_eq!(target, expected);
    }

    #[test]
    fn overwrite_scalars_on_merge() {
        //* Given
        let mut target = Value::String(String::from("old"));
        let source = Value::String(String::from("new"));

        // When
        target.merge(&source);

        // Then
        assert_eq!(target.as_str(), Some("new"));
    }

    #[test]
    fn replace_on_different_types() {
        //* Given
        let mut target: Value = serde_yaml::from_str(indoc::indoc! {r#"
            a: 1
            b: 2
        "#})
        .expect("Failed to parse target YAML");

        let source: Value = serde_yaml::from_str(indoc::indoc! {r#"
            - 1
            - 2
            - 3
        "#})
        .expect("Failed to parse source YAML");

        //* When
        target.merge(&source);

        //* Then
        assert_eq!(target, source);
    }

    #[test]
    fn merge_into() {
        // Given
        let source: Value = serde_yaml::from_str(indoc::indoc! {r#"
            a: 1
            b: 2
        "#})
        .expect("Failed to parse source YAML");

        let mut target: Value = serde_yaml::from_str(indoc::indoc! {r#"
            b: 3
            c: 4
        "#})
        .expect("Failed to parse target YAML");

        // When
        source.merge_into(&mut target);

        // Then
        let expected: Value = serde_yaml::from_str(indoc::indoc! {r#"
            a: 1
            b: 2
            c: 4
        "#})
        .expect("Failed to parse expected YAML");

        assert_eq!(target, expected);
    }

    #[test]
    fn mapping_merge_recursively() {
        //* Given
        let target = serde_yaml::Mapping::from_iter([
            (
                Value::String("a".to_string()),
                Value::Mapping(
                    serde_yaml::from_str("{x: 1, y: 2}").expect("Failed to parse nested mapping"),
                ),
            ),
            (Value::String("b".to_string()), Value::Number(1.into())),
        ]);

        let source = serde_yaml::Mapping::from_iter([
            (
                Value::String("a".to_string()),
                Value::Mapping(
                    serde_yaml::from_str("{y: 3, z: 4}").expect("Failed to parse nested mapping"),
                ),
            ),
            (Value::String("c".to_string()), Value::Number(2.into())),
        ]);

        //* When
        let mut target = target;
        target.merge(&source);

        //* Then
        let expected = serde_yaml::Mapping::from_iter([
            (
                Value::String("a".to_string()),
                Value::Mapping(
                    serde_yaml::from_str("{x: 1, y: 3, z: 4}")
                        .expect("Failed to parse expected mapping"),
                ),
            ),
            (Value::String("b".to_string()), Value::Number(1.into())),
            (Value::String("c".to_string()), Value::Number(2.into())),
        ]);

        assert_eq!(target, expected);
    }

    #[test]
    fn mapping_merge_into() {
        //* Given
        let source = serde_yaml::Mapping::from_iter([
            (Value::String("a".to_string()), Value::Number(1.into())),
            (Value::String("b".to_string()), Value::Number(2.into())),
        ]);

        let mut target = serde_yaml::Mapping::from_iter([
            (Value::String("b".to_string()), Value::Number(3.into())),
            (Value::String("c".to_string()), Value::Number(4.into())),
        ]);

        //* When
        source.merge_into(&mut target);

        //* Then
        let expected = serde_yaml::Mapping::from_iter([
            (Value::String("a".to_string()), Value::Number(1.into())),
            (Value::String("b".to_string()), Value::Number(2.into())),
            (Value::String("c".to_string()), Value::Number(4.into())),
        ]);

        assert_eq!(target, expected);
    }

    #[test]
    fn sequence_merge() {
        //* Given
        let mut target =
            serde_yaml::Sequence::from_iter([Value::Number(1.into()), Value::Number(2.into())]);

        let source =
            serde_yaml::Sequence::from_iter([Value::Number(3.into()), Value::Number(4.into())]);

        //* When
        target.merge(&source);

        //* Then
        let expected = serde_yaml::Sequence::from_iter([
            Value::Number(1.into()),
            Value::Number(2.into()),
            Value::Number(3.into()),
            Value::Number(4.into()),
        ]);

        assert_eq!(target, expected);
    }

    #[test]
    fn sequence_merge_into() {
        //* Given
        let source = serde_yaml::Sequence::from_iter([
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);

        let mut target = serde_yaml::Sequence::from_iter([
            Value::String("c".to_string()),
            Value::String("d".to_string()),
        ]);

        //* When
        source.merge_into(&mut target);

        //* Then
        let expected = serde_yaml::Sequence::from_iter([
            Value::String("c".to_string()),
            Value::String("d".to_string()),
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);

        assert_eq!(target, expected);
    }
}
