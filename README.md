# ccLynx

ccLynx implements a subset of C compiler for the Atari Lynx. It is based on [cc6502](https://github.com/steux/cc6502) and [cc7800](https://github.com/steux/cc7800).

The main goal of ccLynx is to enable making games for the Atari Lynx using C language, not to provide full C support for the 6502.

ccLynx produces assembly code that is then assembled using [lyxass](https://codeberg.org/42Bastian/lyxass).

## Requirements

ccLynx requires [lyxass](https://codeberg.org/42Bastian/lyxass) to be installed and available in your system PATH.

## Known limitations

- The only data types supported are char (8-bit), short (16-bit) and char pointers (16-bits), and one dimensional arrays of these types
- Array subscripts are preferably constants, X and Y variables / registers
- 16-bit arithmetic is constrained; complex expressions may generate errors
- No 32-bit operations, no floating point
- Works with one C file; use `#include "other_file.c"` for multiple files
- Functions are not reentrant

## How to install

Installing from source requires Rust Cargo and [lyxass](https://codeberg.org/42Bastian/lyxass).

### Prerequisites

1. **Rust**: Install Rust using [rustup](https://www.rust-lang.org/tools/install) if not already available
2. **lyxass**: Install from [https://codeberg.org/42Bastian/lyxass](https://codeberg.org/42Bastian/lyxass) and ensure it is in your system PATH

### Building ccLynx

Use `cargo install --path .` in the root directory to compile and install ccLynx locally.

Alternatively, if you have Rust installed, you can use ccLynx directly with `cargo run --`:
```bash
cargo run -- [OPTIONS] <INPUT>
```

## Examples

Several examples are available in the `examples` directory. To compile an example:

```bash
cargo run -- examples/bank.c -I./headers -obank.lnx
```

This will produce `a.lnx`, which is an Atari Lynx cartridge with proper .LNX header.

To keep the intermediate assembly file for inspection:

```bash
cargo run -- examples/graphics.c -I./headers -ographics.lnx --keep-asm
```

## Command-line options

```
Usage: ccLynx [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file name [default: stdin]

Options:
  -I <INCLUDE_DIRECTORIES>
          Include directories

  -o, --output <OUTPUT>
          Output file name [default: a.out]

  --insert-code
          Insert C code as comments in generated assembly

  --fsigned-char
          Set char signedness to signed

  --funsigned-char
          Set char signedness to unsigned (default)

  --version
          Print compiler version

  --cart-version <CART_VERSION>
          Cart header version [default: 1]

  --name <NAME>
          Cart name/title [default: none]

  --author <AUTHOR>
          Cart author [default: none]

  --blocksize <BLOCK_SIZE>
          Block size for the cartridge [default: 1024]

  --rotation <ROTATION>
          Cart rotation mode [possible values: none, left, right] [default: none]

  --aud <AUD>
          Cart AUD [default: 0]

  --eeprom <EEPROM>
          Cart EEPROM type [possible values: none, 93C46_16, 93C56_16, 93C66_16, 93C76_16, 93C86_16, 93C46_8, 93C56_8, 93C66_8, 93C76_8, 93C86_8] [default: none]

  --keep-asm
          Keep intermediate .s assembly file

  -h, --help
          Print help
```

## Technical details

### Zero Page variables with `zp` keyword

The `zp` keyword declares variables that will be allocated in the zeropage memory space.

```c
zp unsigned char current_sprite;
zp unsigned short position_x;
```

### Bankswitching with `bank` and `org` keywords

The Atari Lynx cartridge can contain multiple banks of ROM code. The `bank` keyword allows you to place functions and data in specific banks, and the `org` keyword specifies the destination address.

**Bank organization:**

```c
bank0 org 0x300;    // Bank 0 code will be loaded at $0300
bank1 org 0x2000;   // Bank 1 code will be loaded at $2000

bank0 void main() {
    // Code in bank 0
}

bank1 void bank1_function() {
    // Code in bank 1
}

bank1 const char * bank1_string = "Hello from bank 1";
```

Functions and data without a bank prefix are placed in bank 0.

To load to a bank at runtime, use the `load_bank()` function (provided by the header files):

```c
load_bank(1);      // Switch to bank 1
bank1_function();  // Call function in bank 1
```

### X and Y Registers

The 6502 X and Y registers can be used as `unsigned char` variables directly in your C code:

```c
X++;              // Increment X register
Y = 10;           // Load Y register with value 10
if (X == Y) {     // Compare X and Y registers
    // ...
}
Y = palette[X];   // Use X as index for the palette array
```

This provides direct access to the 6502's index registers, which is useful for loop counters and performance-critical code.

**Note**: X and Y are optimized as `signed char` in comparisons with 0 for loop optimization, despite being typed as `unsigned char`.

### Intrinsics

ccLynx (via cc6502) supports several intrinsics for low-level operations:

- `load(expr)` - loads the 6502 accumulator with `expr` (LDA instruction)
- `store(expr)` - stores `expr` into the accumulator (STA instruction)
- `strobe(pointer)` - STA instruction; typically used for `strobe(WSYNC)`
- `asm(string)` - inlines the given assembler instruction

Example:

```c
asm("jsr asm_function");  // Call an external assembly function
```
