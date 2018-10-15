# nes-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://travis-ci.org/jeffrey-xiao/nes-rs.svg?branch=master)](https://travis-ci.org/jeffrey-xiao/nes-rs)

An NES emulator written in Rust that compiles to WebAssembly.

## Test Rom Progress

CPU
  - instr_misc
    - `01-abs_x_wrap.nes`: Pass
    - `02-branch_wrap.nes`: Pass
    - `03-dummy_reads.nes`: Fail
    - `04-dummy_reads_apu.nes`: Fail
  - instr_test
    - `01-basics.nes`: Pass
    - `02-implied.nes`: Pass
    - `03-immediate.nes`: Fail (unofficial instruction)
    - `04-zero_page.nes`: Pass
    - `05-zp_xy.nes`: Pass
    - `06-absolute.nes`: Pass
    - `07-abs_xy.nes`: Fail (unofficial instruction)
    - `08-ind_x.nes`: Pass
    - `09-ind_y.nes`: Pass
    - `10-branches.nes`: Pass
    - `11-stack.nes`: Pass
    - `12-jmp_jsr.nes`: Pass
    - `13-rts.nes`: Pass
    - `14-rti.nes`: Pass
    - `15-brk.nes`: Pass
    - `16-special.nes`: Pass
  - instr_timing
    - `1-instr_timing.nes`: Fail
    - `2-branch_timing.nes`: Fail
  - interrupts: No progress

## References

## License

`nes-rs` is distributed under the terms of both the MIT License and the Apache License (Version
2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for more details.
