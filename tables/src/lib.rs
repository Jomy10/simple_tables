//! The simple tables crate allows you to easily create table structures.
//!
//! # Getting started
//! You can use the macros [`table_row`](crate::macros::table_row) and [`table`](crate::macros::table)
//! to make a new table with the row structure defined by your table rows.
//!
//! **Example**
//! ```rust
//! # use simple_tables::macros::{table_row, table};
//!
//! #[table_row]
//! struct MyTableRow {
//!   id: u32,
//!   name: String,
//!   email: String,
//!   address: String
//! }
//!
//! #[table(rows = MyTableRow)]
//! struct MyTable {}
//! ```
//!
//! # Examples
//! ## Printing out a table
//!
//! You can use the `to_string()` method to convert a table to a formatted text table
//! and then print it out.
//!
//! ```rust
//! # use simple_tables::Table;
//! # use simple_tables::macros::{table_row, table};
//! #
//! # #[table_row]
//! # struct MyTableRow {
//! #   id: u32,
//! #   name: String,
//! #   email: String,
//! #   address: String
//! # }
//! #
//! # #[table(rows = MyTableRow)]
//! # struct MyTable {}
//!
//! let rows: Vec<MyTableRow> = vec![
//!   MyTableRow{ id: 0, name: "David Bowie".to_string(), email: "david@bowie.com".to_string(), address: "England".to_string()},
//!   MyTableRow{ id: 1, name: "David Gilmour".to_string(), email: "david@gilmour.com".to_string(), address: "England".to_string()},
//!   MyTableRow{ id: 2, name: "Opeth".to_string(), email: "info@opeth.com".to_string(), address: "Sweden".to_string()},
//!   MyTableRow{ id: 3, name: "The Beatles".to_string(), email: "info@beatles.com".to_string(), address: "England".to_string()}
//! ];
//!
//! let table = MyTable::from_vec(&rows);
//! let s = table.to_string();
//! println!("{}", s);
//! ```
//!
//! The output will be:
//! ```bash
//! +----+---------------+-------------------+---------+
//! | id | name          | email             | address |
//! +====+===============+===================+=========+
//! | 0  | David Bowie   | david@bowie.com   | England |
//! +----+---------------+-------------------+---------+
//! | 1  | David Gilmour | david@gilmour.com | England |
//! +----+---------------+-------------------+---------+
//! | 2  | Opeth         | info@opeth.com    | Sweden  |
//! +----+---------------+-------------------+---------+
//! | 3  | The Beatles   | info@beatles.com  | England |
//! +----+---------------+-------------------+---------+
//! ```
//!
//! More examples can be found on [GitHub](https://github.com/jomy10/simple_tables).
//!
//! # Traits
//! - [Table](crate::Table)
//! - [TableRow](crate::TableRow)
//! - [IdTable](crate::IdTable)
//!
//! # Macros
//! - [table_row](crate::macros::table_row)
//! - [table](crate::macros::table)


// Core libraries
pub extern crate simple_tables_core as core;
pub extern crate simple_tables_derive as derive;

// Library structure
pub mod macros {
    pub use derive::table_row as table_row;
    pub use derive::table as table;
}

pub use core::Table;
pub use core::TableRow;
pub use core::IdTable;