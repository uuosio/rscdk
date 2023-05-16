use crate::serializer::{
    Packer,
    Encoder
};

#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Default)]
pub struct BinaryExtension<T: Packer + Default> {
    value: Option<T>
}


impl<T> BinaryExtension<T>
where
    T: Packer + Default
{
    ///
    pub fn new(value: Option<T>) -> Self {
        Self {
            value,
        }
    }

    ///
    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

impl<T> Packer for BinaryExtension<T>
where
    T: Packer + Default
{
    ///
    fn size(&self) -> usize {
        if let Some(x) = &self.value {
            x.size()
        } else {
            0
        }
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        if let Some(x) = &self.value {
            x.pack(enc);
        }
        enc.get_size() - pos
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        if data.len() > 0 {
            let mut value = T::default();
            let size = value.unpack(data);
            self.value = Some(value);
            size
        } else {
            self.value = None;
            0
        }
    }
}

