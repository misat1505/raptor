# Memory management

Raptor offers two strategies selected at compile time with `--memory`:

| Mode       | Default | Behaviour                                            |
| ---------- | ------- | ---------------------------------------------------- |
| **`arc`**  | yes     | Automatic reference counting for managed heap values |
| **`leak`** | no      | Allocate with `malloc`; never free                   |

Both strategies are implemented **on top of libc** (`malloc`, `realloc`, `free`). The compiler binary and compiled programs do not link a separate language runtime beyond what the generated IR and libc provide.

```bash
./bootstrap/raptor --memory arc  file.rp
./bootstrap/raptor --memory leak file.rp
./bootstrap/raptor --arc file.rp    # shortcut
./bootstrap/raptor --leak file.rp
```

## 1. Primitives vs heap

- **Primitives** (`i8`…`i64`, `u8`…`u64`, `f64`, `bool`, `char`) live in registers / stack slots. Copies are plain stores.
- **`str`, `T[]`, structs, enums** are heap-allocated when managed under ARC (and still heap-allocated under `leak`, without releases).

## 2. Object headers (ARC)

Managed objects carry a reference count (and, for some shapes, a drop tag). Layouts are defined by the LLVM backend under `src/backend/llvm/`.

### Strings

Dynamic strings are allocated through runtime helpers that wrap **libc** allocation. Values used with C APIs are compatible with `char*` / `strlen` / `printf` style calls declared in `libc-decls.rp`.

String **literals** may be emitted as immortal globals (retain/release are no-ops).

### Vectors

```text
refcount, data pointer, length, capacity
```

The backing array is a separate allocation. Element types that are managed are retained/released when the vector is copied or destroyed under ARC.

### Structs and enums

User structs gain an implicit refcount field. Enums use a tag plus payload storage on the heap. Payload fields that are managed participate in retain/release.

## 3. Retain / release

Under **`arc`**, the backend inserts runtime calls such as:

- `rt_alloc` / related helpers — allocate header + payload via **libc**
- `rt_retain` — bump refcount (immortal / negative sentinel → no-op)
- `rt_release` — drop refcount; at zero, recursively release children and `free`

Under **`leak`**, allocations still use libc `malloc` (and friends), but releases are omitted. Suitable for short-lived tools and simpler IR.

## 4. Moves, copies, and references

- Assigning or passing a managed value by value retains the referent (ARC).
- `&T` parameters borrow without transferring ownership; writes go back to the caller’s storage when the lowering supports it (plain variables; field/element form may use temporaries — see bootstrap status notes).

## 5. Cycles

Plain refcounting does **not** collect reference cycles. Avoid cyclic `struct` graphs or break them explicitly if you need reclamation under ARC.

## 6. libc surface

Allocation and system interaction used by the runtime and stdlib lower to declarations in `src/backend/llvm/libc-decls.rp`, including:

```text
malloc, free, realloc, memcpy
printf, snprintf, fprintf, fflush
strlen, strcpy, strcat, strcmp
fopen, fread, fwrite, fclose, …
socket, bind, listen, accept, connect, recv, send, close, …
```

No other base runtime library is required for a default build.

## 7. Performance note

String construction via repeated `str` concatenation allocates frequently (`realloc` / `malloc`). Prefer building pieces in a vector and joining once, or writing directly to a file, when profiling shows allocation dominance.
