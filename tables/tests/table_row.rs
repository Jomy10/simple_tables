mod field_names {
    #[test]
    fn const_fields() {
        use simple_tables::macros::table_row;
        
        #[table_row]
        struct Row {
            id: i32,
            name: String
        }
        
        let fields = vec!["id", "name"];
        
        assert_eq!(fields, Row::FIELDS);
    }
    
    #[test]
    fn fields() {
        use simple_tables::macros::table_row;
        
        #[table_row]
        struct Row {
            id: i32,
            name: String,
            author: String
        }
        
        let fields = ["id", "name", "author"];
        
        assert_eq!(fields.to_vec(), Row::get_fields());
    }
    
    #[test]
    fn field_count() {
        use simple_tables::macros::table_row;
        
        #[table_row]
        struct Row {
            id: i32,
            name: String,
            author: String
        }
        
        let fields = ["id", "name", "author"];
        
        assert_eq!(fields.iter().count(), Row::field_count());
    }
}

mod field_types {
    #[test]
    fn field_types() {
        use simple_tables::macros::table_row;
        
        #[table_row]
        struct Row {
            id: u32,
            name: String
        }
        
        let types = ["u32", "String"];
        assert_eq!(types.to_vec(), Row::get_field_types());
    }
    
    #[test]
    fn const_types() {
        use simple_tables::macros::table_row;
        
        #[table_row]
        struct Row {
            id: u32,
            name: String,
            boolean: bool
        }
        
        let types = ["u32", "String", "bool"];
        assert_eq!(types, Row::TYPES);
    }
}

mod other_methods {
    use simple_tables::macros::table_row;
    #[test]
    fn to_string() {
        #[table_row]
        struct TableRow {
            id: i32,
            name: String,
            char: char
        }
        
        let row = TableRow { id: 1, name: String::from("My Name"), char: 'e' };
        assert_eq!(vec!["1".to_string(), "My Name".to_string(), 'e'.to_string()], row.get_field_str());
    }
}