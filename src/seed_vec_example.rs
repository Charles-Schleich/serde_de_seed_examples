extern crate serde;
extern crate serde_json;

use std::fmt;
// use serde::de::{Deserialize, DeserializeSeed, Deserializer, Visitor, SeqAccess};



// pub struct Elem {
//     pub thing:String, 
//     pub a_vec:Vec<u64>,
//     pub b_vec:Vec<u64>,
// }


// struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);

// impl<'a, T> DeserializeSeed<'a> for ExtendVec<'a, T>
//     where T: Deserialize<'a>
// {
//     type Value = ();

//     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//         where D: Deserializer<'a>
//     {
//         struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);

//         impl<'a, T> Visitor<'a> for ExtendVecVisitor<'a, T>
//             where T: Deserialize<'a>
//         {
//             type Value = ();

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("an array")
//             }

//             fn visit_seq<V>(self, mut visitor: V) -> Result<(), V::Error>
//                 where V: SeqAccess<'a>
//             {
//                 while let Some(elem) = visitor.next_element()? {
//                     self.0.push(elem);
//                 }
//                 Ok(())
//             }
//         }

//         deserializer.deserialize_seq(ExtendVecVisitor(self.0))
//     }
// }

pub fn main() {
    // todo try use this with initialized Elem rather
    // let mut vec = vec![1, 2, 3];
    // let mut deserializer = serde_json::Deserializer::from_str("[4, 5, 6]");
    // ExtendVec(&mut vec).deserialize(&mut deserializer).unwrap();
    // println!("{:?}", vec);
}