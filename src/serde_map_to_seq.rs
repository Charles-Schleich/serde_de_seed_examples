use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

enum DataTypes {
    Stri,
    Numb,
}

#[derive(Debug)]
enum Value {
    Stri(String),
    Numb(f64),
}

struct XD<'a>(&'a [DataTypes]);
impl<'de, 'a> DeserializeSeed<'de> for XD<'a> {
    type Value = X;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct XV<'a>(&'a [DataTypes]);
        impl<'de, 'a> Visitor<'de> for XV<'a> {
            type Value = X;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an object containing only x")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let k = map.next_key::<String>()?;
                if k.as_deref() != Some("x") {
                    return Err(serde::de::Error::custom(format!(
                        "Expected key x, got {:?}",
                        k
                    )));
                }
                Ok(X {
                    x: map.next_value_seed(TransposeVecs(self.0))?,
                })
            }
        }

        Ok(deserializer.deserialize_struct("X", &["x"], XV(self.0))?)
    }
}

struct TransposeVecs<'a>(&'a [DataTypes]);
impl<'de, 'a> DeserializeSeed<'de> for TransposeVecs<'a> {
    type Value = Vec<Vec<Value>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransposeVecsVisitor<'a>(&'a [DataTypes]);
        impl<'de, 'a> Visitor<'de> for TransposeVecsVisitor<'a> {
            type Value = Vec<Vec<Value>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of arrays")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Vec<Vec<Value>>, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                vec.resize_with(self.0.len(), || vec![]);
                while let Some(()) = seq.next_element_seed(ExtendVecs(&mut vec, self.0))? {}
                Ok(vec)
            }
        }

        Ok(deserializer.deserialize_seq(TransposeVecsVisitor(self.0))?)
    }
}

struct ExtendVecs<'a>(&'a mut Vec<Vec<Value>>, &'a [DataTypes]);
impl<'de, 'a> DeserializeSeed<'de> for ExtendVecs<'a> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExtendVecVisitor<'a>(&'a mut Vec<Vec<Value>>, &'a [DataTypes]);

        impl<'de, 'a> Visitor<'de> for ExtendVecVisitor<'a> {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of integers")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
            where
                A: SeqAccess<'de>,
            {
                fn too_short<T, E: serde::de::Error, X>(
                    t: &[X],
                    x: Result<Option<T>, E>,
                ) -> Result<T, E> {
                    match x {
                        Err(e) => Err(e),
                        Ok(None) => Err(serde::de::Error::custom(format!(
                            "Too short: Expected exactly {} elements",
                            t.len()
                        ))),
                        Ok(Some(x)) => Ok(x),
                    }
                }
                for (i, typ) in self.1.iter().enumerate() {
                    match typ {
                        DataTypes::Stri => self.0[i].push(Value::Stri(too_short(
                            self.1,
                            seq.next_element::<String>(),
                        )?)),
                        DataTypes::Numb => self.0[i]
                            .push(Value::Numb(too_short(self.1, seq.next_element::<f64>())?)),
                    }
                }
                if seq.next_element::<serde_json::Value>()?.is_some() {
                    return Err(serde::de::Error::custom(format!(
                        "Too long: Expected exactly {} elements",
                        self.1.len()
                    )));
                }
                Ok(())
            }
        }

        deserializer.deserialize_seq(ExtendVecVisitor(self.0, self.1))
    }
}

#[derive(Debug)]
struct X {
    x: Vec<Vec<Value>>,
}

pub fn main() {
    let types = &[DataTypes::Numb, DataTypes::Stri];
    let json_raw = r#"{"x": [[1, "a"], [2, "b"]]}"#;

    let deserializer = &mut serde_json::Deserializer::from_str(json_raw);
    println!(
        "{:?}",
        XD(types).deserialize(deserializer)
    );
}