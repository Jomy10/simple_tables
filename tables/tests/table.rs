mod table_trait {
    use simple_tables::macros::table;
    use simple_tables::macros::table_row;
    use simple_tables::core::Table;
    #[test]
    fn new_is_empty() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        #[table(rows = TableRow)]
        struct ATable {}
        
        assert!(ATable::new().get_rows().is_empty());
    }
    
    #[test]
    fn from_vec() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        
        #[table(rows = TableRow)]
        struct ATable {}
        
        let vec: Vec<TableRow> = vec![TableRow { id: 1, name: String::from("Jomy") }, TableRow { id: 2, name: String::from("David") }];
        let table = ATable::from_vec(&vec);
        
        assert_eq!(&vec, table.get_rows())
    }
    
    #[test]
    fn push_row() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        
        #[table(rows = TableRow)]
        struct ATable {}
        
        // Use this syntax to get full code completion
        let mut vec: Vec<TableRow> = vec![TableRow { id: 1, name: String::from("Jomy") }, TableRow { id: 2, name: String::from("David") }];
        let mut table = ATable::from_vec(&vec);
        
        let row_to_add = TableRow { id: 3, name: String::from("Richard Wright") };
        vec.push(row_to_add.clone());
        table.push(row_to_add.clone());
        
        assert_eq!(&vec, table.get_rows());
        assert!(table.get_rows().contains(&row_to_add))
    }
    
    #[test]
    fn insert_top() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        
        #[table(rows = TableRow)]
        struct ATable {}
        
        // Use this syntax to get full code completion
        let mut vec: Vec<TableRow> = vec![TableRow { id: 1, name: String::from("Jomy") }, TableRow { id: 2, name: String::from("David") }];
        let mut table = ATable::from_vec(&vec);
        
        let row_to_add = TableRow { id: 3, name: String::from("Richard Wright") };
        vec.insert(0, row_to_add.clone());
        table.insert_top(row_to_add.clone());
        
        assert_eq!(&vec, table.get_rows());
        assert!(table.get_rows().contains(&row_to_add))
    }
    
    #[test]
    fn insert() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        
        #[table(rows = TableRow)]
        struct ATable {}
        
        let mut vec: Vec<TableRow> = vec![TableRow { id: 1, name: String::from("Jomy") }, TableRow { id: 2, name: String::from("David") }];
        let mut table = ATable::from_vec(&vec);
        
        let row_to_add = TableRow { id: 3, name: String::from("Richard Wright") };
        vec.insert(1, row_to_add.clone());
        table.insert(1, row_to_add.clone());
        
        assert_eq!(&vec, table.get_rows());
        assert!(table.get_rows().contains(&row_to_add));
    }
    
    #[test]
    fn get_column() {
        #[table_row]
        struct TableRow {
            id: u32,
            name: String
        }
        
        #[table(rows = TableRow)]
        struct MyTable {}
        
        let vec: Vec<TableRow> = vec![TableRow{id: 1, name: String::from("A")}, TableRow{id: 2, name: String::from("B")}];
        let table = MyTable::from_vec(&vec);
        
        let ids: Vec<u32> = table.get_column(|row| row.id);
        assert_eq!(vec![1,2], ids);
    }
    
    #[test]
    fn get_column_sizes() {
        #[table_row]
        struct TableRow {
            id: u32,
            name: String
        }
    
        #[table(rows = TableRow)]
        struct MyTable {}
    
        let vec: Vec<TableRow> = vec![TableRow{id: 1000, name: String::from("Abc")}, TableRow{id: 2, name: String::from("Bd")}];
        let table = MyTable::from_vec(&vec);
        
        assert_eq!(3, table.get_column_size(|row| row.name.clone()).unwrap());
        assert_eq!(4, table.get_column_size(|row| row.id).unwrap());
    }
    
    #[test]
    fn rm_row_at() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        
        #[table(rows = TableRow)]
        #[derive(Debug)]
        struct MyTable {}
        
        impl PartialEq for MyTable {
            fn eq(&self, other: &Self) -> bool {
                let results: Vec<bool> = self.rows.iter().enumerate().map(|(i, row)| {
                    return other.get_row_at(i).unwrap() == row;
                }).collect();
                return if results.contains(&false) {
                    false
                } else {
                    true
                }
            }
        }
        
        let vec: Vec<TableRow> = vec![
            TableRow { id: 0, name: "Ritchie Blackmore".to_string() },
            TableRow { id: 1, name: "The Rolling Stones".to_string() }
        ];
        let rm_should_be = TableRow { id: 0, name: "Ritchie Blackmore".to_string() };
        let left = vec![TableRow { id: 1, name: "The Rolling Stones".to_string() }];
        
        let mut table: MyTable = MyTable::from_vec(&vec);
        let left_in_table = MyTable::from_vec(&left);
        let removed_row = table.rm_row_at(0);
        
        assert_eq!(removed_row, rm_should_be);
        assert_eq!(table, left_in_table)
    }
}

mod uid {
    use simple_tables::macros::table;
    use simple_tables::macros::table_row;
    use simple_tables::core::Table;
    use simple_tables_core::IdTable;
    
    #[test]
    fn get_row() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        
        #[table(rows = TableRow)]
        struct ATable {}
        
        impl IdTable<i32, TableRow> for ATable {
            fn get_id_from_row(row: &TableRow) -> i32 {
                row.id
            }
        }
        
        let vec = vec![ TableRow { id: 1, name: String::from("J")}, TableRow { id: 2, name: String::from("T")}, TableRow { id: 3, name: String::from("A")} ];
        let table = ATable::from_vec(&vec);
        assert_eq!(vec[1], table.get_row(2).unwrap().clone());
    }
    
    #[test]
    fn get_row_mut() {
        
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
    
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
    
        #[table(rows = TableRow)]
        struct ATable {}
    
        impl IdTable<i32, TableRow> for ATable {
            fn get_id_from_row(row: &TableRow) -> i32 {
                row.id
            }
        }
    
        let vec = vec![ TableRow { id: 1, name: String::from("J")}, TableRow { id: 2, name: String::from("T")}, TableRow { id: 3, name: String::from("A")} ];
        let table = ATable::from_vec(&vec);
        let mut table2: ATable = ATable::from_vec(&vec);
        assert_eq!(table2.get_row_mut(2).unwrap().clone(), table.get_row(2).unwrap().clone());
    }
    
    #[test]
    fn get_row_mut_can_edit() {
        
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
        
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        
        #[table(rows = TableRow)]
        struct ATable {}
        
        impl IdTable<i32, TableRow> for ATable {
            fn get_id_from_row(row: &TableRow) -> i32 {
                row.id
            }
        }
        
        let vec = vec![
            TableRow { id: 1, name: String::from("J")},
            TableRow { id: 2, name: String::from("T")},
            TableRow { id: 3, name: String::from("A")}
        ];
        let vec_unedited = vec![
            TableRow { id: 1, name: String::from("J")},
            TableRow { id: 2, name: String::from("T")},
            TableRow { id: 3, name: String::from("B")}
        ];
        let table = ATable::from_vec(&vec);
        let mut table2: ATable = ATable::from_vec(&vec_unedited);
        let row = table2.get_row_mut(3).unwrap();
        row.name = "A".to_string();
        assert_eq!(table2.get_rows(), table.get_rows());
    }
    
    #[test]
    fn rm_row() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
        }
    
        impl PartialEq<Self> for TableRow {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
    
        #[table(rows = TableRow)]
        #[derive(Debug, PartialEq)]
        struct MyTable {}
    
        
    
        impl IdTable<i32, TableRow> for MyTable {
            fn get_id_from_row(row: &TableRow) -> i32 {
                row.id
            }
        }
        
        let vec: Vec<TableRow> = vec![
            TableRow { id: 0, name: "Ritchie Blackmore".to_string() },
            TableRow { id: 1, name: "The Rolling Stones".to_string() }
        ];
        let rm_should_be = TableRow { id: 0, name: "Ritchie Blackmore".to_string() };
        let left_vec = vec![TableRow { id: 1, name: "The Rolling Stones".to_string() }];
        
        let mut table: MyTable = MyTable::from_vec(&vec);
        let removed_row = table.rm_row_at(0);
        
        let table_left: MyTable = MyTable::from_vec(&left_vec);
        
        assert_eq!(rm_should_be, removed_row);
        assert_eq!(table_left, table)
    }
}

mod to_string {
    use simple_tables::macros::table;
    use simple_tables::macros::table_row;
    use simple_tables::core::Table;
    
    #[test]
    fn to_string() {
        #[table_row]
        struct TableRow {
            id: u32,
            name: String
        }
        
        #[table(rows = TableRow)]
        struct MyTable {}
        
        let vec: Vec<TableRow> = vec![TableRow{id: 1000, name: String::from("Abc")}, TableRow{id: 2, name: String::from("Bd")}];
        let table: MyTable = MyTable::from_vec(&vec);
        
        // table.to_string();
        let ascii_table = "\
+------+------+
| id   | name |
+======+======+
| 1000 | Abc  |
+------+------+
| 2    | Bd   |
+------+------+";
        assert_eq!(ascii_table, table.to_string())
    }
    
    #[test]
    fn to_string2() {
        #[table_row]
        struct TableRow {
            id: u64,
            name: String,
            email: String
        }
        
        #[table(rows = TableRow)]
        struct MyTable {}
        
        let vec: Vec<TableRow> = vec![
            TableRow{id: 425549252244, name: "Nothing But Thieves".to_string(), email: "info@nbt.com".to_string()},
            TableRow{id: 34459529244554252, name: "David Bowie".to_string(), email: "info@bowie.com".to_string()},
            TableRow{id: 45052024, name: "Slipknot".to_string(), email: "info@slipknot.com".to_string()}
        ];
        let table: MyTable = MyTable::from_vec(&vec);
        
        // table.to_string();
        let ascii_table = "\
+-------------------+---------------------+-------------------+
| id                | name                | email             |
+===================+=====================+===================+
| 425549252244      | Nothing But Thieves | info@nbt.com      |
+-------------------+---------------------+-------------------+
| 34459529244554252 | David Bowie         | info@bowie.com    |
+-------------------+---------------------+-------------------+
| 45052024          | Slipknot            | info@slipknot.com |
+-------------------+---------------------+-------------------+";
        assert_eq!(ascii_table, table.to_string())
    }
}