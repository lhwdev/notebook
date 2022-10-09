use std::{collections::BTreeMap, marker::PhantomData};
use serde::de::DeserializeSeed;
use toml::*;

pub struct TomlSerdeVisitor<'de, T: TomlVisitor<'de>>(T, PhantomData<fn(&'de ())>);

impl <'de, T: TomlVisitor<'de>> TomlSerdeVisitor<'de, T> {
    pub fn new(base: T) -> Self {
        Self(base, PhantomData)
    }
}

impl<'de, T: TomlVisitor<'de>> serde::de::Visitor<'de> for TomlSerdeVisitor<'de, T> {
    type Value = toml::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_bool(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_i64(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_u64(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_u32(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_i32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_f64(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.0.visit_string(v)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.0.visit_some(deserializer)
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        self.0.visit_seq(visitor)
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        self.0.visit_map(visitor)
    }
}

pub trait TomlVisitor<'de>: Sized {
    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("any valid TOML value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Boolean(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Integer(value))
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        if value <= i64::max_value() as u64 {
            Ok(Value::Integer(value as i64))
        } else {
            Err(serde::de::Error::custom("u64 value was too large"))
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Integer(value.into()))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Integer(value.into()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(value.into()))
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(value))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        serde::de::Deserialize::deserialize(deserializer)
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }
        Ok(Value::Array(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let mut map = BTreeMap::new();
        while let Some(key) = visitor.next_key()? {
            if map.contains_key(&key) {
                let msg = format!("duplicate key: `{}`", key);
                return Err(serde::de::Error::custom(msg));
            }
            map.insert(key, visitor.next_value()?);
        }
        Ok(Value::Table(map))
    }
}

pub struct DatetimeOrTable<'a> {
    pub key: &'a mut String,
}

impl<'a, 'de> serde::de::DeserializeSeed<'de> for DatetimeOrTable<'a> {
    type Value = bool;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'a, 'de> serde::de::Visitor<'de> for DatetimeOrTable<'a> {
    type Value = bool;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string key")
    }

    fn visit_str<E>(self, s: &str) -> Result<bool, E>
    where
        E: serde::de::Error,
    {
        // if s == datetime::FIELD {
        //     Ok(true)
        // } else {
        self.key.push_str(s);
        Ok(false)
        // }
    }

    fn visit_string<E>(self, s: String) -> Result<bool, E>
    where
        E: serde::de::Error,
    {
        // if s == toml::datetime::FIELD {
        //     Ok(true)
        // } else {
        *self.key = s;
        Ok(false)
        // }
    }
}

pub struct MapDeserializeSeed<'de, V : serde::de::Visitor<'de, Value = Value>>(V, PhantomData<fn(&'de ())>);

impl <'de, V : serde::de::Visitor<'de, Value = Value>> MapDeserializeSeed<'de, V> {
    pub fn new(visitor: V) -> Self {
        Self(visitor, PhantomData)
    }
}

impl <'de, V : serde::de::Visitor<'de, Value = Value>> DeserializeSeed<'de> for MapDeserializeSeed<'de, V> {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(self.0)
    }
}
