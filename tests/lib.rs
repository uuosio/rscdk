#![cfg_attr(not(feature = "std"), no_std)]

mod testmi;
mod testmi2;
mod testname;
mod testdestructor;
mod testbinaryextension;
mod testtransaction;

#[eosio_chain::contract]
mod hello {

    use super::testmi::test::TestMI;
    use super::testmi2::test::TestMI2;
    use super::testname::test::NameTest;
    use super::testdestructor::test::TestDestructor;
    use super::testbinaryextension::test::TestBinaryExtension;
    use super::testtransaction::test::TestTransaction;

    use eosio_chain::{
        Name,
        Asset,
        BinaryExtension,
    };

    #[chain(main)]
    struct Main {
        receiver: Name, first_receiver: Name, action: Name
    }

    impl Main {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver, first_receiver, action
            }
        }
        #[chain(action="mitest1")]
        pub fn mitest1(&self) {
            let test = TestMI::new(self.receiver, self.first_receiver, self.action);
            test.test1();    
        }

        #[chain(action="mitest2")]
        pub fn mitest2(&self) {
            let test = TestMI::new(self.receiver, self.first_receiver, self.action);
            test.test2();    
        }

        #[chain(action="mi2test")]
        pub fn mi2test(&self) {
            let test = TestMI2::new(self.receiver, self.first_receiver, self.action);
            test.test();    
        }

        /// name test
        #[chain(action="nametest")]
        pub fn nametest(&self, a11: String, a12: Name, a21: String, a22: Name) {
            let test = NameTest::new(self.receiver, self.first_receiver, self.action);
            test.test(a11, a12, a21, a22);
        }

        #[chain(action="destructtest")]
        pub fn destructor_test(&self) {
            let mut test = TestDestructor::new(self.receiver, self.first_receiver, self.action);
            test.inc_count();
        }

        #[chain(action="binexttest")]
        pub fn binary_extension_test(&self, a: BinaryExtension<u64>) {
            let test = TestBinaryExtension::new(self.receiver, self.first_receiver, self.action);
            test.test(a);
        }

        #[chain(action="trxtest")]
        pub fn transaction_test(&self) {
            let test = TestTransaction::new(self.receiver, self.first_receiver, self.action);
            test.test();
        }
    }
}
