//! This crate provides a way to access Device electronic signature
//! items on STM32 microcontrollers.
//!
//! You need to pass one of the features in order to use this crate:
//! * `stm32f0`
//! * `stm32f1`
//! * `stm32f2`
//! * `stm32f3`
//! * `stm32f4`
//! * `stm32f72x`
//! * `stm32f73x`
//! * `stm32f76x`
//! * `stm32f77x`
//! * `stm32g0`
//! * `stm32h72x`
//! * `stm32h73x`
//! * `stm32h74x`
//! * `stm32h75x`
//! * `stm32h7ax`
//! * `stm32h7bx`
//! * `stm32l0`
//! * `stm32l4`
//! * `stm32wb5x`

#![no_std]

use cortex_m::interrupt;

#[cfg(any(feature = "stm32f0", feature = "stm32f3"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_F7AC as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_F7CC as _;
}

#[cfg(feature = "stm32f1")]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_F7E8 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_F7E0 as _;
}

#[cfg(any(feature = "stm32f2", feature = "stm32f4"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_7A10 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_7A22 as _;
}

#[cfg(any(feature = "stm32f72x", feature = "stm32f73x"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FF0_7A10 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FF0_7A22 as _;
}

#[cfg(any(feature = "stm32f76x", feature = "stm32f77x"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FF0_F420 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FF0_F442 as _;
}

#[cfg(any(feature = "stm32g0", feature = "stm32l4", feature = "stm32wb5x"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_7590 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_75E0 as _;
}

#[cfg(any(
    feature = "stm32h72x",
    feature = "stm32h73x",
    feature = "stm32h74x",
    feature = "stm32h75x"
))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FF1_E800 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FF1_E880 as _;
}

#[cfg(any(feature = "stm32h7ax", feature = "stm32h7bx"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x08FF_F800 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x08FF_F80C as _;
}

#[cfg(feature = "stm32l0")]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FF8_0050 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FF8_007C as _;
}

use pointers::*;


/// Returns a 12-byte unique device ID
pub fn device_id() -> &'static [u8; 12] {
    unsafe { &*DEVICE_ID_PTR.cast::<[u8; 12]>() }
}


/// Returns a string with a hex-encoded unique device ID
pub fn device_id_hex() -> &'static str {
    static mut DEVICE_ID_STR: [u8; 24] = [0; 24];

    unsafe {
        if DEVICE_ID_STR.as_ptr().read_volatile() == 0 {
            interrupt::free(|| {
                let hex = b"0123456789abcdef";
                for (i, b) in device_id().iter().enumerate() {
                    let lo = b & 0xf;
                    let hi = (b >> 4) & 0xfu8;
                    DEVICE_ID_STR[i*2] = hex[hi as usize];
                    DEVICE_ID_STR[i*2+1] = hex[lo as usize];
                }
            });
        }

        core::str::from_utf8_unchecked(&DEVICE_ID_STR)
    }
}


/// Returns the Flash memory size of the device in Kbytes
pub fn flash_size_kb() -> u16 {
    unsafe { *FLASH_SIZE_PTR }
}
