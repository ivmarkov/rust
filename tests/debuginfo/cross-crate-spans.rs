#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

//@ min-lldb-version: 310

//@ aux-build:cross_crate_spans.rs
extern crate cross_crate_spans;

//@ compile-flags:-g


// === GDB TESTS ===================================================================================

// gdb-command:break cross_crate_spans.rs:14
// gdb-command:run

// gdb-command:print result
// gdbg-check:$1 = {__0 = 17, __1 = 17}
// gdbr-check:$1 = (17, 17)
// gdb-command:print a_variable
// gdb-check:$2 = 123456789
// gdb-command:print another_variable
// gdb-check:$3 = 123456789.5
// gdb-command:continue

// gdb-command:print result
// gdbg-check:$4 = {__0 = 1212, __1 = 1212}
// gdbr-check:$4 = (1212, 1212)
// gdb-command:print a_variable
// gdb-check:$5 = 123456789
// gdb-command:print another_variable
// gdb-check:$6 = 123456789.5
// gdb-command:continue



// === LLDB TESTS ==================================================================================

// lldb-command:b cross_crate_spans.rs:14
// lldb-command:run

// lldb-command:v result
// lldbg-check:[...] { 0 = 17 1 = 17 }
// lldbr-check:((u32, u32)) result = { 0 = 17 1 = 17 }
// lldb-command:v a_variable
// lldbg-check:[...] 123456789
// lldbr-check:(u32) a_variable = 123456789
// lldb-command:v another_variable
// lldbg-check:[...] 123456789.5
// lldbr-check:(f64) another_variable = 123456789.5
// lldb-command:continue

// lldb-command:v result
// lldbg-check:[...] { 0 = 1212 1 = 1212 }
// lldbr-check:((i16, i16)) result = { 0 = 1212 1 = 1212 }
// lldb-command:v a_variable
// lldbg-check:[...] 123456789
// lldbr-check:(u32) a_variable = 123456789
// lldb-command:v another_variable
// lldbg-check:[...] 123456789.5
// lldbr-check:(f64) another_variable = 123456789.5
// lldb-command:continue


// This test makes sure that we can break in functions inlined from other crates.

fn main() {

    let _ = cross_crate_spans::generic_function(17u32);
    let _ = cross_crate_spans::generic_function(1212i16);

}
