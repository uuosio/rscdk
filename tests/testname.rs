#[rust_chain::contract]
pub mod testname {
    use rust_chain::{
        Name,
        name,
        check,
        eosio_println,
    };

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct NameTest {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    #[allow(dead_code)]
    impl NameTest {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, a11: String, a12: Name, a21: String, a22: Name) {
            check(a12 == Name::from_str(&a11), "bad value 1");
            check(a22 == Name::from_str(&a21), "bad value 2");
            let mut _a22 = Name::default();
            _a22.unpack(&Encoder::pack(&a22));
            check(a22 == _a22, "a22 == _a22");

            check(Name::from_str("").n == 0, r#"Name::from_str("").n == 0"#);

            check(a12 == name!("hello1"), "bad value 1");
            check(a22 == name!("aaaaaaaaaaaaj"), "bad value 2");

            let n1 = name!("hello1");
            let n2 = name!("hello2");
            let n3 = name!("hello3");
            let n4 = name!("hello4");
            let n5 = name!("hello5");
            let n6 = Name::new("hello11");
            let n7 = Name::new("hello12");
            let n8 = Name::new("hello13");
            let n9 = name!("hello14");
            check(n9.to_string() == "hello14", r#"n9.to_string() == "hello14""#);

            check(name::n2s(name::s2n("hello")) == "hello", r#"name::n2s(name::s2n("hello")) == "hello""#);

            eosio_println!("hello", n1, n2, n3, n4, n5, n6, n7, n8, n9);
        }

        #[chain(action="test2")]
        pub fn test2(&self, a: String) {
            Name::from_str(&a);
        }

        #[chain(action="test3")]
        pub fn test3(&self) {
            check(name::static_str_to_name("123451234512z") == 0xFFFF_FFFF_FFFF_FFFFu64, "bad name string");
            check(name::static_str_to_name("123451234512A") == 0xFFFF_FFFF_FFFF_FFFFu64, "bad name string");
            check(name::static_str_to_name("12345A1234512") == 0xFFFF_FFFF_FFFF_FFFFu64, "bad name string");
            check(name::static_str_to_name("12345123451234") == 0xFFFF_FFFF_FFFF_FFFFu64, "bad name string");
            check(name::static_str_to_name("") == 0u64, "bad name string");
            check(name::static_str_to_name("1234512345123") != 0xFFFF_FFFF_FFFF_FFFFu64, "bad name string");
        }
    }
}

