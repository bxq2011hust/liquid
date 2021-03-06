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

use derive_more::From;

#[derive(From)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum EnvError {
    ScaleDecode(scale::Error),
    AbiDecode(liquid_primitives::Error),
    NotEnoughSpace,
    UnableToReadFromStorage,
    UnableToReadCallData,
    FailToCallForeignContract,
}

/// A result of environmental operations
pub type Result<T> = core::result::Result<T, EnvError>;
