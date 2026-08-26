# AtomicMatrix Derive

AtomicMatrix Derive provides a proc-macro for deriving the `SafeSHM` marker trait into custom structs. Structs without this marker trait are blocked from being allocated into the matrix at compile time in order to avoid heap dependant values like Strings and Vecs from being allocated with dangling pointer, which could cause UB in cross-process environments.

AtomicMatrix already implements `SafeSHM` to most safe rust primitives natively. If some primitive has not been included, the caller can use the default trait implementation method to include this primitive as safe.

Please refrain from implementing it to heap dependant types (TLDR: don't be stupid)

### Usage

```rust
use atomic_matrix::prelude::* // The proc-macro is automatically re-exported through the AtomicMatrix prelude.

// This structure is now consider SHM safe and can be allocated without issues.
#[derive(SafeSHM)]
struct MyCustomStruct {
    val1: u32,
    val2: u64,
    unsafeVal: UnsafeCell<T>
}

// This structure will throw a borrow checker error, as String does not implement SafeSHM natively.
#[derive(SafeSHM)]
struct InvalidStruct {
    val1: AtomicU8,
    invalid_val: String // This will cause an error!
}
```

### Native primitives

The native primitives that comes with `SafeSHM` out of the box are:

- All unsigned integers
- All signed integers
- Floating point numbers
- Cells (Safe and Unsafe)
- Options
- All atomic primitives (Signed and Unsigned integers and bools)
- Functions
- Chars
- PhantomData

### Enums
The proc-macro is struct native. If you need to implement the marker into an enum, AtomicMatrix comes with a standard macro safe_shm! that implements the marker into whatever you may require.

### License
MIT