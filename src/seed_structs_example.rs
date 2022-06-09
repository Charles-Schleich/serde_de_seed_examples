use serde::de::{DeserializeSeed, IgnoredAny, MapAccess, Visitor};
use serde::*;
use std::fmt;

#[derive(Debug)]
enum ContentType {
    A,
    B,
    Unknown,
}

#[derive(Debug)]
enum Content {
    TypeA(String),
    TypeB(i32),
}

#[derive(Debug)]
struct Value {
    id: String,
    content: Content,
}

#[derive(Debug)]
struct Data {
    typ: String,
    value: Value,
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataVisitor;

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Data")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut typ = None;
                let mut value = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        "type" => {
                            typ = Some(access.next_value()?);
                        }
                        "value" => {
                            let seed = match typ.as_deref() {
                                Some("TypeA") => ContentType::A,
                                Some("TypeB") => ContentType::B,
                                _ => ContentType::Unknown,
                            };
                            value = Some(access.next_value_seed(seed)?);
                        }
                        _ => {
                            access.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                Ok(Data {
                    typ: typ.unwrap(),
                    value: value.unwrap(),
                })
            }
        }

        deserializer.deserialize_map(DataVisitor)
    }
}



impl<'de> DeserializeSeed<'de> for Data {
    type Value = Data;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataVisitor;

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Data")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut typ = None;
                let mut value = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        "type" => {
                            typ = Some(access.next_value()?);
                        }
                        "value" => {
                            let seed = match typ.as_deref() {
                                Some("TypeA") => ContentType::A,
                                Some("TypeB") => ContentType::B,
                                _ => ContentType::Unknown,
                            };
                            value = Some(access.next_value_seed(seed)?);
                        }
                        _ => {
                            access.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                Ok(Data {
                    typ: typ.unwrap(),
                    value: value.unwrap(),
                })
            }
        }

        deserializer.deserialize_map(DataVisitor)
    }
}


impl<'de> DeserializeSeed<'de> for ContentType {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor(ContentType);

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Value")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut content = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        "id" => {
                            id = Some(access.next_value()?);
                        }
                        "content" => {
                            content = Some(match self.0 {
                                ContentType::A => Content::TypeA(access.next_value()?),
                                ContentType::B => Content::TypeB(access.next_value()?),
                                ContentType::Unknown => {
                                    panic!("Should not happen if type happens to occur before value, but JSON is unordered.");
                                }
                            });
                        }
                        _ => {
                            access.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                Ok(Value {
                    id: id.unwrap(),
                    content: content.unwrap(),
                })
            }
        }

        deserializer.deserialize_map(ValueVisitor(self))
    }
}

pub  fn main() {
    let j = r#"{"type": "TypeA", "value": {"id": "blah", "content": "0xa1b.."}}"#;
    dbg!(serde_json::from_str::<Data>(j).unwrap());
    let j = r#"{"type": "TypeB", "value": {"id": "blah", "content": 666}}"#;
    dbg!(serde_json::from_str::<Data>(j).unwrap());
    // let j = r#"{"type": "TypeB", "value": {"id": "blah", "content": "Foobar"}}"#;
    // dbg!(serde_json::from_str::<Data>(j).unwrap_err());
}