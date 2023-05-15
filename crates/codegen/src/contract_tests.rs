#[cfg(test)]
mod tests {
    use crate::contract::Contract;
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
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("structs with `_` in name are not supported by contract"));
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
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("action name is empty or contains invalid character(s). valid characters are a-z and 1-5.: test9"));
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
        assert!(contract.err().unwrap().to_compile_error().to_string().contains("table name is empty or contains invalid character(s). valid charaters are a-z & 1-5: mydata9"));
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

    #[test]
    fn test_custom_trait() {
        let ret = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod test {
                    use rust_chain::{
                        Name,
                        Asset,
                        Float128,
                        Uint256,
                
                        check,
                        eosio_println,
                        db::{
                            SecondaryValue,
                            SecondaryIterator,
                            PrimaryValueInterface,
                            SecondaryValueInterface,
                        }
                    };
                
                    #[chain(table="mydata")]
                    pub struct MyData {
                        #[chain(primary)]
                        a1: u64,
                        #[chain(secondary)]
                        a2: u64,
                        #[chain(secondary)]
                        a3: u128,
                        #[chain(secondary)]
                        a4: Uint256,
                        #[chain(secondary)]
                        a5: f64,
                        #[chain(secondary)]
                        a6: Float128,
                    }
                }
            }
        );

        assert!(ret.is_ok(), "bad return");
        assert!(!ret.as_ref().unwrap().has_primary_value_interface_trait("MyData"));
        assert!(!ret.as_ref().unwrap().has_secondary_value_interface_trait("MyData"));
    }

    #[test]
    fn test_custom_trait2() {
        let ret = Contract::new(
            syn::parse_quote! {},
            syn::parse_quote! {
                mod test {
                    use rust_chain::{
                        Name,
                        Asset,
                        Float128,
                        Uint256,
                
                        check,
                        eosio_println,
                        db::{
                            SecondaryValue,
                            SecondaryIterator,
                            PrimaryValueInterface,
                            SecondaryValueInterface,
                        }
                    };
                
                    #[chain(table="mydata")]
                    pub struct MyData {
                        #[chain(primary)]
                        a1: u64,
                        #[chain(secondary)]
                        a2: u64,
                        #[chain(secondary)]
                        a3: u128,
                        #[chain(secondary)]
                        a4: Uint256,
                        #[chain(secondary)]
                        a5: f64,
                        #[chain(secondary)]
                        a6: Float128,
                    }
                
                    impl PrimaryValueInterface for MyData {
                        ///
                        fn get_primary(&self) -> u64 {
                            return self.a1;
                        }
                    }

                    impl SecondaryValueInterface for MyData {
                        ///
                        fn get_secondary_value(&self, i: usize) -> SecondaryValue {
                            match i {
                                0 => self.a2.into(),
                                1 => self.a3.into(),
                                2 => self.a4.into(),
                                3 => self.a5.into(),
                                4 => self.a6.into(),
                                 _ => {SecondaryValue::None}
                            }
                        }
                        ///
                        fn set_secondary_value(&mut self, i: usize, value: SecondaryValue) {
                            match i {
                                0 => self.a2 = value.into(),
                                1 => self.a3 = value.into(),
                                2 => self.a4 = value.into(),
                                3 => self.a5 = value.into(),
                                4 => self.a6 = value.into(),
                                 _ => {}
                            }
                        }
                    }
                }
            }
        );

        assert!(ret.is_ok(), "bad return");
        assert!(ret.as_ref().unwrap().has_primary_value_interface_trait("MyData"));
        assert!(ret.as_ref().unwrap().has_secondary_value_interface_trait("MyData"));
    }
}

