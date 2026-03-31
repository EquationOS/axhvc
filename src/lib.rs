#![no_std]

use bit_field::BitField;
use numeric_enum_macro::numeric_enum;

use axerrno::AxResult;

const HYPER_CALL_CODE_PRIVILEGED_MASK: u32 = 0xe000_0000;

numeric_enum! {
    #[repr(u32)]
    #[derive(Eq, PartialEq, Copy, Clone)]
    pub enum HyperCallCode {
        /// Disable the hypervisor.
        HypervisorDisable = 0,
        /// Prepare to disable the hypervisor, map the hypervisor memory to the guest.
        HyperVisorPrepareDisable = 1,
        /// Only for debugging purposes.
        HyperVisorDebug = 2,
        /// Only for debugging purposes.
        HDebug = HYPER_CALL_CODE_PRIVILEGED_MASK | 0,
        /// Init ring 0 shim.
        HInitShim = HYPER_CALL_CODE_PRIVILEGED_MASK | 1,
        /// Create a new instance, pass the raw binary/executable file by shared pages.
        HCreateInstance = HYPER_CALL_CODE_PRIVILEGED_MASK |2,
        /// Setup a instance, this is called by the instance when it is created and loaded.
        HSetupInstance = HYPER_CALL_CODE_PRIVILEGED_MASK | 4,
        /// Exit from a insance process.
        HExitProcess = HYPER_CALL_CODE_PRIVILEGED_MASK | 5,
        /// Exit from a instance, this is called by the instance when the last process in the instance exits.
        HShutdownInstance = HYPER_CALL_CODE_PRIVILEGED_MASK | 6,
        /// Allocate a memory region for the instance.
        /// This is called by the instance when it needs to extends its memory region.
        HAllocMMRegion = HYPER_CALL_CODE_PRIVILEGED_MASK | 7,
        /// Refer to `shmget` syscall <https://man7.org/linux/man-pages/man2/shmget.2.html>
        /// this is used to get a shared memory region.
        /// It may be used either to obtain the identifier of a previously created
        /// shared memory segment (when shmflg is zero and key does not have
        /// the value IPC_PRIVATE), or to create a new set.
        /// It will return the base address and size of the shared memory region.
        HIVCGet = HYPER_CALL_CODE_PRIVILEGED_MASK | 8,
        /// Refer to `shmdt` syscall, <https://man7.org/linux/man-pages/man3/shmdt.3p.html>
        /// this is used to unsubscribe from a shared memory region.
        HIVCDt = HYPER_CALL_CODE_PRIVILEGED_MASK | 9,
        /// Refer to `shmat` syscall, <https://man7.org/linux/man-pages/man2/shmat.2.html>
        /// this is used to attach a shared memory region to the current instance.
        HIVCSHMAt = HYPER_CALL_CODE_PRIVILEGED_MASK | 10,
        /// Clear all guest memory areas,
        /// I know this HVC seems strange, the thing is, we prepare a early-stage `eqloader`
        /// in guest address space as a user-space executor, when the instance starts running,
        /// the pre-loader `eqloader` is useless and should be cleared, because it will take up
        /// the guest memory space and the mapping is not known by the `EqAddrSpace` in `ProcessInnerRegion`.
        /// So we use this hypercall to notify the hypervisor that the loading is done and
        /// the pre-loader should be cleared.
        /// Generally, this HVC is triggered by `shim` in the first `brk` syscall.
        HClearGuestAreas = HYPER_CALL_CODE_PRIVILEGED_MASK | 11,

        /// Notify the hypervisor to boot a microVM instance.
        HMicroVMBoot = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x20,
        /// Inject an IRQ event into a running microVM from host side.
        /// arg0: instance_id, arg1: msix_index.
        HMicroVMInjectIrq = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x21,

        /// Only for debugging purposes, console read.
        HRead = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x11,
        /// Only for debugging purposes, console write.
        HWrite = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x12,
        /// Duplicate current gaddrspace to a new one, return its EPTP list index.
        HDupGas = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x13,
        /// Just a benchmark hypercall for VM call overhead measurement.
        HBenchVMCall = HYPER_CALL_CODE_PRIVILEGED_MASK | 0xff,
        /// Just a benchmark hypercall for EPT mmap overhead measurement.
        HBenchEPTMmap = HYPER_CALL_CODE_PRIVILEGED_MASK | 0xfe,
        /// Just a benchmark hypercall for EPT munmap overhead measurement.
        HBenchEPTMUnmap = HYPER_CALL_CODE_PRIVILEGED_MASK | 0xfd,
    }
}

impl core::fmt::Debug for HyperCallCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;
        match self {
            HyperCallCode::HypervisorDisable => write!(f, "HypervisorDisable {:#x}", *self as u32),
            HyperCallCode::HyperVisorPrepareDisable => {
                write!(f, "HyperVisorPrepareDisable {:#x}", *self as u32)
            }
            HyperCallCode::HyperVisorDebug => write!(f, "HyperVisorDebug {:#x}", *self as u32),
            HyperCallCode::HDebug => write!(f, "HDebug {:#x}", *self as u32),
            HyperCallCode::HRead => write!(f, "HRead {:#x}", *self as u32),
            HyperCallCode::HWrite => write!(f, "HWrite {:#x}", *self as u32),
            HyperCallCode::HCreateInstance => write!(f, "HCreateInstance {:#x}", *self as u32),
            HyperCallCode::HExitProcess => write!(f, "HExitProcess {:#x}", *self as u32),
            HyperCallCode::HShutdownInstance => write!(f, "HShutdownInstance {:#x}", *self as u32),
            HyperCallCode::HDupGas => write!(f, "HDupGas {:#x}", *self as u32),
            HyperCallCode::HInitShim => write!(f, "HInitShim {:#x}", *self as u32),
            HyperCallCode::HSetupInstance => write!(f, "HSetupInstance {:#x}", *self as u32),
            HyperCallCode::HAllocMMRegion => write!(f, "HAllocMMRegion {:#x}", *self as u32),
            HyperCallCode::HIVCGet => write!(f, "HIVCGet {:#x}", *self as u32),
            HyperCallCode::HIVCDt => write!(f, "HIVCDt {:#x}", *self as u32),
            HyperCallCode::HIVCSHMAt => write!(f, "HIVCSHMAt {:#x}", *self as u32),
            HyperCallCode::HClearGuestAreas => write!(f, "HClearGuestAreas {:#x}", *self as u32),
            HyperCallCode::HBenchVMCall => write!(f, "HBenchVMCall {:#x}", *self as u32),
            HyperCallCode::HBenchEPTMmap => write!(f, "HBenchEPTMmap {:#x}", *self as u32),
            HyperCallCode::HBenchEPTMUnmap => write!(f, "HBenchEPTMUnmap {:#x}", *self as u32),
            HyperCallCode::HMicroVMBoot => write!(f, "HMicroVMBoot {:#x}", *self as u32),
            HyperCallCode::HMicroVMInjectIrq => {
                write!(f, "HMicroVMInjectIrq {:#x}", *self as u32)
            }
        }?;
        write!(f, ")")
    }
}

impl HyperCallCode {
    pub fn is_privileged(self) -> bool {
        (self as u32).get_bits(29..32) == 0
    }
}

pub type HyperCallResult = AxResult<usize>;
