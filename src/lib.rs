//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//
mod collections;
pub use crate::collections::*;
mod config;
pub use crate::config::*;
mod commons;
pub use crate::commons::*;
mod keyexpr;
pub use crate::keyexpr::*;
mod info;
pub use crate::info::*;
mod get;
pub use crate::get::*;
mod queryable;
pub use crate::queryable::*;
mod put;
pub use crate::put::*;
mod scouting;
pub use crate::scouting::*;
mod session;
pub use crate::session::*;
mod subscriber;
pub use crate::subscriber::*;
mod pull_subscriber;
pub use crate::pull_subscriber::*;
mod publisher;
pub use crate::publisher::*;
mod closures;
pub use closures::*;

// #[cfg(target_arch="aarch64")]
// mod platform {
//     mod aarch64;
//     mod default;
//     pub use default::z_owned_keyexpr_t;
//     pub use default::z_owned_session_t;
//     pub use default::z_owned_queryable_t;
//     pub use default::z_owned_publisher_t;
//     pub use aarch64::z_owned_reply_t;
// }

// #[cfg(target_arch="x86_64")]
mod platform {
    mod aarch64;
    mod x86_64;
    mod default;
    pub use default::z_owned_keyexpr_t;
    pub use default::z_owned_session_t;
    pub use default::z_owned_queryable_t;
    pub use default::z_owned_publisher_t;
#[cfg(target_arch="x86_64")]
    pub use x86_64::z_owned_reply_t;
#[cfg(target_arch="aarch64")]
    pub use x86_64::z_owned_reply_t;
}

trait GuardedTransmute<D> {
    fn transmute(self) -> D;
}

#[macro_export]
macro_rules! define_guarded_transmute {
    ($src_type:ty, $dst_type:ty) => {
        const _ : () = assert!(std::mem::align_of::<$src_type>()==std::mem::align_of::<$dst_type>());
        impl crate::GuardedTransmute<$dst_type> for $src_type {
            fn transmute(self) -> $dst_type {
                unsafe { std::mem::transmute::<$src_type,$dst_type>(self)}
            }
        }
    };
}

pub(crate) const LOG_INVALID_SESSION: &str = "Invalid session";

/// Initialises the zenoh runtime logger.
///
/// Note that unless you built zenoh-c with the `logger-autoinit` feature disabled,
/// this will be performed automatically by `z_open` and `z_scout`.
#[no_mangle]
pub extern "C" fn zc_init_logger() {
    let _ = env_logger::try_init();
}

fn copy_to_libc(s: &[u8]) -> *mut libc::c_char {
    unsafe {
        let string = libc::malloc(s.len() + 1) as *mut libc::c_char;
        std::ptr::copy_nonoverlapping(s.as_ptr(), string as _, s.len());
        *string.add(s.len()) = 0;
        string
    }
}
