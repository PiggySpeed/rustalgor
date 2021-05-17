# Notes

## Variables & Types
- vars are default immutable, can be made mutable
- const are always immutable
- scalars: ints, floats, bools, chars
- compound: tuples, arrays

## Scalars

**Integers**
- default is `i32`
- signed numbers stored as two's complement
- isize/usize depends on arch of computer (64/32-bit); generally for indexing a collection

| Length  | Signed | Unsigned |
|---------|-------:|---------:|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- integer overflow:
  - debug mode: panics during runtime
  - release mode: no panic, two's complement performed
  - dealing with overflow:
    - `wrapping_*`
    - `checked_*` then return `None`
    - `overflowing` then return bool
    - `saturating_*`

**Floats**
- default is `f64`
- two types: `f32` and `f64`
- f32: single-precision; range [2^-126 to 2^127]
- f64: double-precision; range [2^-1022 to 2^1023]

**Bools**
- `true` and `false`

**Chars**
- four bytes unicode
- single quotes e.g. `let a = 'a';`, `let cat = '😻';`

## Compounds

**Tuples**
- `let tup: (i32, char, u128, bool) = (-432, 'e', 939, false);`

**Arrays**
- `let arr = [1, 2, 3, 4, 5];`
- fixed length; allocated on the stack
- `let a = [3; 5];` equivalent to `let a = [3, 3, 3, 3, 3];`
- out-of-bounds handled as runtime error

## Functions

