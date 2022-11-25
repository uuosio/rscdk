use crate::serializer::{
    Packer,
    Encoder,
    Decoder,
};

use crate::structs::{
    TimePointSec,
    Uint128,
};

use crate::varint::{
    VarUint32,
};

use crate::action::{
    Action,
};

use crate::{
    vec::Vec,
    Name,
    send_deferred,
    tapos_block_num,
    tapos_block_prefix
};

#[cfg_attr(feature = "std", derive(crate::eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct TransactionExtension {
    pub ty:     u16,
    pub data:   Vec<u8>,
}

impl Packer for TransactionExtension {
    fn size(&self) -> usize {
        let mut _size: usize = 0;
        _size += self.ty.size();
        _size += self.data.size();
        return _size;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.ty.pack(enc);
        self.data.pack(enc);

        enc.get_size() - pos
    }
    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut dec = Decoder::new(data);
        dec.unpack::<u16>(&mut self.ty);
        dec.unpack::<Vec<u8>>(&mut self.data);
        return dec.get_pos();
    }
}

#[cfg_attr(feature = "std", derive(crate::eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Transaction {
    expiration:             TimePointSec,
    ref_block_num:          u16,
    ref_block_prefix:       u32,
    //[VLQ or Base-128 encoding](https://en.wikipedia.org/wiki/Variable-length_quantity)
    //unsigned_int vaint (eosio.cdt/libraries/eosiolib/core/eosio/varint.hpp)
    max_net_usage_words:    VarUint32, /// number of 8 byte words this transaction can serialize into after compressions
    max_cpu_usage_ms:       u8, /// number of CPU usage units to bill transaction for
    delay_sec:              VarUint32, /// number of seconds to delay transaction, default: 0
    context_free_actions:   Vec<Action>,
    actions:                Vec<Action>,
    extention:              Vec<TransactionExtension>,
}

impl Transaction {
    pub fn new(expiration: u32, delay_sec: u32) -> Self {
        Self {
            expiration: TimePointSec { seconds: expiration },
            ref_block_num: tapos_block_num() as u16,
            ref_block_prefix: tapos_block_prefix(),
            max_net_usage_words: VarUint32::new(0),
            max_cpu_usage_ms: 0,
            delay_sec: VarUint32::new(delay_sec),
            context_free_actions: Vec::new(),
            actions: Vec::new(),
            extention: Vec::new()
        }
    }

    pub fn expiration(&self) -> TimePointSec {
        return self.expiration;
    }

    pub fn ref_block_num(&self) -> u32 {
        return self.ref_block_num as u32;
    }

    pub fn ref_block_prefix(&self) -> u32 {
        return self.ref_block_prefix;
    }

    pub fn max_net_usage_words(&self) -> u32 {
        return self.max_net_usage_words.value();
    }

    pub fn max_cpu_usage_ms(&self) -> u32 {
        return self.max_cpu_usage_ms as u32;
    }

    pub fn delay_sec(&self) -> u32 {
        return self.delay_sec.value();
    }

    pub fn actions(&self) -> Vec<Action> {
        return self.actions.clone();
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn context_free_actions(&self) -> Vec<Action> {
        return self.context_free_actions.clone();
    }

    pub fn extention(&self) -> Vec<TransactionExtension> {
        return self.extention.clone();
    }

    pub fn send(&self, payer: Name, id: u128, replace_existing: bool) {
        let id = Uint128{lo: (id & u64::MAX as u128) as u64, hi: (id >> 64) as u64};
        send_deferred(&id, payer, &Encoder::pack(self), replace_existing.into());
    }

}

impl Packer for Transaction {
    fn size(&self) -> usize {
        let mut _size: usize = 0;
        _size += self.expiration.size();
        _size += self.ref_block_num.size();
        _size += self.ref_block_prefix.size();
        _size += self.max_net_usage_words.size();
        _size += self.max_cpu_usage_ms.size();
        _size += self.delay_sec.size();
        _size += self.context_free_actions.size();
        _size += self.actions.size();
        _size += self.extention.size();
        return _size;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.expiration.pack(enc);
        self.ref_block_num.pack(enc);
        self.ref_block_prefix.pack(enc);
        self.max_net_usage_words.pack(enc);
        self.max_cpu_usage_ms.pack(enc);
        self.delay_sec.pack(enc);
        self.context_free_actions.pack(enc);
        self.actions.pack(enc);
        self.extention.pack(enc);

        enc.get_size() - pos
    }

    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut dec = Decoder::new(data);
        dec.unpack::<TimePointSec>(&mut self.expiration);
        dec.unpack::<u16>(&mut self.ref_block_num);
        dec.unpack::<u32>(&mut self.ref_block_prefix);
        dec.unpack::<VarUint32>(&mut self.max_net_usage_words);
        dec.unpack::<u8>(&mut self.max_cpu_usage_ms);
        dec.unpack::<VarUint32>(&mut self.delay_sec);
        dec.unpack::<Vec<Action>>(&mut self.context_free_actions);
        dec.unpack::<Vec<Action>>(&mut self.actions);
        dec.unpack::<Vec<TransactionExtension>>(&mut self.extention);
        return dec.get_pos();
    }
}

// bool
// check_transaction_authorization( const transaction&                 trx,
//                                  const std::set<permission_level>&  provided_permissions ,
//                                  const std::set<public_key>&        provided_keys = std::set<public_key>()
//                                )
