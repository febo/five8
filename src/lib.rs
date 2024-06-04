#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(target_feature = "avx2")]
mod avx;

#[cfg(target_feature = "avx2")]
mod bits_find_lsb;

mod consts;
mod decode;
mod encode;
mod error;
pub use decode::{decode_32, decode_64};
pub use encode::{encode_32, encode_64};
pub use error::DecodeError;
#[cfg(feature = "dev-utils")]
pub use {
    decode::{truncate_and_swap_u64s_64_pub, truncate_and_swap_u64s_scalar_pub},
    encode::{
        in_leading_0s_scalar_pub, intermediate_to_base58_scalar_64_pub, make_binary_array_64_pub,
        make_intermediate_array_64_pub,
    },
};
mod unlikely;
