use serde::de::IgnoredAny;
use serde::de::value::MapAccessDeserializer;
use serde_state::de::{MapAccess, Visitor};

use serde_state::de::{Deserialize, DeserializeState, Deserializer};
use serde_state::ser::{Serialize, SerializeState, Serializer};

use std::cell::Cell;
use std::fmt::{self, Debug};


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl SerializeState<Cell<MyEnum>> for MyEnum {
    fn serialize_state<S>(&self, serializer: S, seed: &Cell<MyEnum>) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialize(serializer)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ███████ ████████ ██████  ██    ██  ██████ ████████     ██████  ███████     ███████ ████████  █████  ████████ ███████
// ██         ██    ██   ██ ██    ██ ██         ██        ██   ██ ██          ██         ██    ██   ██    ██    ██
// ███████    ██    ██████  ██    ██ ██         ██        ██   ██ █████       ███████    ██    ███████    ██    █████
//      ██    ██    ██   ██ ██    ██ ██         ██        ██   ██ ██               ██    ██    ██   ██    ██    ██
// ███████    ██    ██   ██  ██████   ██████    ██        ██████  ███████     ███████    ██    ██   ██    ██    ███████

struct InnerStruct(MyEnum);
impl<'de> DeserializeState<'de, MyEnum> for Struct {
    fn deserialize_state<D>(seed: &mut MyEnum, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        impl<'de> Visitor<'de> for InnerStruct {
            type Value = Struct;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct InnerStruct")
            }
            
            fn visit_map<V>(self, mut map: V) -> Result<Struct, V::Error>
            where
                V: MapAccess<'de>,
            {
                println!("Struct DE With State...");
                println!("  Seed {:?}", self.0);
                println!("  visit map called for InnerStruct");
                let (k, val) = map.next_entry::<String, i32>()?.unwrap();
                println!("  k {}  val: {:?}", k, val);

                let (_, val2) = map.next_entry::<String, MyEnum>()?.unwrap();
                println!("  {:?}", val2);

                // TODO: How do i get this map.next_entry to call DeserializeState of Inner2?
                // working with normal Inner2 Deserializer
                // let (opt3, inner_val_inner) = map.next_entry::<String, Inner2>()?.unwrap();
                
                // Attempts
                let ma_de = MapAccessDeserializer::new(map);
                let mut seed = self.0;
                let inner_val_inner = Inner2::deserialize_state(&mut seed, ma_de)?;


                // let inner_val_inner = map.next_value::<Inner2>()?;

                // Inner2::deserialize_state(&mut seed, &mut deserializer.clone()).unwrap();
                // let mut deserializer = serde_json::Deserializer::from_slice(&buffer);

                // // let mut seed = 0;
                // let mut seed_enum = MyEnum::Variant2;
                // println!("Input ! {:?}", struct_s);
                // let x = Struct::deserialize_state(&mut seed_enum, &mut deserializer).unwrap();
                // println!("Output {:?}", x);


                // println!("  {:?}", opt3);
                println!("... Struct DE With State End");

                Ok(Struct {
                    val,
                    val2: seed,
                    val3: inner_val_inner,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["val", "val2", "val3"];
        deserializer.deserialize_struct("Struct", FIELDS, InnerStruct(*seed))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ██ ███    ██ ███    ██ ███████ ██████      ██████  ███████ 
// ██ ████   ██ ████   ██ ██      ██   ██     ██   ██ ██      
// ██ ██ ██  ██ ██ ██  ██ █████   ██████      ██   ██ █████   
// ██ ██  ██ ██ ██  ██ ██ ██      ██   ██     ██   ██ ██      
// ██ ██   ████ ██   ████ ███████ ██   ██     ██████  ███████ 

struct Inner2Visitor2;
impl<'de> Deserialize<'de> for Inner2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        impl<'de> Visitor<'de> for Inner2Visitor2 {
            type Value = Inner2;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Inner2Visitor")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Inner2, V::Error>
            where
                V: MapAccess<'de>,
            {
                println!("Inner DE... ");
                println!("      Inside visit_Map Inner2Visitor2 ");
                let inner_val;
                if let Some((_, inner_val_inner)) = map.next_entry::<String, u64>()? {
                    inner_val = inner_val_inner;
                } else {
                    return Err(serde::de::Error::custom(
                        "Could not deserialize inner_val field as u64",
                    ));
                };

                ///////////////////////
                let inner_val2;
                if let Some((_, inner_val2_inner)) = map.next_entry::<String, i32>()? {
                    inner_val2 = inner_val2_inner;
                } else {
                    return Err(serde::de::Error::custom(
                        "Could not deserialize inner_val2 field",
                    ));
                };

                // Ignore the rest of the elements in the map
                while let Some(_) = map.next_entry::<IgnoredAny, IgnoredAny>()? {};
                println!("... Inner DE End ");

                Ok(Inner2 {
                    inner_val,
                    inner_val2,
                    inner_enum: MyEnum::Variant4,
                })
            }
        }

        // println!("Inside Inner2 normal DeserializeFunction");
        const FIELDS: &'static [&'static str] = &["inner_val", "inner_val2", "inner_enum"];
        deserializer.deserialize_struct("Inner2", FIELDS, Inner2Visitor2)
        
    }
}


// ██ ███    ██ ███    ██ ███████ ██████      ██████  ███████     ██     ██ ██ ████████ ██   ██     ███████ ████████  █████  ████████ ███████ 
// ██ ████   ██ ████   ██ ██      ██   ██     ██   ██ ██          ██     ██ ██    ██    ██   ██     ██         ██    ██   ██    ██    ██      
// ██ ██ ██  ██ ██ ██  ██ █████   ██████      ██   ██ █████       ██  █  ██ ██    ██    ███████     ███████    ██    ███████    ██    █████   
// ██ ██  ██ ██ ██  ██ ██ ██      ██   ██     ██   ██ ██          ██ ███ ██ ██    ██    ██   ██          ██    ██    ██   ██    ██    ██      
// ██ ██   ████ ██   ████ ███████ ██   ██     ██████  ███████      ███ ███  ██    ██    ██   ██     ███████    ██    ██   ██    ██    ███████ 

struct Inner2Visitor(MyEnum);
impl<'de> DeserializeState<'de, MyEnum> for Inner2 {
    fn deserialize_state<D>(seed: &mut MyEnum, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        println!("Inner DE with State");
        println!("      state {:?}", seed);

        impl<'de> Visitor<'de> for Inner2Visitor {
            type Value = Inner2;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Inner2Visitor")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Inner2, V::Error>
            where
                V: MapAccess<'de>,
            {
                println!("      inside Visit map for Inner2 With State {:?} ", self.0);
                // So now we call
                let (opt3, inner_val_inner) = map.next_entry::<String, Inner2>()?.unwrap();
                println!("      HERE {:?} ", inner_val_inner);

                // let inner_val = map.next_key::<String>()?;
                // let inner_val = map.next_key::<String>();
                // println!("      HERE2 {:?} ", inner_val);
                
                let inner_val = map.next_key::<u64>()?;
                println!("      Map Visitor {:?} ", inner_val);
                println!("      inside Visit map for Inner2 With State {:?} ", self.0);

                Ok(Inner2 {
                    inner_val: 123,
                    inner_val2: 123,
                    inner_enum: self.0,
                })
            }
        }

        println!("      End return inside Visit map for Inner2 {:?} ", *seed);

        const FIELDS: &'static [&'static str] = &["value", "value2", "value3"];
        deserializer.deserialize_struct("Struct", FIELDS, Inner2Visitor(*seed))
        // Ok(Inner2{
        //     inner_val: 321,
        //     inner_val2: 321,
        //     inner_enum: *seed,
        // })
    }
}


// ███████ ████████ ██████  ██    ██  ██████ ████████ ███████ 
// ██         ██    ██   ██ ██    ██ ██         ██    ██      
// ███████    ██    ██████  ██    ██ ██         ██    ███████ 
//      ██    ██    ██   ██ ██    ██ ██         ██         ██ 
// ███████    ██    ██   ██  ██████   ██████    ██    ███████ 

#[derive(SerializeState, Debug)]
#[serde(serialize_state = "Cell<MyEnum>")]
// `de_parameters` can be used to specify additional type parameters for the derived instance
#[serde(de_parameters = "S")]
#[serde(bound(deserialize = "S: MyEnum"))]
#[serde(deserialize_state = "S")]
struct Struct {
    val: i32,

    #[serde(state)]
    // #[serde(deserialize_state)]
    val2: MyEnum,
    
    #[serde(state)]
    val3: Inner2,
}

#[derive(SerializeState, Debug)]
#[serde(serialize_state = "Cell<MyEnum>")]
// `de_parameters` can be used to specify additional type parameters for the derived instance
#[serde(de_parameters = "Z")]
#[serde(bound(deserialize = "Z: MyEnum"))]
#[serde(deserialize_state = "Z")]
struct Inner2 {
    inner_val: u64,
    inner_val2: i32,
    #[serde(state)]
    inner_enum: MyEnum,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
    Variant4,
    Variant5,
}

// ███    ███  █████  ██ ███    ██ 
// ████  ████ ██   ██ ██ ████   ██ 
// ██ ████ ██ ███████ ██ ██ ██  ██ 
// ██  ██  ██ ██   ██ ██ ██  ██ ██ 
// ██      ██ ██   ██ ██ ██   ████ 
                                
                                
pub fn main() {

    let struct_s = Struct {
        val: 0,
        val2: MyEnum::Variant1,
        val3: Inner2 {
            inner_val: 1,
            inner_val2: 2,
            inner_enum: MyEnum::Variant1,
        },
    };

    let mut buffer = Vec::new();
    {
        let mut serializer = serde_json::Serializer::pretty(&mut buffer);
        let seed = Cell::new(MyEnum::Variant1);
        // let seed = Cell::new(0);
        struct_s.serialize_state(&mut serializer, &seed).unwrap();
        // assert_eq!(seed.get(), 12);
        println!("{:?}", seed.get());
        println!("Buffer {:?}", buffer);
        let string_buff = String::from_utf8(buffer.clone()).unwrap();
        println!("String Buffer {} ", string_buff);
        println!("Buffer ========");
    }
        println!("=======================================");
        println!("== Deserialize ==");
        println!("=======================================");

    {
        let mut deserializer = serde_json::Deserializer::from_slice(&buffer);
        let mut seed_enum = MyEnum::Variant5;
        println!("Input ! {:?}", struct_s);
        let x = Struct::deserialize_state(&mut seed_enum, &mut deserializer).unwrap();
        println!("=======================================");
     
        println!("Output {:#?}", x);
    }
}
