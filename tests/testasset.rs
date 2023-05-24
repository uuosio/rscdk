#[rust_chain::contract]
pub mod testasset {
    use rust_chain::{
        Asset,
        Name,
        Symbol,
        SymbolCode,
        ExtendedAsset,

        asset,

        check,
        name,
        chain_println,
    };

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct TestSerialzier {
        receiver: Name,
        first_receiver: Name,
        action: Name,
        value: u32,
    }

    impl TestSerialzier {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
                value: 0,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, a: Asset) {
            chain_println!("+++++=a:", a);
            check(a.is_valid(), "a.is_valid()");
            check(a.amount() == 11234, "a.amount() == 11234");
            check(a == Asset::from_string("1.1234 EOS"), "1: bad asset!");
            check(a.to_string() == "1.1234 EOS", r#"a.to_string() == "1.1234 EOS"#);
            check(a.symbol() == Symbol::new("EOS", 4u8), "2: bad symbol!");
            check(Encoder::pack(&a) == rust_chain::read_action_data(), "a.pack() == read_action_data()");

            check(a.symbol().to_string() == "4,EOS", r#"a.symbol().to_string() == "4,EOS"#);

            check(a.symbol().value() == Symbol::new("EOS", 4u8).value(), r#"a.symbol().value() == Symbol::new("EOS", 4u8).value()"#);
            check(a.symbol() == Asset::new(11234, Symbol::new("EOS", 4u8)).symbol(), "3: bad symbol!");
            check(a == Asset::new(11234, Symbol::new("EOS", 4u8)), "4: bad asset string!");
            check(a.to_string() == "1.1234 EOS", "5: bad asset string!");

            let mut b = a.clone();
             b += a;
             check(b.amount() == 22468, "b.amount() == 22468");
             b -= a;
             check(b.amount() == 11234, "b.amount() == 11234");

            check(!asset::is_valid_symbol_code(0), "asset::is_valid_symbol_code(0)");
            check(!asset::is_valid_symbol_code(18331338125951297), "asset::is_valid_symbol_code(1)"); //'AAAAA A'
            check(!asset::is_valid_symbol_code(18296153753862465), "asset::is_valid_symbol_code(1)"); //'AAAAA\0A'

            let sym = SymbolCode{value: 280267669825}; //AAAAA
            check(sym.value() == 280267669825, "sym.value() == 280267669825");
            check(sym.to_string() == "AAAAA", r#"sym.to_string() == "AAAAA"#);
            check(sym.size() == 8, "sym.size() == 8");
            let mut sym2 = SymbolCode::new("EOS");
            sym2.unpack(&Encoder::pack(&sym));
            check(sym == sym2, "sym == sym2");

            chain_println!("Done!");
        }

        #[chain(action="test2")]
        pub fn test2(&self, invalid_asset: Asset) {
            check(!invalid_asset.is_valid(), "test failed");
        }

        #[chain(action="test3")]
        pub fn test3(&self, error_asset: String) {
            Asset::from_string(&error_asset);
        }
        
        #[chain(action="test4")]
        pub fn test4(&self, _sym: SymbolCode) {
        }

        #[chain(action="test5")]
        pub fn test5(&self, a: ExtendedAsset) {
            let b = ExtendedAsset::new(Asset::new(11234, Symbol::new("EOS", 4u8)), name!("hello"));
            check(a == b, "a == b");
            check(Encoder::pack(&a) == Encoder::pack(&b), "a.pack() == b.pack()");
            check(b.quantity() == Asset::new(11234, Symbol::new("EOS", 4u8)), r#"b.quantity() == Asset::new(11234, Symbol::new("EOS", 4u8))"#);
            check(b.contract() == name!("hello"), r#"b.contract() == name!("hello")"#);
        }

        #[chain(action="test51")]
        pub fn test51(&self, asset: Asset) {
            check(asset == Asset::from_string("-1.0000 EOS"), "test failed");
            check(Asset::new(-10000, Symbol::new("EOS", 4)) == Asset::from_string("-1.0000 EOS"), "test failed");
            Asset::from_string("-461168601842738.7903 EOS");
            Asset::from_string("461168601842738.7903 EOS");
        }
    }
}
