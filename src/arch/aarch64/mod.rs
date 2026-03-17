pub mod cpu;
mod hal;
mod vm;

pub use cpu::HCpu;
pub use hal::Hal;
pub use vm::*;

/// Install the exception vector table that supports hprobe BRK handling.
#[cfg(feature = "hprobe")]
pub fn install_trap_vector() {
    arm_vcpu::install_trap_vector();
}
