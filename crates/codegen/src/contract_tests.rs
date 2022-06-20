use crate::contract::Contract;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bad_struct() {
        let contract = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod hello {
                    struct AAA_ {
                        value: u64,
                    }
                }        
            }
        );
        assert!(contract.is_err(), "bad return");
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("struct name with `_` does not supported by contract"));
    }

    #[test]
    fn test_dumplciated_action_name() {
        let contract = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod hello {
                    #[chain(main)]
                    pub struct Hello {
                        receiver: Name,
                        first_receiver: Name,
                        action: Name,
                    }
                
                    impl Hello {
                        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
                            Self {
                                receiver: receiver,
                                first_receiver: first_receiver,
                                action: action,
                            }
                        }
                
                        #[chain(action="test")]
                        pub fn test(&self) {
                        }
                
                        #[chain(action="test")]
                        pub fn test2(&self) {
                        }
                    }
                }
            }
        );
        assert!(contract.is_err(), "bad return");
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("dumplicated action name: test"));
    }

    #[test]
    fn test_invalid_action_name() {
        let contract = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod hello {
                    #[chain(main)]
                    pub struct Hello {
                        receiver: Name,
                        first_receiver: Name,
                        action: Name,
                    }
                
                    impl Hello {
                        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
                            Self {
                                receiver: receiver,
                                first_receiver: first_receiver,
                                action: action,
                            }
                        }
                
                        #[chain(action="test9")]
                        pub fn test(&self) {
                        }
                    }
                }
            }
        );
        assert!(contract.is_err(), "bad return");
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("action name contain invalid character(s), valid charaters are a-z & 1-5: test9"));
    }

    #[test]
    fn test_bad_table_name() {
        let contract = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod hello {
                    #[chain(table="mydata9")]
                    pub struct MyData {
                        #[chain(primary)]
                        a1: u64,
                    }
                }
            }
        );
        assert!(contract.is_err(), "bad return");
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("table name contain invalid character(s), valid charaters are a-z & 1-5: mydata9"));
    }

    #[test]
    fn test_dumplicated_table_name() {
        let contract = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod hello {
                    #[chain(table="mydata")]
                    pub struct MyData {
                        #[chain(primary)]
                        a1: u64,
                    }
                
                    #[chain(table="mydata")]
                    pub struct MyData2 {
                        #[chain(primary)]
                        a1: u64,
                    }
                }
            }
        );
        assert!(contract.is_err(), "bad return");
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("dumplicated table name: mydata"));
    }

    #[test]
    fn test_table_no_primary() {
        let ret = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod hello {
                    #[chain(table="mydata")]
                    pub struct Counter {
                        count: u64,
                    }
                }
            }
        );

        assert!(ret.is_ok(), "bad return");
        let code = ret.unwrap().generate_code();
        assert!(code.is_err(), "bad return");
        println!("+++++{}", code.as_ref().err().unwrap().to_compile_error().to_string());
        assert!(code.as_ref().err().unwrap().to_compile_error().to_string().contains("primary index does not specified in struct Counter"));
    }

    #[test]
    fn test_singleton_table_with_primary() {
        println!("++++++test test_singleton_table_with_primary");
        let ret = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod hello {
                    #[chain(table="counter", singleton)]
                    pub struct Counter {
                        #[chain(primary)]
                        key: u64,
                        count: u64
                    }
                }
            }
        );

        assert!(ret.is_ok(), "bad return");
        let code = ret.unwrap().generate_code();
        assert!(code.is_err(), "bad return");
        println!("+++++{}", code.as_ref().err().unwrap().to_compile_error().to_string());
        assert!(code.as_ref().err().unwrap().to_compile_error().to_string().contains("singelton table does not need a primary attribute in struct Counter"));
    }
    
}

