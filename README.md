Heap Bytes
==========

Motivation
----------

My `DeterministicHeapBytes` crate [implemented](https://github.com/dfinity/ic/tree/master/packages/ic-heap-bytes)
for the Internet Computer is deterministic, but this might not be what most people want.

The widely used [get-size](https://crates.io/crates/get-size) crate, on the other hand,
is non-deterministic.

Neither can be implemented for external types, so both rely on an extensive list of implementations
for built-in, standard library, and other popular crates.

It would be nice to have a solution that covers both deterministic and non-deterministic cases,
while also providing the flexibility to implement a custom memory estimation strategy
for any type as needed.

This crate is an attempt to achieve that.
