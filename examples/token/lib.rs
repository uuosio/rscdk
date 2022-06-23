#![cfg_attr(not(feature = "std"), no_std)]


#[eosio_chain::contract]
mod token {
    use eosio_chain::{
        require_auth,
        check,
        is_account,
        require_recipient,
        has_auth,
        Asset,
        Name,
        SAME_PAYER,        
        Symbol,
    };

    #[chain(table="accounts")]
    pub struct Account {
        #[chain(primary)]
        balance: Asset 
    }
    
    #[chain(table="stat")]
    pub struct CurrencyStats {
        #[chain(primary)]
        supply: Asset,
        max_supply: Asset,
        issuer: Name,
    }

    #[chain(main)]
    pub struct Token {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Token {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action = "create")]
        pub fn create(&self, issuer: Name, maximum_supply: Asset) {
            require_auth(issuer);
            check(maximum_supply.symbol().is_valid(), "invalid symbol name");
            check(maximum_supply.is_valid(), "invalid supply");
            check(maximum_supply.amount() > 0, "max_supply must be positive");
            let sym_code = maximum_supply.symbol().code().value();
            let db = CurrencyStats::new_mi(self.receiver, Name{n: sym_code});
            db.find(sym_code).expect_not_ok("token with symbol already exists");

            let stats = CurrencyStats{
                supply: Asset::new(0, maximum_supply.symbol()),
                max_supply: maximum_supply,
                issuer: issuer,
            };
            db.store(&stats, self.receiver);
        }
    
        #[chain(action = "issue")]
        pub fn issue(&self, to: Name, quantity: Asset, memo: String) {
            check(quantity.is_valid(), "invalid symbol name");
            check(memo.len() <= 256, "memo has more than 256 bytes");
            let sym_code = quantity.symbol().code().value();
            let db = CurrencyStats::new_mi(self.receiver, Name{n: sym_code});
            let it = db
                .find(sym_code)
                .expect("token with symbol does not exist, create token before issue");
            let mut stats = it.get_value().unwrap();
            check(to == stats.issuer, "tokens can only be issued to issuer account");
            require_auth(stats.issuer);
            check(quantity.is_valid(), "invalid quantity");
            check(quantity.amount() > 0, "must issue positive quantity");

            check(quantity.symbol() == stats.supply.symbol(), "symbol precision mismatch");
            check(quantity.amount() <= stats.max_supply.amount() - stats.supply.amount(), "quantity exceeds available supply");
            stats.supply += quantity;
            db.update(&it, &stats, stats.issuer);
            self.add_balance(to, &quantity, to);
        }

        #[chain(action = "retire")]
        pub fn retire(&self, quantity: Asset, memo: String) {
            check(quantity.symbol().is_valid(), "invalid symbol name");
            check(memo.len() <= 256, "memo has more than 256 bytes");
            let sym_code = quantity.symbol().code().value();
            let db = CurrencyStats::new_mi(self.receiver, Name{n: sym_code});
            let it = db
                .find(sym_code)
                .expect("token with symbol does not exist");
            let mut stat = it.get_value().unwrap();
            require_auth(stat.issuer);
            check(quantity.is_valid(), "invalid quantity");
            check(quantity.amount() > 0, "must retire postive quantity");
            check(quantity.symbol() == stat.supply.symbol(), "symbol precision mismatch");
            stat.supply -= quantity;
            db.update(&it, &stat, SAME_PAYER);
            self.sub_balance(stat.issuer, &quantity);
        }

        #[chain(action = "transfer")]
        pub fn transfer(&self, from: Name, to: Name, quantity: Asset, memo: String) {
            check(from != to, "cannot transfer to self");
            require_auth(from);
            check(is_account(to), "to account does not exist");
            let sym_code = quantity.symbol().code().value();
            let db = CurrencyStats::new_mi(self.receiver, Name::from_u64(sym_code));
            let it = db
                .find(sym_code)
                .expect("token with symbol does not exist");
            let currency = it.get_value().unwrap();
            
            require_recipient(from);
            require_recipient(to);

            check(quantity.is_valid(), "invalid quantity");
            check(quantity.amount() > 0, "must transfer positive quantity");
            check(quantity.symbol() == currency.supply.symbol(), "symbol precision mismatch");
            check(memo.len() <= 256, "memo has more than 256 bytes");
            let payer: Name;
            if has_auth(to) {
                payer = to;
            } else {
                payer = from;
            }
            self.sub_balance(from, &quantity);
            self.add_balance(to, &quantity, payer);

        }

        #[chain(action = "open")]
        pub fn open(&self, owner: Name, symbol: Symbol, ram_payer: Name) {
            require_auth(ram_payer);
            check(is_account(owner), "owner account does not exist");
            let db = CurrencyStats::new_mi(self.receiver, Name::from_u64(symbol.code().value()));
            let it = db
                .find(symbol.code().value())
                .expect("symbol does not exist");
            let stat = it.get_value().unwrap();
            check(stat.supply.symbol() == symbol, "symbol precision mismatch");
            let accounts = Account::new_mi(self.receiver, owner);
            let it_account = accounts.find(symbol.code().value());
            if !it_account.is_ok() {
                let account = Account{
                    balance: Asset::new(0, symbol),
                };
                accounts.store(&account, ram_payer);
            }
        }

        #[chain(action = "close")]
        pub fn close(&self, owner: Name, symbol: Symbol) {
            require_auth(owner);
            let accounts = Account::new_mi(self.receiver, owner);
            let it = accounts
                .find(symbol.code().value())
                .expect("balance row already deleted or never existed. Action won't have any effect.");
            let value = it.get_value().unwrap();
            check(value.balance.amount() == 0, "cannot close because the balance is not zero.");
            accounts.remove(&it);
        }

        fn add_balance(&self, owner: Name, value: &Asset, payer: Name) {
            let accounts = Account::new_mi(self.receiver, owner);
            let it = accounts.find(value.symbol().code().value());
            if !it.is_ok() {
                accounts.store(&Account{balance: *value}, payer);
            } else {
                let mut to = it.get_value().unwrap();
                to.balance += *value;
                accounts.update(&it, &to, Name::from_u64(0));
            }
        }

        fn sub_balance(&self, owner: Name, quantity: &Asset) {
            let accounts = Account::new_mi(self.receiver, owner);
            let it = accounts
                .find(quantity.symbol().code().value())
                .expect("no balance object found");
            let mut from = it.get_value().unwrap();
            check(from.balance.amount() >= quantity.amount(), "overdraw balance");
            from.balance -= *quantity;
            accounts.update(&it, &from, owner);
        }
    }
}
