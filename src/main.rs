use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
// use serde_json::{Result, Value};
use std::str::FromStr;

mod seed_structs_example;

mod seed_vec_example;

#[derive(serde::Deserialize)]
struct MyType {
    foo: String,
    my_enum: MyEnum,
}

#[derive(serde::Deserialize)]
enum MyEnum {
    Variant1,
    Variant2,
}


// // A DeserializeSeed implementation that uses stateful deserialization to
// // append array elements onto the end of an existing vector. The preexisting
// // state ("seed") in this case is the Vec<T>. The `deserialize` method of
// // `ExtendVec` will be traversing the inner arrays of the JSON input and
// // appending each integer into the existing Vec.
// struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);

// impl<'de, 'a, T> DeserializeSeed<'de> for ExtendVec<'a, T>
// where
//     T: Deserialize<'de>,
// {
//     // The return type of the `deserialize` method. This implementation
//     // appends onto an existing vector but does not create any new data
//     // structure, so the return type is ().
//     type Value = ();

//     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         // Visitor implementation that will walk an inner array of the JSON
//         // input.
//         struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);

//         impl<'de, 'a, T> Visitor<'de> for ExtendVecVisitor<'a, T>
//         where
//             T: Deserialize<'de>,
//         {
//             type Value = ();

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 write!(formatter, "an array of integers")
//             }

//             fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
//             where
//                 A: SeqAccess<'de>,
//             {
//                 // Visit each element in the inner array and push it onto
//                 // the existing vector.
//                 while let Some(elem) = seq.next_element()? {
//                     self.0.push(elem);
//                 }
//                 Ok(())
//             }
//         }

//         deserializer.deserialize_seq(ExtendVecVisitor(self.0))
//     }
// }

// // Visitor implementation that will walk the outer array of the JSON input.
// struct FlattenedVecVisitor<T>(PhantomData<T>);

// impl<'de, T> Visitor<'de> for FlattenedVecVisitor<T>
// where
//     T: Deserialize<'de>,
// {
//     // This Visitor constructs a single Vec<T> to hold the flattened
//     // contents of the inner arrays.
//     type Value = Vec<T>;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(formatter, "an array of arrays")
//     }

//     fn visit_seq<A>(self, mut seq: A) -> Result<Vec<T>, A::Error>
//     where
//         A: SeqAccess<'de>,
//     {
//         // Create a single Vec to hold the flattened contents.
//         let mut vec = Vec::new();

//         // Each iteration through this loop is one inner array.
//         while let Some(()) = seq.next_element_seed(ExtendVec(&mut vec))? {
//             // Nothing to do; inner array has been appended into `vec`.
//         }

//         // Return the finished vec.
//         Ok(vec)
//     }
// }

// struct Elems{
//     elem: Vec<u64>
// }

fn main() {
    println!("Hello, world!");

    // 
    seed_structs_example::main();
    seed_vec_example::main();

    // let raw_json = r#"
    //     {
    //         "foo":"bar"
    //     }"#;
    // let res_MyType_withVariant1 = serde_json::from_str::<MyType>(&raw_json);
    // let res_MyType_withVariant2 = serde_json::from_str::<MyType>(&raw_json);

    // let raw_json = r#"
    // {
    //     "elems": [[1, 2], [3, 4, 5], [6]]
    // }
    // "#;
    // let x= serde_json::from_str::<Elems>(raw_json);
    // let visitor = FlattenedVecVisitor(PhantomData);
    // let flattened: Vec<u64> = deserializer.deserialize_seq(visitor)?;

}

