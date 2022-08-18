use core::mem::{
    size_of
};

use crate::serializer::{
    Packer,
    Encoder,
    Decoder,
};

use crate::{
    vec::Vec,
};

#[repr(C)]
#[derive(Default)]
pub struct BlockchainParameters {
        /**
        * The maxiumum net usage in instructions for a block
        * @brief the maxiumum net usage in instructions for a block
        */
        max_block_net_usage: u64,
  
        /**
        * The target percent (1% == 100, 100%= 10,000) of maximum net usage; exceeding this triggers congestion handling
        * @brief The target percent (1% == 100, 100%= 10,000) of maximum net usage; exceeding this triggers congestion handling
        */
        target_block_net_usage_pct: u32,
  
        /**
        * The maximum objectively measured net usage that the chain will allow regardless of account limits
        * @brief The maximum objectively measured net usage that the chain will allow regardless of account limits
        */
        max_transaction_net_usage: u32,
  
        /**
         * The base amount of net usage billed for a transaction to cover incidentals
         */
        base_per_transaction_net_usage: u32,
  
        /**
         * The amount of net usage leeway available whilst executing a transaction (still checks against new limits without leeway at the end of the transaction)
         * @brief The amount of net usage leeway available whilst executing a transaction  (still checks against new limits without leeway at the end of the transaction)
         */
        net_usage_leeway: u32,
 
        /**
        * The numerator for the discount on net usage of context-free data
        * @brief The numerator for the discount on net usage of context-free data
        */
        context_free_discount_net_usage_num: u32,
  
        /**
        * The denominator for the discount on net usage of context-free data
        * @brief The denominator for the discount on net usage of context-free data
        */
        context_free_discount_net_usage_den: u32,
  
        /**
        * The maxiumum billable cpu usage (in microseconds) for a block
        * @brief The maxiumum billable cpu usage (in microseconds) for a block
        */
        max_block_cpu_usage: u32,
  
        /**
        * The target percent (1% == 100, 100%= 10,000) of maximum cpu usage; exceeding this triggers congestion handling
        * @brief The target percent (1% == 100, 100%= 10,000) of maximum cpu usage; exceeding this triggers congestion handling
        */
        target_block_cpu_usage_pct: u32,
  
        /**
        * The maximum billable cpu usage (in microseconds) that the chain will allow regardless of account limits
        * @brief The maximum billable cpu usage (in microseconds) that the chain will allow regardless of account limits
        */
        max_transaction_cpu_usage: u32,
  
        /**
        * The minimum billable cpu usage (in microseconds) that the chain requires
        * @brief The minimum billable cpu usage (in microseconds) that the chain requires
        */
        min_transaction_cpu_usage: u32,
  
        /**
         * Maximum lifetime of a transacton
         * @brief Maximum lifetime of a transacton
         */
        max_transaction_lifetime: u32,
  
        /**
        * The number of seconds after the time a deferred transaction can first execute until it expires
        * @brief the number of seconds after the time a deferred transaction can first execute until it expires
        */
        deferred_trx_expiration_window: u32,
  
  
        /**
        * The maximum number of seconds that can be imposed as a delay requirement by authorization checks
        * @brief The maximum number of seconds that can be imposed as a delay requirement by authorization checks
        */
        max_transaction_delay: u32,
  
        /**
         * Maximum size of inline action
         * @brief Maximum size of inline action
         */
        max_inline_action_size: u32,
  
        /**
         * Maximum depth of inline action
         * @brief Maximum depth of inline action
         */
        max_inline_action_depth: u16,
  
        /**
         * Maximum authority depth
         * @brief Maximum authority depth
         */
        ax_authority_depth: u16,
}

impl Packer for BlockchainParameters {
    fn size(&self) -> usize {
        return size_of::<BlockchainParameters>();
    }

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&self.max_block_net_usage);
        enc.pack(&self.target_block_net_usage_pct);
        enc.pack(&self.max_transaction_net_usage);
        enc.pack(&self.base_per_transaction_net_usage);
        enc.pack(&self.net_usage_leeway);
        enc.pack(&self.context_free_discount_net_usage_num);
        enc.pack(&self.context_free_discount_net_usage_den);
        enc.pack(&self.max_block_cpu_usage);
        enc.pack(&self.target_block_cpu_usage_pct);
        enc.pack(&self.max_transaction_cpu_usage);
        enc.pack(&self.min_transaction_cpu_usage);
        enc.pack(&self.max_transaction_lifetime);
        enc.pack(&self.deferred_trx_expiration_window);
        enc.pack(&self.max_transaction_delay);
        enc.pack(&self.max_inline_action_size);
        enc.pack(&self.max_inline_action_depth);
        enc.pack(&self.ax_authority_depth);
        return enc.get_bytes();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.max_block_net_usage);
        dec.unpack(&mut self.target_block_net_usage_pct);
        dec.unpack(&mut self.max_transaction_net_usage);
        dec.unpack(&mut self.base_per_transaction_net_usage);
        dec.unpack(&mut self.net_usage_leeway);
        dec.unpack(&mut self.context_free_discount_net_usage_num);
        dec.unpack(&mut self.context_free_discount_net_usage_den);
        dec.unpack(&mut self.max_block_cpu_usage);
        dec.unpack(&mut self.target_block_cpu_usage_pct);
        dec.unpack(&mut self.max_transaction_cpu_usage);
        dec.unpack(&mut self.min_transaction_cpu_usage);
        dec.unpack(&mut self.max_transaction_lifetime);
        dec.unpack(&mut self.deferred_trx_expiration_window);
        dec.unpack(&mut self.max_transaction_delay);
        dec.unpack(&mut self.max_inline_action_size);
        dec.unpack(&mut self.max_inline_action_depth);
        dec.unpack(&mut self.ax_authority_depth);
        return dec.get_pos();
    }
}
