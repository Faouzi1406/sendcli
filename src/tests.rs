#[cfg(test)]
pub mod tests{
    use crate::Args;
    use crate::handle_path_type::PathType;
    use crate::handle_path_type::FileTypeTrait;

    #[test]
    fn  test_file_type() {
        //Testing file types path
        let args = Args {path:".".to_string()};
        assert_eq!(PathType::File, args.type_is_file(args.first_char()));


        let args = Args {path:"./".to_string()};
        assert_eq!(PathType::File, args.type_is_file(args.first_char()));


        let args = Args {path:"../".to_string()};
        assert_eq!(PathType::File, args.type_is_file(args.first_char()));


        let args = Args {path:"~".to_string()};
        assert_eq!(PathType::File, args.type_is_file(args.first_char()));

        
        let args = Args {path:"~/".to_string()};
        assert_eq!(PathType::File, args.type_is_file(args.first_char()));
    }   

    #[test]
    fn test_path_url() {
        let args = Args {path:"h".to_string()};
        assert_eq!(PathType::Url, args.type_is_url(args.first_char()));
            
        let args = Args {path:"https://".to_string()};
        assert_eq!(PathType::Url, args.type_is_url(args.first_char()));

        let args = Args {path:"http://".to_string()};
        assert_eq!(PathType::Url, args.type_is_url(args.first_char()));
    }

}
