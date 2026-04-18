#![no_std]
#![feature(new_range_api)]

//! This crate provides a minimal VM monitor (VMM) for running guest VMs.
//!
//! This crate contains:
//! - [`AxVM`]: The main structure representing a VM.

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

extern crate axstd as std;

const TASK_STACK_SIZE: usize = 0x40000; // 256 KB

#[cfg_attr(target_arch = "aarch64", path = "arch/aarch64/mod.rs")]
#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/mod.rs")]
pub(crate) mod arch;

mod fdt;
mod vcpu;
mod vm;
mod vmexit;

pub mod config;
pub mod vhal;

#[cfg(all(target_arch = "aarch64", feature = "hprobe"))]
pub use arch::install_trap_vector;
pub use axvm_types::addr::*;
pub use config::AxVMConfig;
pub use vhal::cpu::CpuId;
pub use vm::*;
pub use vmexit::register_vmexit_handler;

/// Enable hardware virtualization support.
pub fn enable_viretualization() -> anyhow::Result<()> {
    vhal::init()?;
    Ok(())
}
