extern crate proc_macro;
extern crate proc_macro2;
// Parsing rust code
extern crate syn;
// Creating rust code
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};
use syn::parse::Parser;
use proc_macro2::{Ident as Ident2, TokenStream as TokenStream2};
use quote::ToTokens;

/// Initialises a struct to be used as a TableRow so it can be used as an entry inside of a
/// [Table](simple_tables_core::Table)
#[proc_macro_attribute]
pub fn table_row(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    
    let struct_name = &item_struct.ident;
    
    let fields: Vec<(String, syn::Type)>;
    let mut ident_fields: Vec<(Ident2, syn::Type)> = Vec::new();
    if let syn::Fields::Named(ref _fields) = item_struct.fields {
        let _fields = &_fields.named;
        fields = _fields.iter().map(|field| {
            if let Some(ident) = &field.ident {
                let field_name: String = ident.to_string();
                let field_type = &field.ty;
                let entry = (field_name, field_type.clone());
                
                ident_fields.push((ident.clone(), field_type.clone()));
                
                entry
            } else {
                panic!("Only named fields are supported.")
                // syn::Error::into_compile_error("Only named fields are supported."); // TODO
            }
        }).collect();
    } else {
        panic!("The row struct has no fields.");
    }
    
    let mut field_names: Vec<String> = Vec::new();
    let mut field_types: Vec<syn::Type> = Vec::new();
    
    // (field_names, field_types) = fields.iter().unzip(); // TODO
    fields.iter()
        .for_each(|field| {
        let (_name, _type) = field;
        field_names.push(_name.to_owned());
        field_types.push(_type.to_owned());
    });
    let field_types_strings: Vec<String> = field_types.iter().map(|v| v.to_token_stream().to_string()).collect();
    
    let field_len = fields.iter().count();
    let mut get_field_str_elements: Vec<proc_macro2::TokenStream> = Vec::new();
    for ident_field in ident_fields {
        let ident = ident_field.0;
        let field = quote!( self.#ident );
        get_field_str_elements.push(field);
    }
    let get_field_str = quote!(
        fn get_field_str(&self) -> Vec<String> {
            vec![ #(#get_field_str_elements.to_string(),)* ]
        }
    );
    TokenStream::from (
        quote! (
            use simple_tables::core::TableRow as TableRowTrait;
            
            #[derive(Debug, Clone)]
            #item_struct
            
            impl #struct_name {
                const FIELDS: [&'static str; #field_len] = [#(#field_names),*];
                // const TYPES: [FieldType; #field_len] = [#(#field_types),*];
                const TYPES: [&'static str; #field_len] = [#(#field_types_strings),*];
                
                #get_field_str
            }
            
            impl TableRowTrait for #struct_name {
                fn get_fields() -> Vec<&'static str> {
                    Self::FIELDS.to_vec()
                }
                fn get_field_types() -> Vec<&'static str> {
                    Self::TYPES.to_vec()
                }
                fn field_count() -> usize {
                    return #field_len;
                }
            }
        )
    )
}

/// Initialises a struct to be a Table that holds information about a [table row](macro@crate::table_row).
///
/// # Examples
/// ```rust
/// #[table_row]
/// struct TableRow {
///     id: i32,
///     name: String,
///     email: String
/// }
///
/// #[table(rows = TableRow)]
/// struct Table {}
/// ```
#[proc_macro_attribute]
pub fn table(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    
    // parse the attributes
    // # Attributes:
    // - rows: Ident — '=': Punct — TableRowStruct: Ident
    // - uid: Ident — '=': Punct — "FieldName": Literal (kind: Str)
    let mut current_attr: Option<&str> = None;
    let mut table_row_struct: Option<Ident2> = None;
    let mut uid_field_name: Option<String> = None;
    attrs.into_iter().for_each(|token| {
        match token {
            // https://doc.rust-lang.org/proc_macro/enum.TokenTree.html
            proc_macro::TokenTree::Group(group) => panic!("Unexpected attribute: {}", group),
            proc_macro::TokenTree::Ident(ident) => {
                match &ident.to_string().as_str() {
                    &"rows" =>  current_attr = Some("rows"),
                    &"uid" => current_attr = Some("uid"),
                    val => {
                        if current_attr == Some("rows") {
                            table_row_struct = Some(Ident2::new(val, proc_macro2::Span::call_site()));
                        }  else {
                            panic!("Unexpected token: {}", val);
                        }
                    }
                }
            },
            proc_macro::TokenTree::Punct(punct) => {
                if punct.as_char() == '=' && current_attr.is_some() {
                    // ignored (TODO: enforce syntax)
                } else if punct.as_char() == ',' {
                    if current_attr == None {
                        panic!("Unexpected character: {}", punct.as_char());
                    } else {
                        current_attr = None;
                    }
                } else {
                    panic!("Unkown character: {}", punct);
                }
            },
            proc_macro::TokenTree::Literal(literal) => {
                if current_attr == Some("uid") {
                    // Remove "\"" at begin and end using trim_matches
                    uid_field_name = Some(literal.to_string().trim_matches(|c| c == '\"').to_string());
                }
            }
        }
    });
    
    if let Some(table_row_struct) = table_row_struct {
        let field_to_add = quote!(rows: Vec<#table_row_struct>);
        let struct_name = &item_struct.ident;
        // add field to struct
        if let syn::Fields::Named(ref mut fields) = item_struct.fields {
            fields.named.push(
                syn::Field::parse_named
                    .parse2(field_to_add)
                    .unwrap(),
            );
        }
        
        let uid_code: TokenStream2;
        if let Some(uid) = uid_field_name {
            uid_code = quote!(const UID: &'static str = #uid;);
        } else {
            uid_code = quote!();
        }
        
        let impl_to_string = quote!(
            impl ToString for #struct_name {
                fn to_string(&self) -> String {
                    // The names of the fields
                    let field_names = dev_simple_tables_core_table_row_type::get_fields();
                    // All cells
                    let mut row_values: Vec<Vec<String>> = Vec::new();
                    for row in &self.rows {
                        row_values.push(row.get_field_str());
                    }
                    // The sizes of the columns
                    let mut column_sizes: Vec<usize> = vec![0; field_names.len()];
                    row_values.iter().for_each(|(row_val)| {
                        row_val.iter().enumerate().for_each(|(col, col_val)| {
                            let len = col_val.to_string().chars().count();
                            if column_sizes[col] < len {
                                column_sizes[col] = len;
                            }
                        });
                    });
                    
                    println!("Column sizes: {:?}", column_sizes);
                    
                    let mut top_line: String = String::from("+-");
                    let mut headers: String = String::from("| ");
                    let mut bottom_line: String = String::from("+=");
                    let mut actual_column_sizes: Vec<usize> = column_sizes.clone();
                    let total_columns = column_sizes.len();
                    column_sizes.into_iter().enumerate().for_each(|(col, col_size)| {
                        let mut local_col_size = col_size.clone();
                        let field_name = field_names[col];
                        let field_len = field_name.chars().count();
                        println!("Col_size: {} - Field_len: {}", col_size, field_len);
                        // Hanlde case when cells are smaller than the title
                        let left_over = if field_len > local_col_size {
                            local_col_size = field_len;
                            actual_column_sizes[col] = field_len;
                            0
                        } else {
                            local_col_size - field_len
                        };
                        top_line.push_str(format!("{}-+", "-".repeat(local_col_size)).as_str());
                        headers.push_str(format!("{}{} |", field_name, " ".repeat(left_over)).as_str());
                        bottom_line.push_str(format!("{}=+", "=".repeat(local_col_size)).as_str());
                        if col != total_columns - 1 {
                            top_line.push_str("-");
                            headers.push_str(" ");
                            bottom_line.push_str("=");
                        }
                    });
                    
                    // Adding the cells to the formatted table
                    let mut cells: String = String::from("| ");
                    row_values.into_iter().enumerate().for_each(|(row, row_val)| {
                        if row != 0 {
                            cells.push_str("\n| ");
                        }
                        row_val.into_iter().enumerate().for_each(|(col, cell_val)| {
                            let left_over = actual_column_sizes[col] - cell_val.chars().count();
                            cells.push_str(format!("{}{} |", cell_val, " ".repeat(left_over)).as_str());
                            if col != total_columns - 1 {
                            cells.push_str(" ");
                        }
                        });
                        // Add horizontal line to bottom
                        cells.push_str(format!("\n{}", top_line).as_str());
                    });
                    
                    format!("{}\n{}\n{}\n{}", top_line, headers, bottom_line, cells)
                }
            }
        );
        
        let output = quote! (
            #[automatically_derived]
            #item_struct
            
            impl #struct_name {
                #uid_code
            }
            
            type dev_simple_tables_core_table_row_type = #table_row_struct; // TODO: generate uid, then to ident
            
            impl simple_tables::core::Table<#table_row_struct> for #struct_name {
                fn new() -> #struct_name {
                    #struct_name { rows: Vec::new() }
                }
                
                fn from_vec(vec: &Vec<#table_row_struct>) -> #struct_name {
                    #struct_name { rows: vec.to_vec() }
                }
                
                fn get_rows(&self) -> &Vec<#table_row_struct> {
                    &self.rows
                }
                
                fn get_rows_mut(&mut self) -> &mut Vec<#table_row_struct> {
                    &mut self.rows
                }
                
                fn push(&mut self, row: #table_row_struct) {
                    self.rows.push(row);
                }
                
                fn insert_top(&mut self, row: #table_row_struct) {
                    self.rows.insert(0, row);
                }
                
                fn insert(&mut self, i: usize, row: #table_row_struct) {
                    self.rows.insert(i, row);
                }
            }
            
            #impl_to_string
        );
    
        TokenStream::from(output)
    } else {
        panic!("Please specify a struct to use as the data type for the table rows. \
        e.g. `#[table(rows = TableRowStruct)]`. Refer to the `table` macro documentation for more info.")
    }
}

// https://mbuffett.com/posts/incomplete-macro-walkthrough/