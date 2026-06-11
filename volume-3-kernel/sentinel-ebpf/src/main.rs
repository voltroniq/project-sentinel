#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{kprobe, map},
    maps::PerfEventArray,
    programs::ProbeContext,
    helpers::bpf_get_current_pid_tgid,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SystemEvent {
    pub pid: u32,
    pub event_type: u32,
}

#[map]
static TELEMETRY_STREAM: PerfEventArray<SystemEvent> = PerfEventArray::new(0);

#[kprobe]
pub fn sentinel_kprobe(ctx: ProbeContext) -> u32 {
    match try_sentinel_kprobe(ctx) {
        Ok(ret)  => ret,
        Err(ret) => ret,
    }
}

fn try_sentinel_kprobe(ctx: ProbeContext) -> Result<u32, u32> {
    let tgid = bpf_get_current_pid_tgid() >> 32;
    let pid  = tgid as u32;

    let event = SystemEvent { pid, event_type: 1 };

    TELEMETRY_STREAM.output(&ctx, &event, 0);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}