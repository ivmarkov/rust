use crate::spec::{LinkerFlavor, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".to_string(),
        llvm_target: "riscv32".to_string(),
        pointer_width: 32,
        arch: "riscv32".to_string(),

        options: TargetOptions {
            os_family: Some("unix".to_string()),
            os: "none".to_string(),
            env: "newlib".to_string(),
            vendor: "espressif".to_string(),
            linker_flavor: LinkerFlavor::Gcc,
            linker: Some("xtensa-esp32c3-elf-gcc".to_string()),
            cpu: "generic-rv32".to_string(),

            // See https://github.com/espressif/rust-esp32-example/issues/3#issuecomment-861054477
            //
            // Unlike the original ESP32 chip, ESP32-C3 does not really support atomics.
            // If the missing hardware instruction ends up being emulated in ESP-IDF, we might want to revert
            // this change and claim that atomics are supported "in hardware" (even though they would be emulated
            // by actually trapping the illegal instruction exception handler and calling into an ESP-IDF C emulation code).
            //
            // However, for now we just simultaneously claim "max_atomic_width: Some(32)" **and** atomic_cas: true,
            // which should force  the compiler to generate libcalls to functions that emulate atomics
            // and which are already implemented in the ESP-IDF main branch anyway.
            max_atomic_width: Some(32),
            atomic_cas: true,

            features: "+m,+c".to_string(),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            unsupported_abis: super::riscv_base::unsupported_abis(),
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
