use crate::serializer::{
    Packer,
    Encoder,
    Decoder,
};

use crate::structs::{
    TimePointSec,
};

use crate::varint::{
    VarUint32,
};

use crate::action::{
    Action,
};

use crate::{
    vec::Vec,
};

#[cfg_attr(feature = "std", derive(crate::eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct TransactionExtension {
    ty:     u16,
    data:   Vec<u8>,
}

impl Packer for TransactionExtension {
    fn size(&self) -> usize {
        let mut _size: usize = 0;
        _size += self.ty.size();
        _size += self.data.size();
        return _size;
    }
    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack::<u16>(&self.ty);
        enc.pack::<Vec<u8>>(&self.data);
        return enc.get_bytes();
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

    pub fn context_free_actions(&self) -> Vec<Action> {
        return self.context_free_actions.clone();
    }

    pub fn extention(&self) -> Vec<TransactionExtension> {
        return self.extention.clone();
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

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack::<TimePointSec>(&self.expiration);
        enc.pack::<u16>(&self.ref_block_num);
        enc.pack::<u32>(&self.ref_block_prefix);
        enc.pack::<VarUint32>(&self.max_net_usage_words);
        enc.pack::<u8>(&self.max_cpu_usage_ms);
        enc.pack::<VarUint32>(&self.delay_sec);
        enc.pack::<Vec<Action>>(&self.context_free_actions);
        enc.pack::<Vec<Action>>(&self.actions);
        enc.pack::<Vec<TransactionExtension>>(&self.extention);
        return enc.get_bytes();
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
