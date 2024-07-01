# Pokunes

Emulator NES in Rust written in functional programming style.

## Features

### CPU
List of supported instructions:
- SEI - Set Interrupt Disable
  - `$78`: Implied
- JMP - Jump 
  - `$4C`: Absolute
- STA - Store Accumulator
  - `$8D`: Absolute (in progress)

# Build

Build the application with Cargo:<br/>

`cargo build`<br/>
`cargo build --release`<br/>

# Run

Run the application with Cargo:<br/>

`cargo run <rom_path>`<br/>

# Test

Run the tests with Cargo:<br/>

`cargo test`

Test roms: https://github.com/christopherpow/nes-test-roms