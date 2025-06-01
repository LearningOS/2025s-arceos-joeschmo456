use alloc::sync::Arc;

use crate::vcpu::VmCpuRegisters;
use axmm::AddrSpace;
use axsync::Mutex;

/// Task extended data for the monolithic kernel.
pub struct TaskExt {
    /// The vcpu.
    pub vcpu: VmCpuRegisters,
    /// The virtual memory address space.
    pub aspace: Arc<Mutex<AddrSpace>>,
}

impl TaskExt {
    pub const fn new(vcpu: VmCpuRegisters, aspace: Arc<Mutex<AddrSpace>>) -> Self {
        Self { vcpu, aspace }
    }
}

axtask::def_task_ext!(TaskExt);
