use crate::{
    vec::Vec,
};

use crate::vmapi::eosio::{
    send_inline,
    check,
};

use crate::varint::{
    VarUint32,
};

use crate::name::{
    Name,
};

use crate::serializer::{
    Packer,
    Encoder,
    Decoder,
};

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct PermissionLevel {
    ///
    pub actor: Name,
    ///
    pub permission: Name,
}

impl PermissionLevel {
    ///
    pub fn new(actor: Name, permission: Name) -> Self {
        Self { actor, permission }
    }
}

impl Packer for PermissionLevel {
    ///
    fn size(&self) -> usize {
        return 16;
    }

    ///
    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(16);
        enc.pack(&self.actor);
        enc.pack(&self.permission);
        return enc.get_bytes();
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "PermissionLevel.unpack: buffer overflow");
        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.actor);
        dec.unpack(&mut self.permission);
        return 16;
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq)]
pub struct Action {
    /// action account
    pub account: Name,
    /// action name
    pub name: Name,
    ///
    pub authorization: Vec<PermissionLevel>,
    ///
    pub data: Vec<u8>,
}

impl Action {
    ///
    pub fn new(account: Name, name: Name, authorization: &Vec<PermissionLevel>, data: &dyn Packer) -> Self {
        Self {
            account,
            name,
            authorization: authorization.clone(),
            data: data.pack(),
        }
    }
    ///
    pub fn send(&self) {
        let raw = self.pack();
        send_inline(&raw);
    }
}

impl Default for Action {
    fn default() -> Self {
        Self { account: Name{n: 0}, name: Name{n: 0}, authorization: Vec::new(), data: Vec::new() }
    }
}

impl Packer for Action {
    ///
    fn size(&self) -> usize {
        let mut size: usize;
        size = 16;
        size += VarUint32::new(self.authorization.len() as u32).size()+ self.authorization.len() * 16;
        size += VarUint32::new(self.data.len() as u32).size() + self.data.len();
        return size
    }

    ///
    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&self.account);
        enc.pack(&self.name);
        enc.pack(&self.authorization);
        enc.pack(&self.data);
        return enc.get_bytes();
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "Action.unpack: buffer overflow");

        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.account);
        dec.unpack(&mut self.name);
        dec.unpack(&mut self.authorization);
        dec.unpack(&mut self.data);
        dec.get_pos()
    }
}

