use crate::spec::{abi::Abi, LinkerFlavor, PanicStrategy, Target, TargetOptions, RelocModel};
use crate::abi::Endian;

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".to_string(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-n32".to_string(),
        arch: "xtensa".to_string(),

        options: TargetOptions {
            endian: Endian::Little,
            c_int_width: "32".to_string(),
            os_family: Some("unix".to_string()),
            os: "none".to_string(),
            env: "newlib".to_string(),
            vendor: "espressif".to_string(),
            linker_flavor: LinkerFlavor::Gcc,

            executables: true,
            cpu: "esp32-s2".to_string(),
            linker: Some("xtensa-esp32s2-elf-gcc".to_string()),

            // See https://github.com/espressif/rust-esp32-example/issues/3#issuecomment-861054477
            //
            // Unlike the original ESP32 chip, ESP32-S2 does not really support atomics.
            // If the missing hardware instruction ends up being emulated in ESP-IDF, we might want to revert
            // this change and claim that atomics are supported "in hardware" (even though they would be emulated
            // by actually trapping the illegal instruction exception handler and calling into an ESP-IDF C emulation code).
            //
            // However, for now we just expose the reality and hope to provide atomics support with polyfills like
            // https://github.com/embassy-rs/atomic-polyfill
            //
            // The trick with supporting atomics by emulating the missing instruction might anyway be best supported
            // with a separate, dedicated target.
            max_atomic_width: Some(0),
            atomic_cas: false,

            // Because these devices have very little resources having an
            // unwinder is too onerous so we default to "abort" because the
            // "unwind" strategy is very rare.
            panic_strategy: PanicStrategy::Abort,

            // Similarly, one almost always never wants to use relocatable
            // code because of the extra costs it involves.
            relocation_model: RelocModel::Static,

            emit_debug_gdb_scripts: false,

            unsupported_abis: vec![
                Abi::Stdcall { unwind: false },
                Abi::Stdcall { unwind: true },
                Abi::Fastcall,
                Abi::Vectorcall,
                Abi::Thiscall { unwind: false },
                Abi::Thiscall { unwind: true },
                Abi::Win64,
                Abi::SysV64,
            ],

            ..Default::default()
        },
    }
}