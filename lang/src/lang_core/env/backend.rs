// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::lang_core::env::{calldata::CallData, error::Result};
use cfg_if::cfg_if;
use liquid_primitives::{
    types::{timestamp, Address},
    Topics,
};

#[derive(PartialEq)]
pub enum CallMode {
    Deploy,
    Call,
}

pub trait Env {
    fn set_storage<V>(&mut self, key: &[u8], value: &V)
    where
        V: scale::Encode;

    fn get_storage<R>(&mut self, key: &[u8]) -> Result<R>
    where
        R: scale::Decode;

    fn remove_storage(&mut self, key: &[u8]);

    fn get_call_data(&mut self, mode: CallMode) -> Result<CallData>;

    fn get_caller(&mut self) -> Address;

    fn get_tx_origin(&mut self) -> Address;

    fn now(&mut self) -> timestamp;

    fn get_block_number(&mut self) -> u64;

    fn get_address(&mut self) -> Address;

    cfg_if! {
        if #[cfg(feature = "solidity-compatible")] {
            fn emit<Event>(&mut self, event: Event)
            where
                Event: Topics + liquid_abi_codec::Encode;

            fn call<R>(&mut self, addr: &Address, data: &[u8]) -> Result<R>
            where
                R: liquid_abi_codec::Decode + liquid_abi_codec::TypeInfo;

            fn finish<V>(&mut self, return_value: &V)
            where
                V: liquid_abi_codec::Encode;

            fn revert<V>(&mut self, revert_info: &V)
            where
                V: liquid_abi_codec::Encode;
        } else {
            fn emit<Event>(&mut self, event: Event)
            where
                Event: Topics + scale::Encode;

            fn call<R>(&mut self, addr: &Address, data: &[u8]) -> Result<R>
            where
                R: scale::Decode;

            fn finish<V>(&mut self, return_value: &V)
            where
                V: scale::Encode;

            fn revert<V>(&mut self, revert_info: &V)
            where
                V: scale::Encode;
        }
    }
}