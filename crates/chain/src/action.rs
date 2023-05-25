use crate::{
    vec,
    vec::Vec,
};

use crate::vmapi::eosio::{
    send_inline,
    check,
};

use crate::vmapi::eosio_ex;

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

use crate::structs::{
    Checksum256,
};

/// A structure representing a permission level for an action in a smart contract system.
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct PermissionLevel {
    /// The account holding the permission.
    pub actor: Name,
    /// The permission type.
    pub permission: Name,
}

impl PermissionLevel {
    /// Creates a new permission level with the specified actor and permission.
    pub fn new(actor: Name, permission: Name) -> Self {
        Self { actor, permission }
    }
}

/// Implements the Packer trait for PermissionLevel to enable serialization and deserialization.
impl Packer for PermissionLevel {
    /// Returns the packed size of the PermissionLevel structure.
    fn size(&self) -> usize {
        return 16;
    }

    /// Packs the PermissionLevel structure into the provided Encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        self.actor.pack(enc);
        self.permission.pack(enc);
        enc.get_size() - pos
    }

    /// Unpacks the PermissionLevel structure from the provided data slice.
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "PermissionLevel.unpack: buffer overflow");
        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.actor);
        dec.unpack(&mut self.permission);
        return 16;
    }
}

/// A structure representing an action to be executed in a smart contract system.
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq)]
pub struct Action {
    /// The account on which the action is executed.
    pub account: Name,
    /// The name of the action.
    pub name: Name,
    /// A list of permission levels required to execute the action.
    pub authorization: Vec<PermissionLevel>,
    /// The action's payload data.
    pub data: Vec<u8>,
}

impl Action {
    /// Creates an action by specifying contract account, action name, authorization and data.
    pub fn new(account: Name, name: Name, authorization: PermissionLevel, data: &dyn Packer) -> Self {
        let mut enc = Encoder::new(data.size());
        data.pack(&mut enc);
        Self {
            account,
            name,
            authorization: vec![authorization],
            data: enc.get_bytes().to_vec()
        }
    }

    pub fn new_ex(account: Name, name: Name, authorizations: Vec<PermissionLevel>, data: &dyn Packer) -> Self {
        let mut enc = Encoder::new(data.size());
        data.pack(&mut enc);
        Self {
            account,
            name,
            authorization: authorizations,
            data: enc.get_bytes().to_vec()
        }
    }

    /// Send inline action to contract.
    pub fn send(&self) {
        let raw = Encoder::pack(self);
        send_inline(&raw);
    }
}

/// Implements the Default trait for Action.
impl Default for Action {
    fn default() -> Self {
        Self { account: Name{n: 0}, name: Name{n: 0}, authorization: Vec::new(), data: Vec::new() }
    }
}

/// Implements the Packer trait for Action to enable serialization and deserialization.
impl Packer for Action {
    /// Returns the packed size of the Action structure.
    fn size(&self) -> usize {
        let mut size: usize;
        size = 16;
        size += VarUint32::new(self.authorization.len() as u32).size()+ self.authorization.len() * 16;
        size += VarUint32::new(self.data.len() as u32).size() + self.data.len();
        return size
    }

    /// Packs the Action structure into the provided Encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.account.pack(enc);
        self.name.pack(enc);
        self.authorization.pack(enc);
        self.data.pack(enc);

        enc.get_size() - pos
    }

    /// Unpacks the Action structure from the provided data slice.
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

#[derive(Default)]
pub struct GetCodeHashResult {
    struct_version: VarUint32,
    code_sequence: u64,
    code_hash: Checksum256,
    vm_type: u8,
    vm_version: u8,
}

impl Packer for GetCodeHashResult {
    fn size(&self) -> usize {
        self.struct_version.size() + 8 + 32 + 1 + 1
    }

    /// Packs the Action structure into the provided Encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.struct_version.pack(enc);
        self.code_sequence.pack(enc);
        self.code_hash.pack(enc);
        self.vm_type.pack(enc);
        self.vm_version.pack(enc);

        enc.get_size() - pos
    }

    /// Unpacks the Action structure from the provided data slice.
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "Action.unpack: buffer overflow");

        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.struct_version);
        dec.unpack(&mut self.code_sequence);
        dec.unpack(&mut self.code_hash);
        dec.unpack(&mut self.vm_type);
        dec.unpack(&mut self.vm_version);
        dec.get_pos()
    }
}

pub fn get_code_hash(account: Name) -> Checksum256 {
    let data = eosio_ex::get_code_hash(account, 0u32);
    let mut dec = Decoder::new(&data);
    let mut result = GetCodeHashResult::default();
    dec.unpack(&mut result);
    return result.code_hash;
}
