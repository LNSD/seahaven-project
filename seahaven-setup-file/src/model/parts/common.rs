use std::marker::PhantomData;

/// A helper struct that deserializes a package use from a string or a struct.
pub struct FromStructOrString;

impl<'de, T> serde_with::DeserializeAs<'de, T> for FromStructOrString
where
    T: serde::Deserialize<'de> + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // This is a Visitor that forwards string types to T's `FromStr` impl and
        // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
        // keep the compiler from complaining about T being an unused generic type
        // parameter. We need T in order to know the Value type for the Visitor
        // impl.
        struct StringOrStruct<T>(PhantomData<fn() -> T>);

        impl<'de, T> serde::de::Visitor<'de> for StringOrStruct<T>
        where
            T: serde::Deserialize<'de> + std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Display,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<T, E>
            where
                E: serde::de::Error,
            {
                std::str::FromStr::from_str(value).map_err(E::custom)
            }

            fn visit_map<M>(self, map: M) -> Result<T, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                T::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(StringOrStruct(PhantomData))
    }
}

impl<T> serde_with::SerializeAs<T> for FromStructOrString
where
    T: serde::Serialize,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        value.serialize(serializer)
    }
}
