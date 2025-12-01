# network protocol

In the `chatproto/src/netproto` directory, edit `decode.rs` and `encode.rs`. The network protocol
work in the following way:

 * structs are encoded by encoding each field, in the order they appear in the definition
 * enums are encoded by first inserting a byte designing the variant we are in (starting at 0),
   and then encoding the enum fields
 * numbers are encoded in the following way:
    * with `u < 251`, encode it as a single byte with that value,
    * with `251 <= u < 2**16`, encode it as `251` followed by a little-endian `u16` value,
    * with `2**16 <= u < 2**32`, encode it as `252` followed by a little-endian `u32` value,
    * with `2**32 <= u < 2**64`, encode it as `253` followed by a little-endian `u64` value,
    * with `2**64 <= u < 2**128`, encode it as `254` followed by a little-endian `u128` value.
 * collections are encoded by first putting their sizes, and they the contents

You can test your implementation by running:

```
cargo test netproto
```

It is suggested to implement the encoding/decoding pair of functions in the order they are presented.
