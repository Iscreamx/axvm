use fdt_edit::NodeRef;

use axaddrspace::MappingFlags;

use crate::{
    GuestPhysAddr, HostPhysAddr, VmAddrSpace, VmMachineRunningCommon, VmMachineRunningOps,
    VmMachineStoppingOps, arch::vm::DevMapConfig, vhal::cpu::CpuHardId,
};

/// Data needed when VM is running
pub struct VmMachineRunning {
    pub(super) common: VmMachineRunningCommon,
}

impl VmMachineRunning {
    fn handle_node_regs(dev_vec: &mut [DevMapConfig], node: &NodeRef<'_>) {}

    pub fn gpa_to_hpa(&self, gpa: GuestPhysAddr) -> anyhow::Result<HostPhysAddr> {
        self.common
            .vmspace
            .gpa_to_hpa(gpa)
            .ok_or_else(|| anyhow!("GPA {:#x} is not mapped", gpa.as_usize()))
    }

    pub fn read_guest_u64(&self, gpa: GuestPhysAddr) -> anyhow::Result<u64> {
        self.common.vmspace.read_guest_u64(gpa)
    }

    pub fn query_gpa_mapping(
        &self,
        gpa: GuestPhysAddr,
    ) -> anyhow::Result<(HostPhysAddr, MappingFlags, usize)> {
        self.common.vmspace.query_gpa_mapping(gpa)
    }

    pub fn set_gpa_executable(
        &mut self,
        gpa: GuestPhysAddr,
        executable: bool,
    ) -> anyhow::Result<()> {
        self.common.vmspace.set_gpa_executable(gpa, executable)
    }

    pub fn cpu_up(
        &mut self,
        target_cpu: CpuHardId,
        entry_point: GuestPhysAddr,
        arg: u64,
    ) -> anyhow::Result<()> {
        let mut cpu = self
            .common
            .cpus
            .remove(&target_cpu)
            .ok_or(anyhow!("No cpu {target_cpu} found"))?;

        cpu.vcpu.set_entry(entry_point.as_usize().into()).unwrap();
        cpu.vcpu.set_gpr(0, arg as _);
        self.common.run_cpu(cpu)?;
        Ok(())
    }
}

impl VmMachineRunningOps for VmMachineRunning {
    type Stopping = VmStatusStopping;

    fn stop(self) -> Self::Stopping {
        Self::Stopping {
            _vmspace: self.common.vmspace,
        }
    }
}

pub struct VmStatusStopping {
    _vmspace: VmAddrSpace,
}

impl VmMachineStoppingOps for VmStatusStopping {}
