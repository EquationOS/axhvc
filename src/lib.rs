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
        /// Query the host-side posted-interrupt route for a MicroVM MSI-X entry.
        /// arg0: host physical address of EqMicroVmIrqRouteQuery.
        HMicroVMQueryIrqRoute = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x22,
        /// Start a secondary vCPU inside the currently running microVM.
        /// arg0: guest vCPU id, arg1: guest physical 64-bit entry point.
        HMicroVMStartVcpu = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x23,
        /// Send a PV IPI to a vCPU inside the currently running microVM.
        /// arg0: guest vCPU id, arg1: interrupt vector.
        HMicroVMSendIpi = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x24,
        /// Set the desired online vCPU count for a MicroVM from host side.
        /// arg0: instance_id, arg1: desired vCPU count.
        HMicroVMSetVcpuCount = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x25,
        /// Query vCPU limits from inside the currently running MicroVM.
        /// Returns: low 32 bits desired vCPU count, high 32 bits max vCPU count.
        HMicroVMGetVcpuCount = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x26,
        /// Notify EqVisor that eqgate consumed a host VFIO mailbox vector in non-root.
        /// arg0: notification vector, arg1: gate-side consume count.
        HMicroVMDrainVfioMailbox = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x27,
        /// Register an eqLinux LLFree/HyperAlloc zone metadata page range.
        /// arg0: guest physical address of EqHyperAllocRegisterZoneReq.
        HMicroVMHyperAllocRegisterZone = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x28,
        /// Install a previously reclaimed 2 MiB HyperAlloc frame before guest allocation returns it.
        /// arg0: guest physical address of EqHyperAllocInstallReq.
        HMicroVMHyperAllocInstall = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x29,
        /// Notify EqVisor that eqLinux finished a pagecache drop/shrink pass.
        /// arg0: dropped huge-frame count, arg1: remaining file huge-frame count.
        HMicroVMHyperAllocNotifyPagecacheDropped = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x2a,
        /// Query HyperAlloc capability/state for the current microVM.
        /// arg0: guest physical address of EqHyperAllocQuery.
        HMicroVMHyperAllocQuery = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x2b,
        /// Poll a pending HyperAlloc VFIO DMA map/unmap request from host-side axcli.
        /// arg0: host physical address of EqHyperAllocVfioDmaOp.
        HMicroVMHyperAllocVfioDmaPoll = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x2c,
        /// Complete a HyperAlloc VFIO DMA request after host-side axcli executed VFIO ioctl.
        /// arg0: host physical address of EqHyperAllocVfioDmaOp.
        HMicroVMHyperAllocVfioDmaComplete = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x2d,
        /// Debug-only logical hard reclaim/return smoke path.
        /// arg0: frame_gpa, arg1: zone_id.
        HMicroVMHyperAllocDebugReclaim = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x2e,
        /// Poll a pending EqVisor pagecache shrink request from eqLinux.
        /// arg0: guest physical address of EqHyperAllocPagecacheShrinkReq.
        HMicroVMHyperAllocPagecacheShrinkReq = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x2f,
        /// Copy microVM guest RAM through EqVisor on behalf of a host-side userspace backend.
        /// arg0: host physical address of EqMicroVmGuestMemCopy.
        HMicroVMGuestMemCopy = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x32,
        /// Update EqVisor's host-side microVM guest RAM mmap diagnostic state.
        /// arg0: host physical address of EqMicroVmGuestRamMmapStateUpdate.
        HMicroVMGuestRamMmapStateUpdate = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x33,
        /// Set a host-side HyperAlloc memory target request for a microVM.
        /// arg0: instance_id, arg1: host physical address of EqHyperAllocPagecacheShrinkReq.
        HMicroVMHyperAllocMemoryTarget = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x34,
        /// Query HyperAlloc state for a microVM from host side.
        /// arg0: instance_id, arg1: host physical address of EqHyperAllocQuery.
        HMicroVMHyperAllocHostQuery = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x35,
        /// Debug-only request injection for a HyperAlloc VFIO DMA map/unmap op.
        /// arg0: instance_id, arg1: host physical address of EqHyperAllocVfioDmaOp.
        HMicroVMHyperAllocVfioDmaDebugRequest = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x36,
        /// Host-side manual drain for EqGate HyperAlloc install batches.
        /// arg0: instance_id, arg1: host physical address of EqHyperAllocEqGateDrainReq.
        HMicroVMHyperAllocEqGateDrain = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x37,
        /// Host-side debug enqueue into EqGate HyperAlloc install batches.
        /// arg0: instance_id, arg1: host physical address of EqHyperAllocEqGateDebugEnqueueReq.
        HMicroVMHyperAllocEqGateDebugEnqueue = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x38,
        /// Guest-side enqueue into EqGate HyperAlloc install batches.
        /// arg0: guest physical address of EqHyperAllocEqGateEnqueueReq.
        HMicroVMHyperAllocEqGateEnqueue = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x39,
        /// Translate current microVM guest RAM GPA to HPA for host-side mmap faults.
        /// arg0: host physical address of EqMicroVmGuestRamTranslate.
        HMicroVMGuestRamTranslate = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x3a,
        /// Poll a pending host-side guest RAM VMA zap request.
        /// arg0: host physical address of EqMicroVmGuestRamMmapZapOp.
        HMicroVMGuestRamMmapZapPoll = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x3b,
        /// Complete a host-side guest RAM VMA zap request.
        /// arg0: host physical address of EqMicroVmGuestRamMmapZapOp.
        HMicroVMGuestRamMmapZapComplete = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x3c,
        /// Explicitly remove a stopped non-VFIO MicroVM from EqVisor host state.
        /// arg0: instance_id.
        HMicroVMRemove = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x3d,
        /// Request EqVisor to park a MicroVM back to the gate before host removal/cleanup.
        /// arg0: instance_id.
        HMicroVMStop = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x3e,
        /// Ask the current EqGate pCPU to exit so root can synchronize its
        /// Gate EPT alias generation before the next scheduler dequeue.
        /// arg0: requested Gate EPT alias generation.
        HMicroVMGateEptSync = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x3f,
        /// Batch logical HyperAlloc reclaim.  The guest supplies ranges that
        /// were aggregated from LLFree allocator metadata; EqVisor unmaps
        /// each contiguous range once and reconciles every huge frame.
        /// arg0: guest physical address of EqHyperAllocDebugReclaimBatchReq.
        HMicroVMHyperAllocDebugReclaimBatch = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x40,
        /// Debug-only deterministic two-vCPU race for one READY registry GPA.
        /// arg0: frame_gpa, arg1: barrier target (2=arm, 0=query/disarm).
        HMicroVMHyperAllocDebugRegistryRace = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x41,
        /// Synchronous host-side LLFree metadata scan/resize.
        /// arg0: guest physical address of EqHyperAllocResizeReq.
        HMicroVMHyperAllocResize = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x42,

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
            HyperCallCode::HMicroVMQueryIrqRoute => {
                write!(f, "HMicroVMQueryIrqRoute {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMStartVcpu => {
                write!(f, "HMicroVMStartVcpu {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMSendIpi => {
                write!(f, "HMicroVMSendIpi {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMSetVcpuCount => {
                write!(f, "HMicroVMSetVcpuCount {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMGetVcpuCount => {
                write!(f, "HMicroVMGetVcpuCount {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMDrainVfioMailbox => {
                write!(f, "HMicroVMDrainVfioMailbox {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocRegisterZone => {
                write!(f, "HMicroVMHyperAllocRegisterZone {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocInstall => {
                write!(f, "HMicroVMHyperAllocInstall {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocNotifyPagecacheDropped => {
                write!(
                    f,
                    "HMicroVMHyperAllocNotifyPagecacheDropped {:#x}",
                    *self as u32
                )
            }
            HyperCallCode::HMicroVMHyperAllocQuery => {
                write!(f, "HMicroVMHyperAllocQuery {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocVfioDmaPoll => {
                write!(f, "HMicroVMHyperAllocVfioDmaPoll {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocVfioDmaComplete => {
                write!(f, "HMicroVMHyperAllocVfioDmaComplete {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocDebugReclaim => {
                write!(f, "HMicroVMHyperAllocDebugReclaim {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocResize => {
                write!(f, "HMicroVMHyperAllocResize {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocPagecacheShrinkReq => {
                write!(
                    f,
                    "HMicroVMHyperAllocPagecacheShrinkReq {:#x}",
                    *self as u32
                )
            }
            HyperCallCode::HMicroVMGuestMemCopy => {
                write!(f, "HMicroVMGuestMemCopy {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMGuestRamMmapStateUpdate => {
                write!(f, "HMicroVMGuestRamMmapStateUpdate {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocMemoryTarget => {
                write!(f, "HMicroVMHyperAllocMemoryTarget {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocHostQuery => {
                write!(f, "HMicroVMHyperAllocHostQuery {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocVfioDmaDebugRequest => {
                write!(
                    f,
                    "HMicroVMHyperAllocVfioDmaDebugRequest {:#x}",
                    *self as u32
                )
            }
            HyperCallCode::HMicroVMHyperAllocEqGateDrain => {
                write!(f, "HMicroVMHyperAllocEqGateDrain {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocEqGateDebugEnqueue => {
                write!(
                    f,
                    "HMicroVMHyperAllocEqGateDebugEnqueue {:#x}",
                    *self as u32
                )
            }
            HyperCallCode::HMicroVMHyperAllocEqGateEnqueue => {
                write!(f, "HMicroVMHyperAllocEqGateEnqueue {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMGuestRamTranslate => {
                write!(f, "HMicroVMGuestRamTranslate {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMGuestRamMmapZapPoll => {
                write!(f, "HMicroVMGuestRamMmapZapPoll {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMGuestRamMmapZapComplete => {
                write!(f, "HMicroVMGuestRamMmapZapComplete {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMRemove => {
                write!(f, "HMicroVMRemove {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMStop => {
                write!(f, "HMicroVMStop {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMGateEptSync => {
                write!(f, "HMicroVMGateEptSync {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocDebugReclaimBatch => {
                write!(f, "HMicroVMHyperAllocDebugReclaimBatch {:#x}", *self as u32)
            }
            HyperCallCode::HMicroVMHyperAllocDebugRegistryRace => {
                write!(f, "HMicroVMHyperAllocDebugRegistryRace {:#x}", *self as u32)
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
