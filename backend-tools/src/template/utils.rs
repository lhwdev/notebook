use std::marker::PhantomData;

use serde::{Deserialize, de::DeserializeSeed};

pub struct NextValue<'de, 'a, T : serde::de::MapAccess<'de>>(&'a mut T, PhantomData<fn(&'de ())>);

impl <'de, 'a, T : serde::de::MapAccess<'de>> NextValue<'de, 'a, T> {
    #[inline]
    pub fn new(access: &'a mut T) -> Self {
        Self(access, PhantomData)
    }
    
    #[inline]
    pub fn get<V>(&mut self) -> Result<V, T::Error>
    where
        V: Deserialize<'de>,
    {
        self.0.next_value()
    }

    #[inline]
    pub fn with_seed<V>(&mut self, seed: V) -> Result<V::Value, T::Error>
    where
        V: DeserializeSeed<'de>
    {
        self.0.next_value_seed(seed)
    }
}

macro_rules! map_toml_visitor {
    (
        impl Visitor for $name:ident {
            $(fn $fn_name:ident ($($param:ident),*) $body:block)+
        }
    ) => {
        impl<'de, 'a> serde::de::Visitor<'de> for $name<'a> {
            type Value = toml::Value;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("any valid TOML value")
            }

            fn visit_map<V>(self, mut __visitor: V) -> Result<toml::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut __map = std::collections::BTreeMap::new();
                let __data = self.0;
                $(
                    $crate::template::utils::macros::$fn_name!((__visitor, __map, __data), ($($param),*) $body);
                )+
                Ok(toml::Value::Table(__map))
            }
        }
    }
}

#[macro_use]
pub mod macros {
    macro_rules! extra {
        (($visitor:ident, $my_map:ident, $my_data:ident), ($map:ident, $data:ident) $body:block) => {
            {
                let $map = &mut $my_map;
                let $data = $my_data;
                $body
            }
        };
    }
    pub(crate) use extra;

    macro_rules! map_each_entry {
        (($visitor:ident, $my_map:ident, $my_data:ident), ($data:ident, $key:ident, $value:ident) $body:block) => {
            let $data = $my_data;
            while let Some($key) = $visitor.next_key::<String>()? {
                if $my_map.contains_key(&$key) {
                    let msg = format!("duplicate key: `{}`", $key);
                    return Err(serde::de::Error::custom(msg));
                }

                let mut $value = $crate::template::utils::NextValue::new(&mut $visitor);
                let (key, value) = $body;
                $my_map.insert(key, value);
            }
        }
    }
    pub(crate) use map_each_entry;

    macro_rules! transform_each_entry {
        (($visitor:ident, $my_map:ident, $my_data:ident), ($map:ident, $data:ident, $key:ident, $value:ident) $body:block) => {
            {
                let $map = &mut $my_map;
                let $data = $my_data;
                while let Some($key) = $visitor.next_key::<String>()? {
                    if $map.contains_key(&$key) {
                        let msg = format!("duplicate key: `{}`", $key);
                        return Err(serde::de::Error::custom(msg));
                    }

                    let mut $value = $crate::template::utils::NextValue::new(&mut $visitor);
                    $body
                }
            }
        }
    }
    pub(crate) use transform_each_entry;
}

pub(crate) use map_toml_visitor;
