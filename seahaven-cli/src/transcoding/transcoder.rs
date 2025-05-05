use std::cell::Cell;

use serde::ser::{SerializeMap as _, SerializeSeq as _};

/// A Serde transcoder.
///
/// In most cases, the `transcode` function should be used instead of this
/// type.
///
/// # Note
///
/// Unlike traditional serializable types, `Transcoder`'s `Serialize`
/// implementation is *not* idempotent, as it advances the state of its
/// internal `Deserializer`. It should only ever be serialized once.
pub struct Transcoder<D>(Cell<Option<D>>);

impl<'de, D> Transcoder<D>
where
    D: serde::de::Deserializer<'de>,
{
    /// Constructs a new `Transcoder`.
    pub fn new(d: D) -> Transcoder<D> {
        Transcoder(Cell::new(Some(d)))
    }
}

impl<'de, D> serde::ser::Serialize for Transcoder<D>
where
    D: serde::de::Deserializer<'de>,
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.0
            .take()
            .expect("Transcoder may only be serialized once")
            .deserialize_any(Visitor(s))
            .map_err(d2s)
    }
}

struct Visitor<S>(S);

impl<'de, S> serde::de::Visitor<'de> for Visitor<S>
where
    S: serde::ser::Serializer,
{
    type Value = S::Ok;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "any value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_bool(v).map_err(s2d)
    }

    fn visit_i8<E>(self, v: i8) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_i8(v).map_err(s2d)
    }

    fn visit_i16<E>(self, v: i16) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_i16(v).map_err(s2d)
    }

    fn visit_i32<E>(self, v: i32) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_i32(v).map_err(s2d)
    }

    fn visit_i64<E>(self, v: i64) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_i64(v).map_err(s2d)
    }

    fn visit_u8<E>(self, v: u8) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_u8(v).map_err(s2d)
    }

    fn visit_u16<E>(self, v: u16) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_u16(v).map_err(s2d)
    }

    fn visit_u32<E>(self, v: u32) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_u32(v).map_err(s2d)
    }

    fn visit_u64<E>(self, v: u64) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_u64(v).map_err(s2d)
    }

    serde_if_integer128! {
        fn visit_i128<E>(self, v: i128) -> Result<S::Ok, E>
            where E: serde::de::Error
        {
            self.0.serialize_i128(v).map_err(s2d)
        }

        fn visit_u128<E>(self, v: u128) -> Result<S::Ok, E>
            where E: serde::de::Error
        {
            self.0.serialize_u128(v).map_err(s2d)
        }
    }

    fn visit_f32<E>(self, v: f32) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_f32(v).map_err(s2d)
    }

    fn visit_f64<E>(self, v: f64) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_f64(v).map_err(s2d)
    }

    fn visit_char<E>(self, v: char) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_char(v).map_err(s2d)
    }

    fn visit_str<E>(self, v: &str) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_str(v).map_err(s2d)
    }

    fn visit_string<E>(self, v: String) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_str(&v).map_err(s2d)
    }

    fn visit_unit<E>(self) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_unit().map_err(s2d)
    }

    fn visit_none<E>(self) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_none().map_err(s2d)
    }

    fn visit_some<D>(self, d: D) -> Result<S::Ok, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.0.serialize_some(&Transcoder::new(d)).map_err(s2d)
    }

    fn visit_newtype_struct<D>(self, d: D) -> Result<S::Ok, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.0
            .serialize_newtype_struct("<unknown>", &Transcoder::new(d))
            .map_err(s2d)
    }

    fn visit_seq<V>(self, mut v: V) -> Result<S::Ok, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut s = self.0.serialize_seq(v.size_hint()).map_err(s2d)?;
        while let Some(()) = v.next_element_seed(SeqSeed(&mut s))? {}
        s.end().map_err(s2d)
    }

    fn visit_map<V>(self, mut v: V) -> Result<S::Ok, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let mut s = self.0.serialize_map(v.size_hint()).map_err(s2d)?;
        while let Some(()) = v.next_key_seed(KeySeed(&mut s))? {
            v.next_value_seed(ValueSeed(&mut s))?;
        }
        s.end().map_err(s2d)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_bytes(v).map_err(s2d)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<S::Ok, E>
    where
        E: serde::de::Error,
    {
        self.0.serialize_bytes(&v).map_err(s2d)
    }
}

struct SeqSeed<'a, S: 'a>(&'a mut S);

impl<'de, S> serde::de::DeserializeSeed<'de> for SeqSeed<'_, S>
where
    S: serde::ser::SerializeSeq,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.0
            .serialize_element(&Transcoder::new(deserializer))
            .map_err(s2d)
    }
}

struct KeySeed<'a, S: 'a>(&'a mut S);

impl<'de, S> serde::de::DeserializeSeed<'de> for KeySeed<'_, S>
where
    S: serde::ser::SerializeMap,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.0
            .serialize_key(&Transcoder::new(deserializer))
            .map_err(s2d)
    }
}

struct ValueSeed<'a, S: 'a>(&'a mut S);

impl<'de, S> serde::de::DeserializeSeed<'de> for ValueSeed<'_, S>
where
    S: serde::ser::SerializeMap,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.0
            .serialize_value(&Transcoder::new(deserializer))
            .map_err(s2d)
    }
}

fn d2s<D, S>(d: D) -> S
where
    D: serde::de::Error,
    S: serde::ser::Error,
{
    S::custom(d.to_string())
}

fn s2d<S, D>(s: S) -> D
where
    S: serde::ser::Error,
    D: serde::de::Error,
{
    D::custom(s.to_string())
}
