use axaddrspace::MappingFlags;

use crate::{AxVMConfig, GuestPhysAddr, HostPhysAddr, data::VmData};

mod addrspace;
pub(crate) mod data;
mod define;
mod machine;

pub(crate) use addrspace::*;
pub use define::*;
pub use machine::*;

pub struct Vm {
    data: VmData,
}

impl Vm {
    pub fn new(config: AxVMConfig) -> anyhow::Result<Self> {
        let data = VmData::new(config)?;
        data.init()?;
        Ok(Self { data })
    }

    pub fn id(&self) -> VmId {
        self.data.id()
    }

    pub fn name(&self) -> &str {
        self.data.name()
    }

    pub fn boot(&self) -> anyhow::Result<()> {
        self.data.start()
    }

    pub fn shutdown(&self) -> anyhow::Result<()> {
        self.data.stop()
    }

    #[inline]
    pub fn status(&self) -> VMStatus {
        self.data.status()
    }

    pub fn wait(&self) -> anyhow::Result<()> {
        self.data.wait()
    }

    /// Get total memory size in bytes.
    pub fn memory_size(&self) -> usize {
        self.data.memory_size
    }

    /// Get vCPU count.
    pub fn vcpu_num(&self) -> usize {
        self.data.vcpu_num
    }

    /// Translate a guest physical address into host physical address.
    pub fn gpa_to_hpa(&self, gpa: GuestPhysAddr) -> anyhow::Result<HostPhysAddr> {
        self.data
            .with_machine_running(|running| running.gpa_to_hpa(gpa))
            .map_err(|e| anyhow!("{e}"))?
    }

    /// Read a little-endian u64 from guest physical memory.
    pub fn read_guest_u64(&self, gpa: GuestPhysAddr) -> anyhow::Result<u64> {
        self.data
            .with_machine_running(|running| running.read_guest_u64(gpa))
            .map_err(|e| anyhow!("{e}"))?
    }

    pub fn query_gpa_mapping(
        &self,
        gpa: GuestPhysAddr,
    ) -> anyhow::Result<(HostPhysAddr, MappingFlags, usize)> {
        self.data
            .with_machine_running(|running| running.query_gpa_mapping(gpa))
            .map_err(|e| anyhow!("{e}"))?
    }

    /// Update Stage-2 execute permission for one guest physical page.
    pub fn set_gpa_executable(&self, gpa: GuestPhysAddr, executable: bool) -> anyhow::Result<()> {
        self.data
            .with_machine_running_mut(|running| running.set_gpa_executable(gpa, executable))
            .map_err(|e| anyhow!("{e}"))?
    }

    /// Return last observed guest TTBR1_EL1 saved on VM exits.
    pub fn guest_ttbr1_el1(&self) -> u64 {
        self.data.last_ttbr1_el1()
    }
}
