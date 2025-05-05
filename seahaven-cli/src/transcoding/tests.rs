// Code borrowed with modifications from: https://github.com/sfackler/serde-transcode/blob/7c8dae5816dae317b0132ee02a35bb5d59b163a7/src/test.rs
// The original project (sfackler/serde-transcode) is licensed under either of the MIT license or the Apache License 2.0.

use std::collections::HashMap;

use crate::transcoding::transcode;

#[test]
fn boolean_true() {
    //* Given
    let input = true;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: bool =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn boolean_false() {
    //* Given
    let input = false;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: bool =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i8_min() {
    //* Given
    let input = i8::MIN;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i8 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i8_zero() {
    //* Given
    let input = 0i8;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i8 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i8_max() {
    //* Given
    let input = i8::MAX;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i8 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i16_min() {
    //* Given
    let input = i16::MIN;

    // Type 'i16' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i16 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i16_zero() {
    //* Given
    let input = 0i16;

    // Type 'i16' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i16 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i16_max() {
    //* Given
    let input = i16::MAX;

    // Type 'i16' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i16 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i32_min() {
    //* Given
    let input = i32::MIN;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i32 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i32_zero() {
    //* Given
    let input = 0i32;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i32 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i32_max() {
    //* Given
    let input = i32::MAX;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i32 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i64_min() {
    //* Given
    let input = i64::MIN;

    // Type 'i64' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i64 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i64_zero() {
    //* Given
    let input = 0i64;

    // Type 'i64' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i64 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn i64_max() {
    //* Given
    let input = i64::MAX;

    // Type 'i64' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: i64 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn isize_min() {
    //* Given
    let input = isize::MIN;

    // Type 'isize' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: isize =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn isize_zero() {
    //* Given
    let input = 0isize;

    // Type 'isize' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: isize =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn isize_max() {
    //* Given
    let input = isize::MAX;

    // Type 'isize' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: isize =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u8_zero() {
    //* Given
    let input = 0u8;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u8 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u8_max() {
    //* Given
    let input = u8::MAX;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u8 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u16_zero() {
    //* Given
    let input = 0u16;

    // Type 'u16' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u16 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u16_max() {
    //* Given
    let input = u16::MAX;

    // Type 'u16' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u16 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u32_zero() {
    //* Given
    let input = 0u32;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u32 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u32_max() {
    //* Given
    let input = u32::MAX;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u32 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u64_zero() {
    //* Given
    let input = 0u64;

    // Type 'u64' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u64 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn u64_large() {
    //* Given
    let input = u32::MAX as u64 + 1;

    // Type 'u64' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: u64 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn usize_zero() {
    //* Given
    let input = 0usize;

    // Type 'usize' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: usize =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn usize_large() {
    //* Given
    let input = u32::MAX as usize + 1;

    // Type 'usize' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: usize =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

serde_if_integer128! {
    #[test]
    fn i128_min() {
        //* Given
        let input = i64::MIN as i128;

        // Type 'i128' is not supported by `toml::Value`
        let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

        //* When
        let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
            .expect("Failed to serialize input to YAML value");

        //* Then
        let output: i128 = serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
        assert_eq!(input, output);
    }

    #[test]
    fn i128_zero() {
        //* Given
        let input = 0i128;

        // Type 'i128' is not supported by `toml::Value`
        let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

        //* When
        let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
            .expect("Failed to serialize input to YAML value");

        //* Then
        let output: i128 = serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
        assert_eq!(input, output);
    }

    #[test]
    fn i128_large() {
        //* Given
        let input = i64::MAX as i128 + 1;

        // Type 'i128' is not supported by `toml::Value`
        let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

        //* When
        let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
            .expect("Failed to serialize input to YAML value");

        //* Then
        let output: i128 = serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
        assert_eq!(input, output);
    }

    #[test]
    fn u128_zero() {
        //* Given
        let input = 0u128;

        // Type 'u128' is not supported by `toml::Value`
        let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

        //* When
        let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
            .expect("Failed to serialize input to YAML value");

        //* Then
        let output: u128 = serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
        assert_eq!(input, output);
    }

    #[test]
    fn u128_large() {
        //* Given
        let input = u32::MAX as u128 + 1;

        // Type 'u128' is not supported by `toml::Value`
        let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

        //* When
        let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
            .expect("Failed to serialize input to YAML value");

        //* Then
        let output: u128 = serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
        assert_eq!(input, output);
    }
}

#[test]
fn f32_positive() {
    //* Given
    let input = 1.3f32;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: f32 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn f32_negative() {
    //* Given
    let input = -1e10f32;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: f32 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn f64_positive() {
    //* Given
    let input = 1.3f64;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: f64 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn f64_negative() {
    //* Given
    let input = -1e10f64;

    let input_value = toml::Value::from(input);

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: f64 =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn char_letter() {
    //* Given
    let input = 'a';

    // Type 'char' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: char =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn char_null() {
    //* Given
    let input = '\0';

    // Type 'char' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: char =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn string_nonempty() {
    //* Given
    let input = String::from("hello world");

    let input_value = toml::Value::from(input.clone());

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: String =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn string_empty() {
    //* Given
    let input = String::from("");

    let input_value = toml::Value::from(input.clone());

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: String =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn unit_type() {
    //* Given
    let input = ();

    // Type '()' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let _output: () =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    // `assert_eq!` of unit values will always succeed
}

#[test]
fn option_none() {
    //* Given
    let input: Option<i32> = None;

    // Type 'Option' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: Option<i32> =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn option_some_int() {
    //* Given
    let input = Some(0i32);

    // Type 'Option' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: Option<i32> =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn option_some_string() {
    //* Given
    let input = Some(String::from("hi"));

    // Type 'Option' is not supported by `toml::Value`
    let input_value = serde_yaml::to_value(&input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: Option<String> =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn sequence_numbers() {
    //* Given
    let input = vec![0, 1, 2, 3];

    let input_value = toml::Value::from(input.clone());

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: Vec<i32> =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn map_string_vec() {
    //* Given
    let input = HashMap::from_iter([
        ("hello".to_string(), vec![1, 2]),
        ("goodbye".to_string(), vec![]),
    ]);

    let input_value = toml::Value::from(input.clone());

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: HashMap<String, Vec<i32>> =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}

#[test]
fn newtype_struct() {
    //* Given
    #[derive(PartialEq, Debug)]
    struct Foo(i32);

    impl serde::ser::Serialize for Foo {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            s.serialize_newtype_struct("Foo", &self.0)
        }
    }

    impl<'de> serde::de::Deserialize<'de> for Foo {
        fn deserialize<D>(d: D) -> Result<Foo, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct V;

            impl<'de> serde::de::Visitor<'de> for V {
                type Value = Foo;

                fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(fmt, "a Foo struct")
                }

                fn visit_newtype_struct<D>(self, d: D) -> Result<Foo, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    Ok(Foo(serde::de::Deserialize::deserialize(d)?))
                }
            }

            d.deserialize_newtype_struct("Foo", V)
        }
    }

    let input = Foo(100);

    // Serialize the input to `serde_yaml::Value`
    let input_value = serde_yaml::to_value(&input).expect("Failed to serialize input to YAML");

    //* When
    let transcoded_value = transcode(input_value, serde_yaml::value::Serializer)
        .expect("Failed to serialize input to YAML value");

    //* Then
    let output: Foo =
        serde_yaml::from_value(transcoded_value).expect("Failed to deserialize to input type");
    assert_eq!(input, output);
}
