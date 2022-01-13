
// #[test]
// fn hello_world_fields() {
//     use simple_tables_derive::hello_world_macro;
//     struct TableRow {
//         id: i32,
//         field1: String
//     }
//     #[hello_world_macro(rows = TableRow)]
//     struct HelloWorldStruct {
//         field1: String,
//         field2: i32,
//         field3: usize,
//         field4: bool
//     }
//
//     let fields: [&str; 4] = ["field1", "field2", "field3", "field4"];
//     for (i, field) in fields.iter().enumerate() {
//         assert_eq!(field.to_owned(), HelloWorldStruct::FIELDS[i]);
//     }
// }

mod table;
mod table_row;
mod error;
// mod table_row;