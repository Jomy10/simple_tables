mod error {
    use simple_tables::error::*;
    
    #[test]
    fn display() {
        let error: TableError = TableError {
            kind: TableErrorKind::CouldNotRemove,
            message: "This is the error message".to_string()
        };
        
        assert_eq!(error.message, format!("{}", error))
    }
    
    #[test]
    fn dbg() {
        let error: TableError = TableError {
            kind: TableErrorKind::CouldNotRemove,
            message: "This is the error message".to_string()
        };
        
        assert_eq!("CouldNotRemove: This is the error message", format!("{:?}", error))
    }
    
}

mod kind {
    use simple_tables::error::*;
    
    #[test]
    fn dbg() {
        let err = TableErrorKind::CouldNotRemove;
        assert_eq!("CouldNotRemove", format!("{:?}", err).as_str());
    }
    
    #[test]
    fn display() {
        let err = TableErrorKind::CouldNotRemove;
        assert_eq!("CouldNotRemove", format!("{}", err).as_str());
    }
    
    #[test]
    fn to_string() {
        let err = TableErrorKind::CouldNotRemove;
        assert_eq!("CouldNotRemove", err.to_string().as_str());
    }
}
