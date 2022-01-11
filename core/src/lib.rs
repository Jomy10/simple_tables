//! Simple Tables Core

// Trait
pub trait TableRow {
    /// Returns a vector containing the names of the fields
    fn get_fields() -> Vec<&'static str>;
    /// Returns a vecrtor containing the types of the fields
    fn get_field_types() -> Vec<&'static str>;
    // TODO? add a function to return a map of the fields and their types
    /// Returns the amount of fields in this struct
    fn field_count() -> usize { Self::get_fields().iter().count() }
}

/// A table should conform to this trait. `Row` is the table's row type.
pub trait Table<Row: TableRow> {
    // Maybe not needed here: const UID: String;
    // TODO: to_string() and fmt() using pretty ASCII tables. Also have a max column size => wrap lines, but that's for later
    // /// Gets the highest width of each element in the table, used for debugging and printing
    // fn get_sizes(&self) -> Vec<isize>;
    /// Gets the column size of a specific column. Requires that the column can be converted to a
    /// String.
    ///
    /// # Examples
    /// ```rust
    /// # use simple_tables::macros::{table_row, table};
    /// # #[table_row]
    /// # struct TableRow {
    /// #     id: u32,
    /// #     name: String
    /// # }
    /// #
    /// # #[table(rows = TableRow)]
    /// # struct MyTable {}
    ///
    /// let vec: Vec<TableRow> = vec![TableRow{id: 1000, name: String::from("Abc")}, TableRow{id: 2, name: String::from("Bd")}];
    /// let table = MyTable::from_vec(&vec);
    ///
    /// assert_eq!(3, table.get_column_size(|row| row.name.clone()).unwrap());
    /// assert_eq!(4, table.get_column_size(|row| row.id).unwrap());
    /// ```
    fn get_column_size<ColumnType: ToString>(&self, column: fn(&Row) -> ColumnType) -> Option<usize> {
        let mut sizes: Vec<usize> = Vec::new();
        for row in self.get_rows() {
            let col = column(row);
            let size = get_size(col);
            sizes.push(size);
        }
        match sizes.iter().max() {
            Some(max) => Some(max.to_owned()),
            None => None
        }
    }
    /// Creates a new empty `Table`
    fn new() -> Self;
    
    // fn to_string(&self) -> String {
    //     let field_names = Row::get_fields();
    //     let field_lenghts = /*for all fields self.get_column_size(/*TODO*/ |row| { 1 });*/ vec![5,5,5];
    //     let cols: Vec<String> = self.get_column(|row| row.to_string());
    //
    //     String::from("")
    // }
    /// Creates a new `Table` with an initial value for the rows
    fn from_vec(vec: &Vec<Row>) -> Self;
    /// Returns an immutable reference to the rows of this table
    fn get_rows(&self) -> &Vec<Row>;
    /// Returns a mutable reference to the rows of this table
    fn get_rows_mut(&mut self) -> &mut Vec<Row>;
    /// Pushes a new row to the end of the table
    fn push(&mut self, row: Row) { self.get_rows_mut().push(row); }
    /// Inserts a new row at the top of the table (element 0)
    fn insert_top(&mut self, row: Row) { self.get_rows_mut().insert(0, row); }
    /// Inserts a new row at index `i`
    fn insert(&mut self, i: usize, row: Row) { self.get_rows_mut().insert(i, row); }
    /// Returns the column with the specific name
    ///
    /// # Example
    /// ```rust
    /// # use simple_tables::macros::{table_row, table};
    /// # #[table_row]
    /// # struct TableRow {
    /// #     id: u32,
    /// #     name: String
    /// # }
    /// #
    /// # #[table(rows = TableRow)]
    /// # struct MyTable {}
    /// #
    /// let vec: Vec<TableRow> = vec![TableRow{id: 1, name: String::from("A")}, TableRow{id: 2, name: String::from("B")}];
    /// let table = MyTable::from_vec(&vec);
    ///
    /// let ids: Vec<u32> = table.get_column(|row| row.id);
    /// assert_eq!(vec![1,2], ids);
    /// ```
    fn get_column<ColumnType>(
        &self,
        column: fn(&Row) -> ColumnType
    )
        -> Vec<ColumnType>
    {
        let columns: Vec<ColumnType> = self.get_rows().to_owned().into_iter().map(|row| {
            column(row)
        }).collect();
        columns
    }
    // TODO (is this even possible?)
    // /// Returns the column at the spefic index
    // fn get_column_at<ColumnType>(&self, column: usize) -> Vec<ColumnType>;
    /// Returns the row at the index
    fn get_row_at(&self, i: usize) -> Option<&Row> { self.get_rows().get(i) }

    // TODO
    // /// Sorts the rows based on a specific column
    // fn sort_on(&mut self, based_on: &str);
    // /// Returns a sorted copy of the rows.<br/>
    // /// The rows are sorted based on a column.
    // fn get_sorted(&self, based_on: &str) -> Vec<Row>;

    fn column_count(&self) -> usize { Row::field_count() }
    fn row_count(&self) -> usize { self.get_rows().len() }
}
// TODO: implement IntoIter for Table

/// Defines a table with a unique identifier. This class should be implemented alongside the
/// [`Table`](crate::Table) trait.
///
/// When you have a table with a uid, this trait has to be implemented manually for now.
///
/// # To implement
/// - [`get_id_from_row`](crate::IdTable)
///
/// # Notes
/// - If your IDE tells you following error when implementing this trait:
///   ```bash
///   the trait bound `MyTable: Table<TableRow>` is not satisfied
///   ```
///   you can simply ignore it given
///   that you are using the `table_row` macro. Your IDE just doesn't know the `Table` trait is
///   already implemented for your struct. When you run your program, it will actually compile and
///   run.
pub trait IdTable<UidType: PartialEq, Row: TableRow>: Table<Row> {
    /// Gets the uid from a row, this should be implemented manually (for now) for structs with uids.
    ///
    /// # Example
    /// ```rust
    /// # use simple_tables::macros::{table_row, table};
    /// # fn main() {
    /// #[table_row]
    /// struct MyTableRow {
    ///     id: i32,
    ///     name: String
    /// }
    ///
    /// #[table(rows = MyTableRow, uid = "id")]
    /// struct MyTable {}
    ///
    /// impl simple_tables::IdTable<i32, MyTableRow> for MyTable {
    ///     fn get_id_from_row(row: i32) -> i32 {
    ///         row.id
    ///     }
    /// }
    /// # }
    /// ```
    fn get_id_from_row(row: &Row) -> UidType;
    
    /// Returns the row with the specific uid
    fn get_row(&self, uid: UidType) -> Option<&Row> {
        let val: Option<&Row> = self.get_rows().iter().find_map(|row| {
            if Self::get_id_from_row(row) == uid {
                Some(row)
            }  else {
                None
            }
        });
        
        val
    }
    
    // TODO
    // /// Searches through the sorted rows using the uid. Only works if the rows have been sorted first,
    // /// the [`sort`](table_rows::core::Table::sort) function can help with this.
    // fn sorted_search(&self, uid: V) -> Option<Row>;
    // /// Sorts the rows based on the `uid`
    // ///
    // /// # Panics
    // /// if there is no `uid` set
    // fn sort(&mut self);
}

fn get_size<Type: ToString>(var: Type) -> usize {
    // let type_id = var.type_id();
    let len = var.to_string().chars().into_iter().count();
    len
    // if type_id == TypeId::of::<String>() {
    //     (var as String).chars().into_iter().count();
    // } else if type_id == TypeId::of::<str>() {
    //     (var as str).chars().into_iter().count();
    // } else if type_id == TypeId::of::<usize> {
    //     (var as usize)
    // }
}