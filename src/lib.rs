#![allow(dead_code)]
#![allow(unused_variables)]

pub mod bus;
pub mod core;

pub mod condition;
pub mod register;
pub mod instruction;
pub mod operation;

pub mod decoder;
pub mod executor;

extern crate byteorder;
extern crate bit_field;
extern crate enum_set;

use core::Core;
use bus::Bus;

#[cfg(test)]
use bus::ram::RAM;
#[cfg(test)]
use bus::flash::FlashMemory;

pub fn run_bin<T: Bus, R: Bus>(code: &mut T, sram: &mut R) {

    let mut internal_bus = bus::internal::InternalBus::new();
    let mut ahb = bus::ahblite::AHBLite::new(code, sram);

    let mut bus = bus::busmatrix::BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    core.reset();
    loop {
        core.run();
    }
}

#[test]
fn test_hello_world() {
    let mut hellow_bin: [u8; 1204] =
        [0x08, 0x04, 0x00, 0x20, 0xa1, 0x04, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00,
         0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83,
         0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00,
         0x83, 0x02, 0x00, 0x00, 0x70, 0xb5, 0x06, 0x00, 0x08, 0x00, 0x14, 0x00, 0x00, 0x2b, 0x05,
         0xd0, 0x1d, 0x00, 0x21, 0x78, 0xb0, 0x47, 0x64, 0x1c, 0x6d, 0x1e, 0xfa, 0xd1, 0x70, 0xbd,
         0xfd, 0xb5, 0x86, 0xb0, 0x0c, 0x00, 0x09, 0xe0, 0x01, 0x23, 0x07, 0x9a, 0x21, 0x00, 0x06,
         0x98, 0xff, 0xf7, 0xe9, 0xff, 0x04, 0x00, 0x07, 0x98, 0x40, 0x1c, 0x07, 0x90, 0x07, 0x98,
         0x00, 0x78, 0x00, 0x28, 0x01, 0xd1, 0x09, 0xb0, 0xf0, 0xbd, 0x25, 0x28, 0xed, 0xd1, 0x07,
         0x98, 0x46, 0x78, 0x80, 0x1c, 0x07, 0x90, 0x25, 0x2e, 0x00, 0xd1, 0x95, 0xe0, 0x58, 0x2e,
         0x69, 0xd0, 0x63, 0x2e, 0x0e, 0xd0, 0x64, 0x2e, 0x1a, 0xd0, 0x69, 0x2e, 0x18, 0xd0, 0x6f,
         0x2e, 0x61, 0xd0, 0x70, 0x2e, 0x68, 0xd0, 0x73, 0x2e, 0x70, 0xd0, 0x75, 0x2e, 0x5b, 0xd0,
         0x78, 0x2e, 0x59, 0xd0, 0xdd, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60, 0x01,
         0x68, 0x00, 0x1d, 0x08, 0x9a, 0x10, 0x60, 0xc9, 0xb2, 0x20, 0x00, 0x06, 0x9a, 0x90, 0x47,
         0x04, 0x00, 0xcf, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60, 0x05, 0x68, 0x00,
         0x1d, 0x08, 0x99, 0x08, 0x60, 0x00, 0x2d, 0x04, 0xd5, 0x2d, 0x21, 0x20, 0x00, 0x06, 0x9a,
         0x90, 0x47, 0x04, 0x00, 0x64, 0x26, 0x6f, 0x2e, 0x01, 0xd1, 0x08, 0x21, 0x06, 0xe0, 0x20,
         0x21, 0x31, 0x43, 0x78, 0x29, 0x01, 0xd0, 0x0a, 0x21, 0x00, 0xe0, 0x10, 0x21, 0x01, 0x91,
         0x0b, 0x21, 0x00, 0x91, 0x2f, 0x00, 0x64, 0x2e, 0x03, 0xd1, 0x00, 0x2d, 0x01, 0xd5, 0x78,
         0x42, 0x07, 0x00, 0x00, 0x9d, 0x6d, 0x1e, 0x00, 0x95, 0x38, 0x00, 0x01, 0x99, 0x00, 0xf0,
         0x4b, 0xf8, 0x30, 0x31, 0xc9, 0xb2, 0x3a, 0x29, 0x02, 0xdb, 0x30, 0x00, 0x51, 0x38, 0x09,
         0x18, 0x02, 0xa8, 0x45, 0x19, 0x29, 0x70, 0x38, 0x00, 0x01, 0x99, 0x00, 0xf0, 0x3d, 0xf8,
         0x07, 0x00, 0x02, 0xd0, 0x00, 0x98, 0x01, 0x28, 0xe5, 0xda, 0x20, 0x00, 0x0b, 0x21, 0x00,
         0x9a, 0x8c, 0x1a, 0xb9, 0xd0, 0x29, 0x78, 0x06, 0x9a, 0x90, 0x47, 0x6d, 0x1c, 0x64, 0x1e,
         0xf9, 0xd1, 0xb2, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60, 0x05, 0x68, 0x00,
         0x1d, 0x08, 0x99, 0x08, 0x60, 0xbb, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60,
         0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60, 0x78, 0x26, 0xb1, 0xe7, 0x08, 0x98, 0x00,
         0x68, 0x08, 0x99, 0x08, 0x60, 0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60, 0x28, 0x00,
         0x00, 0xf0, 0x65, 0xf8, 0x01, 0x00, 0x20, 0x00, 0x00, 0x29, 0x90, 0xd0, 0x0c, 0x00, 0x29,
         0x78, 0x06, 0x9a, 0x90, 0x47, 0x6d, 0x1c, 0x64, 0x1e, 0xf9, 0xd1, 0x88, 0xe7, 0x25, 0x21,
         0x83, 0xe7, 0x00, 0x22, 0x03, 0x0a, 0x8b, 0x42, 0x0b, 0xd2, 0x03, 0x09, 0x8b, 0x42, 0x19,
         0xd2, 0x43, 0x08, 0x8b, 0x42, 0x2e, 0xd2, 0x41, 0x1a, 0x00, 0xd2, 0x01, 0x46, 0x52, 0x41,
         0x10, 0x46, 0x70, 0x47, 0xff, 0x22, 0x09, 0x02, 0x3f, 0xd0, 0x12, 0x06, 0x8b, 0x42, 0x05,
         0xd3, 0x12, 0x12, 0x09, 0x02, 0x8b, 0x42, 0x01, 0xd3, 0x12, 0x12, 0x09, 0x02, 0x03, 0x09,
         0x8b, 0x42, 0x19, 0xd3, 0x00, 0xe0, 0x09, 0x0a, 0xc3, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0xcb,
         0x01, 0xc0, 0x1a, 0x52, 0x41, 0x83, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0x8b, 0x01, 0xc0, 0x1a,
         0x52, 0x41, 0x43, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0x4b, 0x01, 0xc0, 0x1a, 0x52, 0x41, 0x03,
         0x09, 0x8b, 0x42, 0x01, 0xd3, 0x0b, 0x01, 0xc0, 0x1a, 0x52, 0x41, 0xc3, 0x08, 0x8b, 0x42,
         0x01, 0xd3, 0xcb, 0x00, 0xc0, 0x1a, 0x52, 0x41, 0x83, 0x08, 0x8b, 0x42, 0x01, 0xd3, 0x8b,
         0x00, 0xc0, 0x1a, 0x52, 0x41, 0x43, 0x08, 0x8b, 0x42, 0x01, 0xd3, 0x4b, 0x00, 0xc0, 0x1a,
         0x52, 0x41, 0x88, 0x42, 0x00, 0xd3, 0x40, 0x1a, 0x52, 0x41, 0xcf, 0xd2, 0x01, 0x46, 0x10,
         0x46, 0x70, 0x47, 0x08, 0xb5, 0x00, 0xf0, 0x0a, 0xf8, 0x08, 0xbd, 0x01, 0x00, 0x00, 0xe0,
         0x49, 0x1c, 0x0a, 0x78, 0x00, 0x2a, 0xfb, 0xd1, 0x08, 0x1a, 0x70, 0x47, 0xfe, 0xe7, 0x70,
         0x47, 0x00, 0x00, 0x80, 0xb5, 0x00, 0xf0, 0x33, 0xf8, 0x02, 0x00, 0x00, 0x23, 0xdb, 0x43,
         0x10, 0x68, 0x98, 0x42, 0x04, 0xd0, 0x11, 0x00, 0x02, 0x20, 0xab, 0xbe, 0x00, 0x20, 0x10,
         0x60, 0x50, 0x68, 0x98, 0x42, 0x04, 0xd0, 0x11, 0x1d, 0x02, 0x20, 0xab, 0xbe, 0x00, 0x20,
         0x50, 0x60, 0x01, 0xbd, 0x00, 0x00, 0x10, 0xb5, 0x84, 0xb0, 0x04, 0x00, 0x00, 0xf0, 0x19,
         0xf8, 0xa1, 0x00, 0x42, 0x18, 0x10, 0x68, 0x00, 0x21, 0xc9, 0x43, 0x88, 0x42, 0x0d, 0xd1,
         0x07, 0xa0, 0x00, 0x90, 0x00, 0x2c, 0x01, 0xd1, 0x00, 0x20, 0x00, 0xe0, 0x04, 0x20, 0x01,
         0x90, 0x03, 0x20, 0x02, 0x90, 0x69, 0x46, 0x01, 0x20, 0xab, 0xbe, 0x10, 0x60, 0x04, 0xb0,
         0x10, 0xbd, 0x3a, 0x74, 0x74, 0x00, 0x00, 0x48, 0x70, 0x47, 0x00, 0x00, 0x00, 0x20, 0x30,
         0xb4, 0x01, 0x21, 0x02, 0x68, 0x00, 0x1d, 0x00, 0x2a, 0x0f, 0xd0, 0x03, 0x68, 0xc3, 0x18,
         0x44, 0x68, 0x08, 0x30, 0x0c, 0x42, 0x02, 0xd0, 0x4d, 0x46, 0x6d, 0x1e, 0x64, 0x19, 0x1d,
         0x68, 0x25, 0x60, 0x1b, 0x1d, 0x24, 0x1d, 0x12, 0x1f, 0xec, 0xd0, 0xf8, 0xe7, 0x30, 0xbc,
         0x70, 0x47, 0x10, 0xb5, 0x07, 0x49, 0x79, 0x44, 0x18, 0x31, 0x06, 0x4c, 0x7c, 0x44, 0x16,
         0x34, 0x04, 0xe0, 0x08, 0x1d, 0x0a, 0x68, 0x89, 0x18, 0x88, 0x47, 0x01, 0x00, 0xa1, 0x42,
         0xf8, 0xd1, 0x10, 0xbd, 0xc0, 0x00, 0x00, 0x00, 0xd0, 0x00, 0x00, 0x00, 0x0e, 0xb4, 0x00,
         0xb5, 0x82, 0xb0, 0x03, 0xa9, 0x00, 0x91, 0x6b, 0x46, 0x02, 0x00, 0x01, 0x21, 0x03, 0x48,
         0x78, 0x44, 0x0a, 0x30, 0xff, 0xf7, 0x76, 0xfe, 0x02, 0x99, 0x06, 0xb0, 0x08, 0x47, 0x05,
         0x01, 0x00, 0x00, 0x38, 0xb5, 0x04, 0x00, 0x00, 0x25, 0xed, 0x43, 0xac, 0x42, 0x09, 0xd0,
         0x69, 0x46, 0x08, 0x70, 0x01, 0x22, 0x01, 0x20, 0x00, 0xf0, 0x06, 0xf8, 0x01, 0x28, 0x01,
         0xd1, 0x20, 0x00, 0x32, 0xbd, 0x28, 0x00, 0x32, 0xbd, 0x80, 0xb5, 0x00, 0x28, 0x02, 0xd4,
         0x00, 0xf0, 0x03, 0xf8, 0x02, 0xbd, 0x00, 0x20, 0x02, 0xbd, 0x80, 0xb5, 0x00, 0x29, 0x01,
         0xd1, 0x00, 0x20, 0x02, 0xbd, 0x01, 0x28, 0x01, 0xd0, 0x02, 0x28, 0x02, 0xd1, 0x00, 0xf0,
         0x04, 0xf8, 0x02, 0xbd, 0x00, 0xf0, 0x11, 0xf8, 0x02, 0xbd, 0x30, 0xb5, 0x83, 0xb0, 0x0c,
         0x00, 0x15, 0x00, 0x01, 0x20, 0xff, 0xf7, 0x70, 0xff, 0x00, 0x90, 0x01, 0x94, 0x02, 0x95,
         0x69, 0x46, 0x05, 0x20, 0xab, 0xbe, 0x28, 0x1a, 0x03, 0xb0, 0x30, 0xbd, 0x10, 0xb5, 0x84,
         0xb0, 0x03, 0x00, 0x14, 0x00, 0x00, 0x93, 0x01, 0x91, 0x02, 0x94, 0x69, 0x46, 0x05, 0x20,
         0xab, 0xbe, 0x01, 0x22, 0x19, 0x00, 0x20, 0x1a, 0xc0, 0x46, 0xc0, 0x46, 0x04, 0xb0, 0x10,
         0xbd, 0xf1, 0xfe, 0xff, 0xff, 0x08, 0x00, 0x00, 0x00, 0x98, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x0b, 0xf8, 0x00, 0x28, 0x01, 0xd0, 0xff,
         0xf7, 0x80, 0xff, 0x00, 0x20, 0xc0, 0x46, 0xc0, 0x46, 0x00, 0xf0, 0x05, 0xf8, 0x00, 0xf0,
         0x0b, 0xf8, 0x01, 0x20, 0x70, 0x47, 0x00, 0x00, 0x80, 0xb5, 0x02, 0x48, 0xff, 0xf7, 0x86,
         0xff, 0x00, 0x20, 0x02, 0xbd, 0x90, 0x04, 0x00, 0x00, 0x80, 0xb5, 0x00, 0xf0, 0x01, 0xf8,
         0x01, 0xbd, 0x07, 0x46, 0x38, 0x46, 0x00, 0xf0, 0x02, 0xf8, 0xfb, 0xe7, 0x00, 0x00, 0x80,
         0xb5, 0xff, 0xf7, 0x0f, 0xff, 0x02, 0x4a, 0x11, 0x00, 0x18, 0x20, 0xab, 0xbe, 0xfb, 0xe7,
         0x26, 0x00, 0x02, 0x00, 0x38, 0xb5, 0x05, 0x00, 0x0c, 0x00, 0x20, 0x00, 0xff, 0xf7, 0x7a,
         0xff, 0xa0, 0x42, 0x00, 0xd0, 0x00, 0x25, 0x28, 0x00, 0x32, 0xbd, 0x00, 0x00, 0x68, 0x65,
         0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x0a, 0x00, 0x00, 0x00, 0xc0,
         0x46, 0xc0, 0x46, 0xc0, 0x46, 0xc0, 0x46, 0xff, 0xf7, 0xba, 0xff, 0xff, 0xff, 0xff, 0xff,
         0xff, 0xff, 0xff, 0xff];

    let mut ram_mem = vec![0; 1024];

    let mut hellow = FlashMemory::new(&mut hellow_bin, 0x0);
    let mut ram = RAM::new(&mut ram_mem, 0x20000000);
    run_bin(&mut hellow, &mut ram);
}
