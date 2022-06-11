use serde::de::{Visitor, SeqAccess, self, MapAccess};
use serde_state::de::{Deserialize, DeserializeState, Deserializer};
use serde_state::ser::{Serialize, SerializeState, Serializer};
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::fmt::{Debug, self};

#[derive(Deserialize, Serialize, Debug)]
struct Inner(i32);

impl SerializeState<Cell<MyEnum>> for Inner {
    fn serialize_state<S>(&self, serializer: S, seed: &Cell<MyEnum>) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // seed.set(seed.get() + 1);
        self.serialize(serializer)
    }
}

fn serialize_inner<S>(self_: &Inner, serializer: S, seed: &Cell<MyEnum>) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // seed.set(seed.get() + 10);
    self_.serialize(serializer)
}

impl<'de, S> DeserializeState<'de, S> for Inner
where
    S: BorrowMut<MyEnum>,
{
    fn deserialize_state<D>(seed: &mut S, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // *seed.borrow_mut() += 1;
        *seed.borrow_mut() = MyEnum::Variant2;
        // deserializer.deserialize_tuple_struct("Inner", 1, deserializer);W
        Self::deserialize(deserializer)
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl SerializeState<Cell<MyEnum>> for MyEnum {
    fn serialize_state<S>(&self, serializer: S, seed: &Cell<MyEnum>) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // seed.set(seed.get() + 1);
        self.serialize(serializer)
    }
}

// impl<'de, S> DeserializeState<'de, S> for MyEnum
// where
//     S: BorrowMut<MyEnum>,
// {
//     fn deserialize_state<D>(seed: &mut S, deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         // *seed.borrow_mut() += 1;
//         *seed.borrow_mut() = MyEnum::Variant2;

//         // deserializer.deserialize_tuple_struct("Inner", 1, deserializer);W
//         Self::deserialize(deserializer)
//     }
// }

// impl<'de, S:Debug> DeserializeState<'de, S> for Struct {
//     fn deserialize_state<D>(seed: &mut S, deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>, 
//         // S: std::fmt::Debug
//         {
//             // deserializer
//             // match seed.borrow_mut() 
//             println!("{:?}",seed);

//         Ok(Struct{
//                 value: Inner(123),
//                 value2: MyEnum::Variant1,
//             })
//         // todo!()
//     }
// }

// trait GetType {
//     type Output;
//     fn get_value(self) -> Self::Output;
// }

// impl GetType for MyEnum {
//     type Output = MyEnum;
//     fn get_value(self) -> Self::Output {
//         self.clone()
//     }
// }


// impl<'de, S:Debug+GetType+Copy> DeserializeState<'de,S> for Struct {
//     fn deserialize_state<D>(seed: &mut S, deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>, 
//         // S: std::fmt::Debug
//         {
//             println!("{:?}",seed);
//             let x= seed.get_value() ;
//             println!("its so close ffs {:?}",x);
//             let myenum = MyEnum::Variant1;
//             std::mem::swap(&mut x, &mut myenum);

//         Ok(Struct{
//                 value: Inner(123),
//                 value2: MyEnum::Variant1,
//             })
//         // todo!()
//     }
// }


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct InnerStruct(MyEnum);

impl<'de> DeserializeState<'de,MyEnum> for Struct {
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
                    // println!("inside Visit map for Inner2 {:?} ", self.0);
                    // let inner_val = map.next_key::<u64>()?; 
                    // println!("Map Visitor {:?} ", inner_val );
                    // println!("inside Visit map for Inner2 {:?} ", self.0);
                    println!("Internal Seed{:?}",self.0); 
                    println!("Internal visit map called for InnerStruct"); 
                    let opt= map.next_entry::<String,i32>()?;
                    println!("{:?}",opt); 

                    let opt2= map.next_entry::<String,MyEnum>()?;
                    println!("{:?}",opt2); 

                    let opt3= map.next_key::<String>();
                    println!("{:?}",opt3); 
                   
                    // let opt2= map.next_entry::<String,Inner2>()?;
                    // println!("{:?}",opt2); 
                    
                    println!("End"); 
                    // const FIELDS: &'static [&'static str] = &["inner_val" ,"inner_val2" ,"inner_enum"];
                    // let output_res = deserializer.deserialize_struct("value2", FIELDS, Inner2Visitor(self.0));
                    // println!("output_res {:?}", output_res); 
                    // let output = output_res.unwrap(); 

                    let temp_inner = Inner2{ inner_val: 3, inner_val2: 4, inner_enum: MyEnum::Variant2 };

                    Ok(Struct{
                        value: 123,
                        value2: self.0,
                        value3: temp_inner
                    })
                }
                // fn visit_
            }
            
            // println!("{:?}",seed); 
            // const FIELDS: &'static [&'static str] = &["inner_val" ,"inner_val2" ,"inner_enum"];
            // let output_res = deserializer.deserialize_struct("value2", FIELDS, Inner2Visitor(*seed));
            // println!("output_res {:?}", output_res); 
            // let output = output_res.unwrap(); 
        //     let temp_inner = Inner2{ inner_val: 1, inner_val2: 2, inner_enum: MyEnum::Variant2 };
        // Ok(Struct{
        //         value: 123,
        //         value2: *seed,
        //         value3: temp_inner
        //     })

        const FIELDS: &'static [&'static str] = &["value","value2","value3"];
        deserializer.deserialize_struct("Struct", FIELDS, InnerStruct(*seed))

    }
}

struct Inner2Visitor(MyEnum);

impl<'de> DeserializeState<'de,MyEnum> for Inner2 {
    fn deserialize_state<D>(seed: &mut MyEnum, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, 
        {
            println!("Inner seed {:?}",seed);

            impl<'de> Visitor<'de> for Inner2Visitor {
                type Value = Inner2;
    
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct Inner2Visitor")
                }
    
                fn visit_map<V>(self, mut map: V) -> Result<Inner2, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    println!("inside Visit map for Inner2 {:?} ", self.0);
                    let inner_val = map.next_key::<u64>()?; 
                    println!("Map Visitor {:?} ", inner_val );
                    println!("inside Visit map for Inner2 {:?} ", self.0);

                    Ok(Inner2{
                        inner_val: 123,
                        inner_val2: 123,
                        inner_enum: self.0,
                    })
                }
            }
    
            println!("End return inside Visit map for Inner2 {:?} ", *seed);
        
            // const FIELDS: &'static [&'static str] = &["value","value2","value3"];
            // deserializer.deserialize_struct("Struct", FIELDS, Inner2Visitor(*seed))
            Ok(Inner2{
                inner_val: 321,
                inner_val2: 321,
                inner_enum: *seed,
            })
    }
}


#[derive(SerializeState,Debug)]
#[serde(serialize_state = "Cell<MyEnum>")]
// `de_parameters` can be used to specify additional type parameters for the derived instance
#[serde(de_parameters = "S")]
#[serde(bound(deserialize = "S: MyEnum"))]
#[serde(deserialize_state = "S")]
struct Struct {
    // The `serialize_state` attribute must be specified to use seeded serialization
    // #[serde(serialize_state)]
    // The `deserialize_state` attribute must be specified to use seeded deserialization
    // #[serde(deserialize_state)]
    value: i32,
    // value: Inner,

    // The `seed` attribute can be used to specify `deserialize_state` and `serialize_state`
    // simultaneously
    #[serde(state)]
    // #[serde(deserialize_state)]
    value2: MyEnum,

    #[serde(state)]
    value3: Inner2,

    // // If no attributes are specified then normal serialization and/or deserialization is used
    // value3: Inner,

    // // The `[de]serialize_state_with` attribute can be used to specify a custom function which
    // // does the serialization or deserialization
    // #[serde(serialize_state_with = "serialize_inner")]
    // value4: Inner,
}

#[derive(SerializeState,Debug)]
#[serde(serialize_state = "Cell<MyEnum>")]
// `de_parameters` can be used to specify additional type parameters for the derived instance
#[serde(de_parameters = "Z")]
#[serde(bound(deserialize = "Z: MyEnum"))]
#[serde(deserialize_state = "Z")]
struct Inner2 {
    inner_val: u64, 
    inner_val2: i32,
    #[serde(state)]
    inner_enum: MyEnum
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
// impl<'de, S> DeserializeState<'de, S> for DummyType
// where
//     S: BorrowMut<i32>,
// {
//     fn deserialize_state<D>(seed: &mut S, deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         *seed.borrow_mut() += 1;
//         Self::deserialize(deserializer)
//     }
// }

// #[derive(Deserialize, Serialize)]
// struct DummyType;

// #[derive(DeserializeState,SerializeState)]
// // `serialize_state` or `deserialize_state` is necessary to tell the derived implementation which
// // seed that is passed
// // #[serde(serialize_state = "Cell<i32>")]
// #[serde(de_parameters = "Z")]
// #[serde(bound(deserialize = "Z: BorrowMut<i32>"))]
// #[serde(deserialize_state = "Z")]
// struct MyType {
//     foo: String,

//     #[serde(serialize_state)]
//     #[serde(deserialize_state)]
//     my_enum: MyEnum,
// }

#[derive(serde::Serialize,serde::Deserialize,Copy,Clone,Debug)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}


pub fn main() {
    let struct_s = Struct {
        value: 0,
        // value2: Inner(0),
        // value3: Inner(1),
        // value4: Inner(0),
        value2: MyEnum::Variant1,
        value3: Inner2 {
            inner_val: 1, 
            inner_val2: 2,
            inner_enum: MyEnum::Variant1
        }
    };

    let mut buffer = Vec::new();
    {
        let mut serializer = serde_json::Serializer::new(&mut buffer);
        let seed = Cell::new(MyEnum::Variant1);
        // let seed = Cell::new(0);
        struct_s.serialize_state(&mut serializer, &seed).unwrap();
        // assert_eq!(seed.get(), 12);
        println!("{:?}", seed.get());
        println!("Buffer {:?}", buffer);
        let string_buff = String::from_utf8(buffer.clone()).unwrap();
        println!("String Buffer {} ",string_buff);
        println!("Buffer ========");

        
    }

    {

        let mut deserializer = serde_json::Deserializer::from_slice(&buffer);
        // let mut seed = 0;
        let mut seed_enum = MyEnum::Variant2;
        println!("Input ! {:?}", struct_s);
        let x = Struct::deserialize_state(&mut seed_enum, &mut deserializer).unwrap();
        // println!("{:?}", seed_enum);
        println!("Output {:?}", x);
        // assert_eq!(seed, 2);
    }

    //////////////////////////////////////////////////////

    {
        // let raw_json = r#"
        // {
        //     "foo":"bar"
        // }"#;
        // let mut deserializer = serde_json::Deserializer::from_str(&raw_json);

        // let mut seed = 0;
        // let x= MyType::deserialize_state(&mut seed, &mut deserializer);
        // println!();
    }
    // ::<MyType>
}
