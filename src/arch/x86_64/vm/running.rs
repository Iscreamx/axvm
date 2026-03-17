use core::ops::Deref;

use axaddrspace::MappingFlags;

use crate::{
    GuestPhysAddr, HostPhysAddr, VmMachineRunningCommon, VmMachineRunningOps, VmMachineStoppingOps,
    arch::cpu::VCpu, vhal::cpu::CpuHardId,
};

pub struct VmMachineRunning {
    pub common: VmMachineRunningCommon,
}

impl VmMachineRunning {
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

        // x86 使用 SIPI (Startup IPI) 来启动 AP
        // 这里设置 entry point 和参数
        cpu.vcpu.set_entry(entry_point.as_usize().into())?;
        cpu.vcpu.set_gpr(0, arg as _);
        self.common.run_cpu(cpu)?;
        Ok(())
    }
}

impl Deref for VmMachineRunning {
    type Target = VmMachineRunningCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl VmMachineRunningOps for VmMachineRunning {
    type Stopping = super::stopping::VmStatusStopping;

    fn stop(self) -> Self::Stopping {
        debug!("Stopping x86_64 VM");
        super::stopping::VmStatusStopping {}
    }
}
