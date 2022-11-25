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
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        self.actor.pack(enc);
        self.permission.pack(enc);
        enc.get_size() - pos
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
    /// Creates an action by specifying contract account, action name, authorization and data.
    pub fn new(account: Name, name: Name, authorization: Vec<PermissionLevel>, data: &dyn Packer) -> Self {
        let mut enc = Encoder::new(data.size());
        data.pack(&mut enc);
        Self {
            account,
            name,
            authorization: authorization,
            data: enc.get_bytes()
        }
    }

    /// Send inline action to contract.
    pub fn send(&self) {
        let raw = Encoder::pack(self);
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
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.account.pack(enc);
        self.name.pack(enc);
        self.authorization.pack(enc);
        self.data.pack(enc);

        enc.get_size() - pos
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

