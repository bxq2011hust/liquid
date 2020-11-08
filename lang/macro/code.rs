#![feature(prelude_import)]
#![feature(unboxed_closures, fn_traits)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use liquid_lang as liquid;
mod entry {
    mod __liquid_private {
        use super::*;
        #[allow(non_camel_case_types)]
        pub type address = liquid_primitives::types::Address;
        #[allow(non_camel_case_types)]
        pub type bytes = liquid_primitives::types::Bytes;
        #[allow(non_camel_case_types)]
        pub type byte = liquid_primitives::types::Byte;
        #[allow(non_camel_case_types)]
        pub type bytes1 = liquid_primitives::types::Bytes1;
        #[allow(non_camel_case_types)]
        pub type bytes2 = liquid_primitives::types::Bytes2;
        #[allow(non_camel_case_types)]
        pub type bytes3 = liquid_primitives::types::Bytes3;
        #[allow(non_camel_case_types)]
        pub type bytes4 = liquid_primitives::types::Bytes4;
        #[allow(non_camel_case_types)]
        pub type bytes5 = liquid_primitives::types::Bytes5;
        #[allow(non_camel_case_types)]
        pub type bytes6 = liquid_primitives::types::Bytes6;
        #[allow(non_camel_case_types)]
        pub type bytes7 = liquid_primitives::types::Bytes7;
        #[allow(non_camel_case_types)]
        pub type bytes8 = liquid_primitives::types::Bytes8;
        #[allow(non_camel_case_types)]
        pub type bytes9 = liquid_primitives::types::Bytes9;
        #[allow(non_camel_case_types)]
        pub type bytes10 = liquid_primitives::types::Bytes10;
        #[allow(non_camel_case_types)]
        pub type bytes11 = liquid_primitives::types::Bytes11;
        #[allow(non_camel_case_types)]
        pub type bytes12 = liquid_primitives::types::Bytes12;
        #[allow(non_camel_case_types)]
        pub type bytes13 = liquid_primitives::types::Bytes13;
        #[allow(non_camel_case_types)]
        pub type bytes14 = liquid_primitives::types::Bytes14;
        #[allow(non_camel_case_types)]
        pub type bytes15 = liquid_primitives::types::Bytes15;
        #[allow(non_camel_case_types)]
        pub type bytes16 = liquid_primitives::types::Bytes16;
        #[allow(non_camel_case_types)]
        pub type bytes17 = liquid_primitives::types::Bytes17;
        #[allow(non_camel_case_types)]
        pub type bytes18 = liquid_primitives::types::Bytes18;
        #[allow(non_camel_case_types)]
        pub type bytes19 = liquid_primitives::types::Bytes19;
        #[allow(non_camel_case_types)]
        pub type bytes20 = liquid_primitives::types::Bytes20;
        #[allow(non_camel_case_types)]
        pub type bytes21 = liquid_primitives::types::Bytes21;
        #[allow(non_camel_case_types)]
        pub type bytes22 = liquid_primitives::types::Bytes22;
        #[allow(non_camel_case_types)]
        pub type bytes23 = liquid_primitives::types::Bytes23;
        #[allow(non_camel_case_types)]
        pub type bytes24 = liquid_primitives::types::Bytes24;
        #[allow(non_camel_case_types)]
        pub type bytes25 = liquid_primitives::types::Bytes25;
        #[allow(non_camel_case_types)]
        pub type bytes26 = liquid_primitives::types::Bytes26;
        #[allow(non_camel_case_types)]
        pub type bytes27 = liquid_primitives::types::Bytes27;
        #[allow(non_camel_case_types)]
        pub type bytes28 = liquid_primitives::types::Bytes28;
        #[allow(non_camel_case_types)]
        pub type bytes29 = liquid_primitives::types::Bytes29;
        #[allow(non_camel_case_types)]
        pub type bytes30 = liquid_primitives::types::Bytes30;
        #[allow(non_camel_case_types)]
        pub type bytes31 = liquid_primitives::types::Bytes31;
        #[allow(non_camel_case_types)]
        pub type bytes32 = liquid_primitives::types::Bytes32;
        pub use liquid_primitives::types::u256;
        pub use liquid_primitives::types::i256;
        pub use liquid_prelude::string::String;
        pub type Vec<T> = liquid_prelude::vec::Vec<T>;
        #[allow(non_camel_case_types)]
        pub struct __liquid_interface {
            __liquid_address: liquid_primitives::types::Address,
            pub set: set,
        }
        impl __liquid_interface {
            pub fn at(addr: liquid_primitives::types::Address) -> Self {
                Self {
                    __liquid_address: addr,
                    set: addr.into(),
                }
            }
        }
        impl From<liquid_primitives::types::Address> for __liquid_interface {
            fn from(addr: liquid_primitives::types::Address) -> Self {
                Self::at(addr)
            }
        }
        impl scale::Decode for __liquid_interface {
            fn decode<I: scale::Input>(value: &mut I) -> Result<Self, scale::Error> {
                let __liquid_address = liquid_primitives::types::Address::decode(value)?;
                Ok(Self {
                    __liquid_address,
                    set: __liquid_address.into(),
                })
            }
        }
        impl scale::Encode for __liquid_interface {
            fn encode(&self) -> Vec<u8> {
                self.__liquid_address.encode()
            }
        }
        #[allow(non_camel_case_types)]
        pub struct set {
            __liquid_address: liquid_primitives::types::Address,
        }
        impl From<liquid_primitives::types::Address> for set {
            fn from(__liquid_address: liquid_primitives::types::Address) -> Self {
                Self { __liquid_address }
            }
        }
        #[allow(non_snake_case)]
        fn set_0(
            __liquid_address: &liquid_primitives::types::Address,
            key: String,
            value: i256,
        ) -> Option<()> {
            #[allow(dead_code)]
            type Input = (
                <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                <i256 as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
            );
            #[allow(dead_code)]
            const SET_0: liquid_primitives::Selector = {
                const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 3usize + 2;
                const SIG: [u8; SIG_LEN] =
                    liquid_ty_mapping::composite::<Input, SIG_LEN>(&[115u8, 101u8, 116u8]);
                let hash = liquid_primitives::hash::hash(&SIG);
                [hash[0], hash[1], hash[2], hash[3]]
            };
            let mut encoded = SET_0.to_vec();
            encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key, value)));
            liquid_core::env::call::<()>(&__liquid_address, &encoded).ok()
        }
        impl FnOnce<(String, i256)> for set {
            type Output = Option<()>;
            extern "rust-call" fn call_once(self, args: (String, i256)) -> Self::Output {
                self.call(args)
            }
        }
        impl FnMut<(String, i256)> for set {
            extern "rust-call" fn call_mut(&mut self, args: (String, i256)) -> Self::Output {
                self.call(args)
            }
        }
        impl Fn<(String, i256)> for set {
            extern "rust-call" fn call(&self, (key, value): (String, i256)) -> Self::Output {
                set_0(&self.__liquid_address, key, value)
            }
        }
        #[allow(non_snake_case)]
        fn set_1(
            __liquid_address: &liquid_primitives::types::Address,
            key: String,
            value: u256,
        ) -> Option<()> {
            #[allow(dead_code)]
            type Input = (
                <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                <u256 as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
            );
            #[allow(dead_code)]
            const SET_1: liquid_primitives::Selector = {
                const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 3usize + 2;
                const SIG: [u8; SIG_LEN] =
                    liquid_ty_mapping::composite::<Input, SIG_LEN>(&[115u8, 101u8, 116u8]);
                let hash = liquid_primitives::hash::hash(&SIG);
                [hash[0], hash[1], hash[2], hash[3]]
            };
            let mut encoded = SET_1.to_vec();
            encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key, value)));
            liquid_core::env::call::<()>(&__liquid_address, &encoded).ok()
        }
        impl FnOnce<(String, u256)> for set {
            type Output = Option<()>;
            extern "rust-call" fn call_once(self, args: (String, u256)) -> Self::Output {
                self.call(args)
            }
        }
        impl FnMut<(String, u256)> for set {
            extern "rust-call" fn call_mut(&mut self, args: (String, u256)) -> Self::Output {
                self.call(args)
            }
        }
        impl Fn<(String, u256)> for set {
            extern "rust-call" fn call(&self, (key, value): (String, u256)) -> Self::Output {
                set_1(&self.__liquid_address, key, value)
            }
        }
        #[allow(non_snake_case)]
        fn set_2(
            __liquid_address: &liquid_primitives::types::Address,
            key: String,
            value: address,
        ) -> Option<()> {
            #[allow(dead_code)]
            type Input = (
                <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                <address as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
            );
            #[allow(dead_code)]
            const SET_2: liquid_primitives::Selector = {
                const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 3usize + 2;
                const SIG: [u8; SIG_LEN] =
                    liquid_ty_mapping::composite::<Input, SIG_LEN>(&[115u8, 101u8, 116u8]);
                let hash = liquid_primitives::hash::hash(&SIG);
                [hash[0], hash[1], hash[2], hash[3]]
            };
            let mut encoded = SET_2.to_vec();
            encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key, value)));
            liquid_core::env::call::<()>(&__liquid_address, &encoded).ok()
        }
        impl FnOnce<(String, address)> for set {
            type Output = Option<()>;
            extern "rust-call" fn call_once(self, args: (String, address)) -> Self::Output {
                self.call(args)
            }
        }
        impl FnMut<(String, address)> for set {
            extern "rust-call" fn call_mut(&mut self, args: (String, address)) -> Self::Output {
                self.call(args)
            }
        }
        impl Fn<(String, address)> for set {
            extern "rust-call" fn call(&self, (key, value): (String, address)) -> Self::Output {
                set_2(&self.__liquid_address, key, value)
            }
        }
        #[allow(non_snake_case)]
        fn set_3(
            __liquid_address: &liquid_primitives::types::Address,
            key: String,
            value: String,
        ) -> Option<()> {
            #[allow(dead_code)]
            type Input = (
                <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
            );
            #[allow(dead_code)]
            const SET_3: liquid_primitives::Selector = {
                const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 3usize + 2;
                const SIG: [u8; SIG_LEN] =
                    liquid_ty_mapping::composite::<Input, SIG_LEN>(&[115u8, 101u8, 116u8]);
                let hash = liquid_primitives::hash::hash(&SIG);
                [hash[0], hash[1], hash[2], hash[3]]
            };
            let mut encoded = SET_3.to_vec();
            encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key, value)));
            liquid_core::env::call::<()>(&__liquid_address, &encoded).ok()
        }
        impl FnOnce<(String, String)> for set {
            type Output = Option<()>;
            extern "rust-call" fn call_once(self, args: (String, String)) -> Self::Output {
                self.call(args)
            }
        }
        impl FnMut<(String, String)> for set {
            extern "rust-call" fn call_mut(&mut self, args: (String, String)) -> Self::Output {
                self.call(args)
            }
        }
        impl Fn<(String, String)> for set {
            extern "rust-call" fn call(&self, (key, value): (String, String)) -> Self::Output {
                set_3(&self.__liquid_address, key, value)
            }
        }
        impl Into<liquid_primitives::types::Address> for __liquid_interface {
            fn into(self) -> liquid_primitives::types::Address {
                self.__liquid_address
            }
        }
        impl liquid_ty_mapping::MappingToSolidityType for __liquid_interface {
            const MAPPED_TYPE_NAME : [ u8 ; liquid_ty_mapping :: MAX_LENGTH_OF_MAPPED_TYPE_NAME ] = < liquid_primitives :: types :: Address as liquid_ty_mapping :: MappingToSolidityType > :: MAPPED_TYPE_NAME ;
        }
        impl liquid_abi_codec::TypeInfo for __liquid_interface {}
        impl liquid_abi_codec::MediateEncode for __liquid_interface {
            fn encode(&self) -> liquid_abi_codec::Mediate {
                self.__liquid_address.encode()
            }
        }
        impl liquid_abi_codec::MediateDecode for __liquid_interface {
            fn decode(
                slices: &[liquid_abi_codec::Word],
                offset: usize,
            ) -> Result<liquid_abi_codec::DecodeResult<Self>, liquid_primitives::Error>
            {
                let decode_result =
                    <liquid_primitives::types::Address as liquid_abi_codec::MediateDecode>::decode(
                        slices, offset,
                    )?;
                let value = Self {
                    __liquid_address: decode_result.value,
                    set: decode_result.value.into(),
                };
                Ok(liquid_abi_codec::DecodeResult {
                    value,
                    new_offset: decode_result.new_offset,
                })
            }
        }
        impl liquid_lang::You_Should_Use_An_Valid_Parameter_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Return_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Input_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Field_Data_Type for __liquid_interface {}
        impl __liquid_interface {
            #[allow(non_snake_case)]
            pub fn getAddress(
                &self,
                key: String,
            ) -> Option<<address as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T>
            {
                #[allow(dead_code)]
                type Input = (<String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,);
                #[allow(dead_code)]
                const GET_ADDRESS: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 10usize + 2;
                    const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<Input, SIG_LEN>(&[
                        103u8, 101u8, 116u8, 65u8, 100u8, 100u8, 114u8, 101u8, 115u8, 115u8,
                    ]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = GET_ADDRESS.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key,)));
                liquid_core::env::call::<
                    <address as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
            #[allow(non_snake_case)]
            pub fn getInt(
                &self,
                key: String,
            ) -> Option<<i256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T> {
                #[allow(dead_code)]
                type Input = (<String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,);
                #[allow(dead_code)]
                const GET_INT: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 6usize + 2;
                    const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<Input, SIG_LEN>(&[
                        103u8, 101u8, 116u8, 73u8, 110u8, 116u8,
                    ]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = GET_INT.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key,)));
                liquid_core::env::call::<
                    <i256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
            #[allow(non_snake_case)]
            pub fn getString(
                &self,
                key: String,
            ) -> Option<<String as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T>
            {
                #[allow(dead_code)]
                type Input = (<String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,);
                #[allow(dead_code)]
                const GET_STRING: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 9usize + 2;
                    const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<Input, SIG_LEN>(&[
                        103u8, 101u8, 116u8, 83u8, 116u8, 114u8, 105u8, 110u8, 103u8,
                    ]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = GET_STRING.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key,)));
                liquid_core::env::call::<
                    <String as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
            #[allow(non_snake_case)]
            pub fn getUint(
                &self,
                key: String,
            ) -> Option<<u256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T> {
                #[allow(dead_code)]
                type Input = (<String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,);
                #[allow(dead_code)]
                const GET_UINT: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 7usize + 2;
                    const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<Input, SIG_LEN>(&[
                        103u8, 101u8, 116u8, 85u8, 105u8, 110u8, 116u8,
                    ]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = GET_UINT.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(key,)));
                liquid_core::env::call::<
                    <u256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
        }
    }
    pub type Entry = __liquid_private::__liquid_interface;
}
mod kv_table {
    use super::entry::*;
    mod __liquid_private {
        use super::*;
        #[allow(non_camel_case_types)]
        pub type address = liquid_primitives::types::Address;
        #[allow(non_camel_case_types)]
        pub type bytes = liquid_primitives::types::Bytes;
        #[allow(non_camel_case_types)]
        pub type byte = liquid_primitives::types::Byte;
        #[allow(non_camel_case_types)]
        pub type bytes1 = liquid_primitives::types::Bytes1;
        #[allow(non_camel_case_types)]
        pub type bytes2 = liquid_primitives::types::Bytes2;
        #[allow(non_camel_case_types)]
        pub type bytes3 = liquid_primitives::types::Bytes3;
        #[allow(non_camel_case_types)]
        pub type bytes4 = liquid_primitives::types::Bytes4;
        #[allow(non_camel_case_types)]
        pub type bytes5 = liquid_primitives::types::Bytes5;
        #[allow(non_camel_case_types)]
        pub type bytes6 = liquid_primitives::types::Bytes6;
        #[allow(non_camel_case_types)]
        pub type bytes7 = liquid_primitives::types::Bytes7;
        #[allow(non_camel_case_types)]
        pub type bytes8 = liquid_primitives::types::Bytes8;
        #[allow(non_camel_case_types)]
        pub type bytes9 = liquid_primitives::types::Bytes9;
        #[allow(non_camel_case_types)]
        pub type bytes10 = liquid_primitives::types::Bytes10;
        #[allow(non_camel_case_types)]
        pub type bytes11 = liquid_primitives::types::Bytes11;
        #[allow(non_camel_case_types)]
        pub type bytes12 = liquid_primitives::types::Bytes12;
        #[allow(non_camel_case_types)]
        pub type bytes13 = liquid_primitives::types::Bytes13;
        #[allow(non_camel_case_types)]
        pub type bytes14 = liquid_primitives::types::Bytes14;
        #[allow(non_camel_case_types)]
        pub type bytes15 = liquid_primitives::types::Bytes15;
        #[allow(non_camel_case_types)]
        pub type bytes16 = liquid_primitives::types::Bytes16;
        #[allow(non_camel_case_types)]
        pub type bytes17 = liquid_primitives::types::Bytes17;
        #[allow(non_camel_case_types)]
        pub type bytes18 = liquid_primitives::types::Bytes18;
        #[allow(non_camel_case_types)]
        pub type bytes19 = liquid_primitives::types::Bytes19;
        #[allow(non_camel_case_types)]
        pub type bytes20 = liquid_primitives::types::Bytes20;
        #[allow(non_camel_case_types)]
        pub type bytes21 = liquid_primitives::types::Bytes21;
        #[allow(non_camel_case_types)]
        pub type bytes22 = liquid_primitives::types::Bytes22;
        #[allow(non_camel_case_types)]
        pub type bytes23 = liquid_primitives::types::Bytes23;
        #[allow(non_camel_case_types)]
        pub type bytes24 = liquid_primitives::types::Bytes24;
        #[allow(non_camel_case_types)]
        pub type bytes25 = liquid_primitives::types::Bytes25;
        #[allow(non_camel_case_types)]
        pub type bytes26 = liquid_primitives::types::Bytes26;
        #[allow(non_camel_case_types)]
        pub type bytes27 = liquid_primitives::types::Bytes27;
        #[allow(non_camel_case_types)]
        pub type bytes28 = liquid_primitives::types::Bytes28;
        #[allow(non_camel_case_types)]
        pub type bytes29 = liquid_primitives::types::Bytes29;
        #[allow(non_camel_case_types)]
        pub type bytes30 = liquid_primitives::types::Bytes30;
        #[allow(non_camel_case_types)]
        pub type bytes31 = liquid_primitives::types::Bytes31;
        #[allow(non_camel_case_types)]
        pub type bytes32 = liquid_primitives::types::Bytes32;
        pub use liquid_primitives::types::u256;
        pub use liquid_primitives::types::i256;
        pub use liquid_prelude::string::String;
        pub type Vec<T> = liquid_prelude::vec::Vec<T>;
        #[allow(non_camel_case_types)]
        pub struct __liquid_interface {
            __liquid_address: liquid_primitives::types::Address,
        }
        impl __liquid_interface {
            pub fn at(addr: liquid_primitives::types::Address) -> Self {
                Self {
                    __liquid_address: addr,
                }
            }
        }
        impl From<liquid_primitives::types::Address> for __liquid_interface {
            fn from(addr: liquid_primitives::types::Address) -> Self {
                Self::at(addr)
            }
        }
        impl scale::Decode for __liquid_interface {
            fn decode<I: scale::Input>(value: &mut I) -> Result<Self, scale::Error> {
                let __liquid_address = liquid_primitives::types::Address::decode(value)?;
                Ok(Self { __liquid_address })
            }
        }
        impl scale::Encode for __liquid_interface {
            fn encode(&self) -> Vec<u8> {
                self.__liquid_address.encode()
            }
        }
        impl Into<liquid_primitives::types::Address> for __liquid_interface {
            fn into(self) -> liquid_primitives::types::Address {
                self.__liquid_address
            }
        }
        impl liquid_ty_mapping::MappingToSolidityType for __liquid_interface {
            const MAPPED_TYPE_NAME : [ u8 ; liquid_ty_mapping :: MAX_LENGTH_OF_MAPPED_TYPE_NAME ] = < liquid_primitives :: types :: Address as liquid_ty_mapping :: MappingToSolidityType > :: MAPPED_TYPE_NAME ;
        }
        impl liquid_abi_codec::TypeInfo for __liquid_interface {}
        impl liquid_abi_codec::MediateEncode for __liquid_interface {
            fn encode(&self) -> liquid_abi_codec::Mediate {
                self.__liquid_address.encode()
            }
        }
        impl liquid_abi_codec::MediateDecode for __liquid_interface {
            fn decode(
                slices: &[liquid_abi_codec::Word],
                offset: usize,
            ) -> Result<liquid_abi_codec::DecodeResult<Self>, liquid_primitives::Error>
            {
                let decode_result =
                    <liquid_primitives::types::Address as liquid_abi_codec::MediateDecode>::decode(
                        slices, offset,
                    )?;
                let value = Self {
                    __liquid_address: decode_result.value,
                };
                Ok(liquid_abi_codec::DecodeResult {
                    value,
                    new_offset: decode_result.new_offset,
                })
            }
        }
        impl liquid_lang::You_Should_Use_An_Valid_Parameter_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Return_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Input_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Field_Data_Type for __liquid_interface {}
        impl __liquid_interface {
            #[allow(non_snake_case)]
            pub fn get(
                &self,
                primary_key: String,
            ) -> Option<<(bool, Entry) as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T>
            {
                #[allow(dead_code)]
                type Input = (<String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,);
                #[allow(dead_code)]
                const GET: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 3usize + 2;
                    const SIG: [u8; SIG_LEN] =
                        liquid_ty_mapping::composite::<Input, SIG_LEN>(&[103u8, 101u8, 116u8]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = GET.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(primary_key,)));
                liquid_core::env::call::<
                    <(bool, Entry) as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
            #[allow(non_snake_case)]
            pub fn newEntry(
                &self,
            ) -> Option<<Entry as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T>
            {
                #[allow(dead_code)]
                type Input = ();
                #[allow(dead_code)]
                const NEW_ENTRY: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 8usize + 2;
                    const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<Input, SIG_LEN>(&[
                        110u8, 101u8, 119u8, 69u8, 110u8, 116u8, 114u8, 121u8,
                    ]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = NEW_ENTRY.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&()));
                liquid_core::env::call::<
                    <Entry as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
            #[allow(non_snake_case)]
            pub fn set(
                &self,
                primary_key: String,
                entry: Entry,
            ) -> Option<<i256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T> {
                #[allow(dead_code)]
                type Input = (
                    <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                    <Entry as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                );
                #[allow(dead_code)]
                const SET: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 3usize + 2;
                    const SIG: [u8; SIG_LEN] =
                        liquid_ty_mapping::composite::<Input, SIG_LEN>(&[115u8, 101u8, 116u8]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = SET.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(
                    primary_key,
                    entry,
                )));
                liquid_core::env::call::<
                    <i256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
        }
    }
    pub type KvTable = __liquid_private::__liquid_interface;
}
mod kv_table_factory {
    use super::kv_table::*;
    mod __liquid_private {
        use super::*;
        #[allow(non_camel_case_types)]
        pub type address = liquid_primitives::types::Address;
        #[allow(non_camel_case_types)]
        pub type bytes = liquid_primitives::types::Bytes;
        #[allow(non_camel_case_types)]
        pub type byte = liquid_primitives::types::Byte;
        #[allow(non_camel_case_types)]
        pub type bytes1 = liquid_primitives::types::Bytes1;
        #[allow(non_camel_case_types)]
        pub type bytes2 = liquid_primitives::types::Bytes2;
        #[allow(non_camel_case_types)]
        pub type bytes3 = liquid_primitives::types::Bytes3;
        #[allow(non_camel_case_types)]
        pub type bytes4 = liquid_primitives::types::Bytes4;
        #[allow(non_camel_case_types)]
        pub type bytes5 = liquid_primitives::types::Bytes5;
        #[allow(non_camel_case_types)]
        pub type bytes6 = liquid_primitives::types::Bytes6;
        #[allow(non_camel_case_types)]
        pub type bytes7 = liquid_primitives::types::Bytes7;
        #[allow(non_camel_case_types)]
        pub type bytes8 = liquid_primitives::types::Bytes8;
        #[allow(non_camel_case_types)]
        pub type bytes9 = liquid_primitives::types::Bytes9;
        #[allow(non_camel_case_types)]
        pub type bytes10 = liquid_primitives::types::Bytes10;
        #[allow(non_camel_case_types)]
        pub type bytes11 = liquid_primitives::types::Bytes11;
        #[allow(non_camel_case_types)]
        pub type bytes12 = liquid_primitives::types::Bytes12;
        #[allow(non_camel_case_types)]
        pub type bytes13 = liquid_primitives::types::Bytes13;
        #[allow(non_camel_case_types)]
        pub type bytes14 = liquid_primitives::types::Bytes14;
        #[allow(non_camel_case_types)]
        pub type bytes15 = liquid_primitives::types::Bytes15;
        #[allow(non_camel_case_types)]
        pub type bytes16 = liquid_primitives::types::Bytes16;
        #[allow(non_camel_case_types)]
        pub type bytes17 = liquid_primitives::types::Bytes17;
        #[allow(non_camel_case_types)]
        pub type bytes18 = liquid_primitives::types::Bytes18;
        #[allow(non_camel_case_types)]
        pub type bytes19 = liquid_primitives::types::Bytes19;
        #[allow(non_camel_case_types)]
        pub type bytes20 = liquid_primitives::types::Bytes20;
        #[allow(non_camel_case_types)]
        pub type bytes21 = liquid_primitives::types::Bytes21;
        #[allow(non_camel_case_types)]
        pub type bytes22 = liquid_primitives::types::Bytes22;
        #[allow(non_camel_case_types)]
        pub type bytes23 = liquid_primitives::types::Bytes23;
        #[allow(non_camel_case_types)]
        pub type bytes24 = liquid_primitives::types::Bytes24;
        #[allow(non_camel_case_types)]
        pub type bytes25 = liquid_primitives::types::Bytes25;
        #[allow(non_camel_case_types)]
        pub type bytes26 = liquid_primitives::types::Bytes26;
        #[allow(non_camel_case_types)]
        pub type bytes27 = liquid_primitives::types::Bytes27;
        #[allow(non_camel_case_types)]
        pub type bytes28 = liquid_primitives::types::Bytes28;
        #[allow(non_camel_case_types)]
        pub type bytes29 = liquid_primitives::types::Bytes29;
        #[allow(non_camel_case_types)]
        pub type bytes30 = liquid_primitives::types::Bytes30;
        #[allow(non_camel_case_types)]
        pub type bytes31 = liquid_primitives::types::Bytes31;
        #[allow(non_camel_case_types)]
        pub type bytes32 = liquid_primitives::types::Bytes32;
        pub use liquid_primitives::types::u256;
        pub use liquid_primitives::types::i256;
        pub use liquid_prelude::string::String;
        pub type Vec<T> = liquid_prelude::vec::Vec<T>;
        #[allow(non_camel_case_types)]
        pub struct __liquid_interface {
            __liquid_address: liquid_primitives::types::Address,
        }
        impl __liquid_interface {
            pub fn at(addr: liquid_primitives::types::Address) -> Self {
                Self {
                    __liquid_address: addr,
                }
            }
        }
        impl From<liquid_primitives::types::Address> for __liquid_interface {
            fn from(addr: liquid_primitives::types::Address) -> Self {
                Self::at(addr)
            }
        }
        impl scale::Decode for __liquid_interface {
            fn decode<I: scale::Input>(value: &mut I) -> Result<Self, scale::Error> {
                let __liquid_address = liquid_primitives::types::Address::decode(value)?;
                Ok(Self { __liquid_address })
            }
        }
        impl scale::Encode for __liquid_interface {
            fn encode(&self) -> Vec<u8> {
                self.__liquid_address.encode()
            }
        }
        impl Into<liquid_primitives::types::Address> for __liquid_interface {
            fn into(self) -> liquid_primitives::types::Address {
                self.__liquid_address
            }
        }
        impl liquid_ty_mapping::MappingToSolidityType for __liquid_interface {
            const MAPPED_TYPE_NAME : [ u8 ; liquid_ty_mapping :: MAX_LENGTH_OF_MAPPED_TYPE_NAME ] = < liquid_primitives :: types :: Address as liquid_ty_mapping :: MappingToSolidityType > :: MAPPED_TYPE_NAME ;
        }
        impl liquid_abi_codec::TypeInfo for __liquid_interface {}
        impl liquid_abi_codec::MediateEncode for __liquid_interface {
            fn encode(&self) -> liquid_abi_codec::Mediate {
                self.__liquid_address.encode()
            }
        }
        impl liquid_abi_codec::MediateDecode for __liquid_interface {
            fn decode(
                slices: &[liquid_abi_codec::Word],
                offset: usize,
            ) -> Result<liquid_abi_codec::DecodeResult<Self>, liquid_primitives::Error>
            {
                let decode_result =
                    <liquid_primitives::types::Address as liquid_abi_codec::MediateDecode>::decode(
                        slices, offset,
                    )?;
                let value = Self {
                    __liquid_address: decode_result.value,
                };
                Ok(liquid_abi_codec::DecodeResult {
                    value,
                    new_offset: decode_result.new_offset,
                })
            }
        }
        impl liquid_lang::You_Should_Use_An_Valid_Parameter_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Return_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Input_Type for __liquid_interface {}
        impl liquid_lang::You_Should_Use_An_Valid_Field_Data_Type for __liquid_interface {}
        impl __liquid_interface {
            #[allow(non_snake_case)]
            pub fn createTable(
                &self,
                name: String,
                primary_key: String,
                fields: String,
            ) -> Option<<i256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T> {
                #[allow(dead_code)]
                type Input = (
                    <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                    <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                    <String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,
                );
                #[allow(dead_code)]
                const CREATE_TABLE: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 11usize + 2;
                    const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<Input, SIG_LEN>(&[
                        99u8, 114u8, 101u8, 97u8, 116u8, 101u8, 84u8, 97u8, 98u8, 108u8, 101u8,
                    ]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = CREATE_TABLE.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(
                    name,
                    primary_key,
                    fields,
                )));
                liquid_core::env::call::<
                    <i256 as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
            #[allow(non_snake_case)]
            pub fn openTable(
                &self,
                name: String,
            ) -> Option<<KvTable as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T>
            {
                #[allow(dead_code)]
                type Input = (<String as liquid_lang::You_Should_Use_An_Valid_Input_Type>::T,);
                #[allow(dead_code)]
                const OPEN_TABLE: liquid_primitives::Selector = {
                    const SIG_LEN: usize = liquid_ty_mapping::len::<Input>() + 9usize + 2;
                    const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<Input, SIG_LEN>(&[
                        111u8, 112u8, 101u8, 110u8, 84u8, 97u8, 98u8, 108u8, 101u8,
                    ]);
                    let hash = liquid_primitives::hash::hash(&SIG);
                    [hash[0], hash[1], hash[2], hash[3]]
                };
                let mut encoded = OPEN_TABLE.to_vec();
                encoded.extend(<Input as liquid_abi_codec::Encode>::encode(&(name,)));
                liquid_core::env::call::<
                    <KvTable as liquid_lang::You_Should_Use_An_Valid_Return_Type>::T,
                >(&self.__liquid_address, &encoded)
                .ok()
            }
        }
    }
    pub type KvTableFactory = __liquid_private::__liquid_interface;
}
mod kv_table_test {
    use liquid_lang::intrinsics::*;
    #[allow(non_camel_case_types)]
    pub type address = liquid_primitives::types::Address;
    #[allow(non_camel_case_types)]
    pub type bytes = liquid_primitives::types::Bytes;
    #[allow(non_camel_case_types)]
    pub type byte = liquid_primitives::types::Byte;
    #[allow(non_camel_case_types)]
    pub type bytes1 = liquid_primitives::types::Bytes1;
    #[allow(non_camel_case_types)]
    pub type bytes2 = liquid_primitives::types::Bytes2;
    #[allow(non_camel_case_types)]
    pub type bytes3 = liquid_primitives::types::Bytes3;
    #[allow(non_camel_case_types)]
    pub type bytes4 = liquid_primitives::types::Bytes4;
    #[allow(non_camel_case_types)]
    pub type bytes5 = liquid_primitives::types::Bytes5;
    #[allow(non_camel_case_types)]
    pub type bytes6 = liquid_primitives::types::Bytes6;
    #[allow(non_camel_case_types)]
    pub type bytes7 = liquid_primitives::types::Bytes7;
    #[allow(non_camel_case_types)]
    pub type bytes8 = liquid_primitives::types::Bytes8;
    #[allow(non_camel_case_types)]
    pub type bytes9 = liquid_primitives::types::Bytes9;
    #[allow(non_camel_case_types)]
    pub type bytes10 = liquid_primitives::types::Bytes10;
    #[allow(non_camel_case_types)]
    pub type bytes11 = liquid_primitives::types::Bytes11;
    #[allow(non_camel_case_types)]
    pub type bytes12 = liquid_primitives::types::Bytes12;
    #[allow(non_camel_case_types)]
    pub type bytes13 = liquid_primitives::types::Bytes13;
    #[allow(non_camel_case_types)]
    pub type bytes14 = liquid_primitives::types::Bytes14;
    #[allow(non_camel_case_types)]
    pub type bytes15 = liquid_primitives::types::Bytes15;
    #[allow(non_camel_case_types)]
    pub type bytes16 = liquid_primitives::types::Bytes16;
    #[allow(non_camel_case_types)]
    pub type bytes17 = liquid_primitives::types::Bytes17;
    #[allow(non_camel_case_types)]
    pub type bytes18 = liquid_primitives::types::Bytes18;
    #[allow(non_camel_case_types)]
    pub type bytes19 = liquid_primitives::types::Bytes19;
    #[allow(non_camel_case_types)]
    pub type bytes20 = liquid_primitives::types::Bytes20;
    #[allow(non_camel_case_types)]
    pub type bytes21 = liquid_primitives::types::Bytes21;
    #[allow(non_camel_case_types)]
    pub type bytes22 = liquid_primitives::types::Bytes22;
    #[allow(non_camel_case_types)]
    pub type bytes23 = liquid_primitives::types::Bytes23;
    #[allow(non_camel_case_types)]
    pub type bytes24 = liquid_primitives::types::Bytes24;
    #[allow(non_camel_case_types)]
    pub type bytes25 = liquid_primitives::types::Bytes25;
    #[allow(non_camel_case_types)]
    pub type bytes26 = liquid_primitives::types::Bytes26;
    #[allow(non_camel_case_types)]
    pub type bytes27 = liquid_primitives::types::Bytes27;
    #[allow(non_camel_case_types)]
    pub type bytes28 = liquid_primitives::types::Bytes28;
    #[allow(non_camel_case_types)]
    pub type bytes29 = liquid_primitives::types::Bytes29;
    #[allow(non_camel_case_types)]
    pub type bytes30 = liquid_primitives::types::Bytes30;
    #[allow(non_camel_case_types)]
    pub type bytes31 = liquid_primitives::types::Bytes31;
    #[allow(non_camel_case_types)]
    pub type bytes32 = liquid_primitives::types::Bytes32;
    pub use liquid_primitives::types::u256;
    pub use liquid_primitives::types::i256;
    pub use liquid_prelude::string::String;
    pub type Vec<T> = liquid_prelude::vec::Vec<T>;
    mod __liquid_private {
        use super::*;
        mod __liquid_storage {
            #[allow(unused_imports)]
            use super::*;
            pub struct Storage { pub table_factory : < storage :: Value < KvTableFactory > as liquid_core :: storage :: You_Should_Use_A_Container_To_Wrap_Your_State_Field_In_Storage > :: T , }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Storage {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Storage {
                            table_factory: ref __self_0_0,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("Storage");
                            let _ = debug_trait_builder.field("table_factory", &&(*__self_0_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl liquid_core::storage::Flush for Storage {
                fn flush(&mut self) {
                    liquid_core::storage::Flush::flush(&mut self.table_factory);
                }
            }
            impl Storage {
                #[allow(unused)]
                const STORAGE_KEYS: [&'static str; 1usize] = ["table_factory"];
                #[allow(unused)]
                pub fn env(&self) -> liquid_lang::EnvAccess {
                    liquid_lang::EnvAccess {}
                }
            }
            impl liquid_core::storage::New for Storage {
                fn new() -> Self {
                    Self {
                        table_factory: liquid_core::storage::Bind::bind_with(
                            Self::STORAGE_KEYS[0usize].as_bytes(),
                        ),
                    }
                }
            }
        }
        pub use __liquid_storage::Storage;
        const _: () = {
            impl Storage {
                pub fn new(&mut self) {
                    self.table_factory
                        .initialize(KvTableFactory::at("0x1010".parse().unwrap()));
                    self.table_factory.createTable(
                        String::from(Self::TABLE_NAME),
                        String::from("id"),
                        String::from("item_price,item_name"),
                    );
                }
                pub fn get(&self, id: String) -> (bool, i256, String) {
                    let table = self
                        .table_factory
                        .openTable(String::from(Self::TABLE_NAME))
                        .unwrap();
                    let (ok, entry) = table.get(id).unwrap();
                    let (item_price, item_name) = if ok {
                        (
                            entry.getInt("item_price".to_string()).unwrap(),
                            entry.getString("item_name".to_string()).unwrap(),
                        )
                    } else {
                        (0.into(), Default::default())
                    };
                    (ok, item_price, item_name)
                }
                pub fn set(&mut self, id: String, item_price: i256, item_name: String) -> i256 {
                    let table = self
                        .table_factory
                        .openTable(String::from(Self::TABLE_NAME))
                        .unwrap();
                    let entry = table.newEntry().unwrap();
                    (entry.set)(String::from("id"), id.clone());
                    (entry.set)(String::from("item_price"), item_price);
                    (entry.set)(String::from("item_name"), item_name);
                    let count = table.set(id, entry).unwrap();
                    self.env().emit(SetResult {
                        count: count.clone(),
                    });
                    count
                }
            }
            impl Storage {
                const TABLE_NAME: &'static str = "t_kvtest";
            }
        };
        mod __liquid_event {
            #[allow(unused_imports)]
            use super::*;
            pub struct EventSigHelper<S, T> {
                marker_s: core::marker::PhantomData<fn() -> S>,
                marker_t: core::marker::PhantomData<fn() -> T>,
            }
            impl liquid_primitives::Topics for SetResult {
                fn topics(&self) -> liquid_prelude::vec::Vec<liquid_primitives::types::Hash> {
                    [{
                        const SIG_LEN: usize = liquid_ty_mapping::len::<(
                            <i256 as liquid_lang::You_Should_Use_An_Valid_Event_Data_Type>::T,
                        )>() + 9usize
                            + 2;
                        const SIG: [u8; SIG_LEN] = liquid_ty_mapping::composite::<
                            (<i256 as liquid_lang::You_Should_Use_An_Valid_Event_Data_Type>::T,),
                            SIG_LEN,
                        >(&[
                            83u8, 101u8, 116u8, 82u8, 101u8, 115u8, 117u8, 108u8, 116u8,
                        ]);
                        liquid_primitives::hash::hash(&SIG)
                    }
                    .into()]
                    .to_vec()
                }
            }
            impl liquid_abi_codec::Encode for SetResult {
                fn encode(&self) -> liquid_prelude::vec::Vec<u8> {
                    #[allow(unused_mut)]
                    let mut mediates = Vec::<liquid_abi_codec::Mediate>::new();
                    mediates.push(<i256 as liquid_abi_codec::MediateEncode>::encode(
                        &self.count,
                    ));
                    liquid_abi_codec::encode_head_tail(&mediates)
                        .iter()
                        .flat_map(|word| word.to_vec())
                        .collect()
                }
            }
            pub enum Event {
                SetResult(SetResult),
            }
            impl From<SetResult> for Event {
                fn from(event: SetResult) -> Self {
                    Event::SetResult(event)
                }
            }
            impl liquid_primitives::Topics for Event {
                fn topics(&self) -> liquid_prelude::vec::Vec<liquid_primitives::types::Hash> {
                    match self {
                        Event::SetResult(event) => event.topics(),
                    }
                }
            }
            impl liquid_abi_codec::Encode for Event {
                fn encode(&self) -> Vec<u8> {
                    match self {
                        Event::SetResult(event) => event.encode(),
                    }
                }
            }
            pub trait Emit {
                type Event;
                fn emit<E>(self, event: E)
                where
                    E: Into<Self::Event>;
            }
            impl Emit for liquid_lang::EnvAccess {
                type Event = Event;
                fn emit<E>(self, event: E)
                where
                    E: Into<Self::Event>,
                {
                    liquid_core::env::emit(event.into())
                }
            }
        }
        pub use __liquid_event::{Event, Emit};
        #[cfg(test)]
        mod __liquid_testable {
            use super::*;
            pub struct TestableStorage {
                contract: Storage,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for TestableStorage {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        TestableStorage {
                            contract: ref __self_0_0,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("TestableStorage");
                            let _ = debug_trait_builder.field("contract", &&(*__self_0_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl From<Storage> for TestableStorage {
                fn from(contract: Storage) -> Self {
                    Self { contract }
                }
            }
            impl core::ops::Deref for TestableStorage {
                type Target = Storage;
                fn deref(&self) -> &Self::Target {
                    &self.contract
                }
            }
            impl core::ops::DerefMut for TestableStorage {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.contract
                }
            }
            impl TestableStorage {
                pub fn new() -> Self {
                    let mut contract = <Storage as liquid_core::storage::New>::new();
                    contract.new();
                    Self { contract }
                }
            }
        }
        #[cfg(test)]
        pub use __liquid_testable::TestableStorage;
    }
    #[cfg(test)]
    #[allow(non_snake_case)]
    pub type KvTableTest = __liquid_private::TestableStorage;
    pub struct SetResult {
        pub count: i256,
    }
    use super::kv_table_factory::*;
    use liquid_core::storage;
}
#[allow(dead_code)]
fn main() {}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
