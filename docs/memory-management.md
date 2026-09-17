# Memory Management

This document describes the memory management model used by the LLVM compiler backend: automatic reference counting (refcounting) for composite types and strings, ownership rules when passing values around, and known limitations.

## 1. Primitives vs. heap

* **Primitives** (`i8`/`i16`/`i32`/`i64`, `u8`/`u16`/`u32`/`u64`, `f64`, `bool`, `char`) live entirely on the stack / in registers. Copying them is a plain `store`, with no bookkeeping at all.

* **`str`, `T[]` (vector), struct, and enum** always live on the heap and are managed by the refcounting scheme described below.

An enum is represented by a heap-allocated `EnumHeader` containing a reference count, a variant tag, and raw storage for the active variant's payload.

## 2. Heap object layout

Every managed object has a header with a reference count `refcount: i64`.

### `StrHeader`

```text
struct StrHeader {
    i64 refcount;
    i8* data;       // NUL-terminated character buffer, separate malloc
}
```

`LlvmValue::Str` **always** points at a `StrHeader`, never directly at the character buffer. Access to the characters always goes through the `data` field (see `Compiler::str_data_ptr`).

### `VecHeader`

```text
struct VecHeader {
    i64 refcount;
    T*  data;       // backing array, separate malloc
    i64 length;
    i64 capacity;
}
```

The vector's `data` points to a separately allocated backing array. Elements that are themselves managed values (`str`, vectors, structs, or enums) are retained/released individually when the vector is copied or destroyed.

### User struct

The declared fields plus one extra, hidden field appended at the end:

```text
struct <Name> {
    <fields declared by the user>;
    i64 refcount;   // appended automatically by struct_llvm_type
}
```

A struct is therefore itself a refcounted heap object. Its composite fields can point to other managed objects and are retained/released together with the struct.

### `EnumHeader`

Enums are represented as heap-allocated objects with the following layout:

```text
struct EnumHeader {
    i64 refcount;
    i64 tag;
    [N x i64] payload;
}
```

The field order is fixed:

* `ENUM_REFCOUNT = 0` — reference count;
* `ENUM_TAG = 1` — zero-based index of the active variant;
* `ENUM_PAYLOAD = 2` — raw storage for the active variant's payload.

`N` is large enough to hold the largest payload used by any variant of that enum. The payload is raw union-like storage: it is not interpreted as a complete `[N x i64]` value. The compiler GEPs into the payload and loads/stores it using the concrete type of the active variant.

For example:

```raptor
enum Task {
    InProgress(Deadline),
    Completed(str),
    Aborted
};
```

is represented conceptually as:

```text
EnumHeader {
    refcount,
    tag,
    payload
}
```

where `tag` identifies the active variant and `payload` contains the corresponding `Deadline` or `str` value.

`LlvmValue::Enum` stores a pointer to the `EnumHeader` together with the enum's `Type`, which contains the variant names and payload types.

An enum's payload may itself be a managed value such as `str`, `T[]`, a struct, or another enum. In such cases the enum owns a reference to that payload and must retain/release it as part of the enum's lifetime.

## 3. Value vs. reference semantics

| Type      | `let b = a;`                        | argument by value                                                                                                                   | argument by `&` (reference)          |
| --------- | ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------ |
| primitive | bitwise copy                        | bitwise copy                                                                                                                        | pointer to the caller's variable     |
| `str`     | **full, deep copy** of a new buffer | **full, deep copy**                                                                                                                 | pointer to the caller's `StrHeader`  |
| `T[]`     | retain (shared reference)           | **shallow copy**: new `VecHeader` + new backing array; composite elements (`str`/`T[]`/struct/enum) are *retained*, not deep-copied | pointer to the caller's `VecHeader`  |
| `struct`  | retain (shared reference)           | **shallow copy**: new struct, primitive fields copied bitwise, composite fields (`str`/vector/struct/enum) *retained*               | pointer to the caller's struct       |
| `enum`    | retain (shared reference)           | **shallow copy** of the enum object; primitive payloads are copied directly, managed payloads are *retained*                        | pointer to the caller's `EnumHeader` |

Strings **always** get full value semantics - never shared, even though they're refcounted under the hood. This is intentional: the refcount on a string exists mainly so releasing goes through one consistent mechanism (`release_value`), not so strings can be shared.

Enums, vectors, and structs use shared-reference semantics when assigned with `let b = a;`: the underlying heap object is retained rather than duplicated.

When an enum is copied by value, the enum container itself is copied into a new enum object. The active variant's payload is copied according to its type:

* primitive payloads are copied directly;
* `str` payloads are deep-copied according to string value semantics;
* vector, struct, and enum payloads are retained and therefore remain shared.

For example:

```raptor
enum Result {
    Number(i64),
    Message(str),
    Data(i64[]),
    Nested(Other)
};
```

Copying a `Result::Data` value creates a new enum object, but both enum objects retain the same underlying vector.

## 4. Retain / release - when they happen

### Base rule: "new reference +1"

Any expression whose result is **anything other than a bare variable read** (`Expression::Variable`) returns an owned (+1) reference by construction:

* a fresh literal (`[1,2,3]`, `Struct { ... }`, `Enum::Variant(...)`);
* a function call result;
* a struct field read (`obj.field`) or vector element read (`arr[i]`) - these two specifically retain at the point of reading, precisely to satisfy this rule;
* an enum payload/field read when it produces a managed value;
* a cast, concatenation, vector stringify result, etc.

Reading a bare variable (`x`) is **not** a "new reference" - it's a borrow of the variable's own reference.

Consequence: whenever an expression's result lands in a new owning slot (`let`, assignment, struct field, vector element, by-value argument, `return`), an extra `retain` is needed **only** if the source expression was a bare variable - in every other case the result already carries its own +1 and nothing further is needed.

For enum values, the same ownership rule applies to the enum object itself. If the enum contains a managed payload, the payload's reference must also be balanced when the enum is copied, replaced, or released.

See: `Compiler::expr_needs_retain` / `Compiler::expr_needs_release` in `memory.rs`.

### Scope release

Every block `{ ... }` is its own lexical scope (`Compiler::push_scope` / `pop_scope_and_release`, wired up through `visit_block`). Owning variables declared in that block are released automatically at the end of the block, in reverse declaration order - unless the block already ended via `return`/`break`/`continue` (in which case release already happened earlier, see below).

By-value function parameters live in their own, outer scope (opened before the function body) - the function owns them and releases them at the end, unless an explicit `return` releases them earlier.

Enums are released like other refcounted heap objects. Releasing an enum decrements its own reference count; when it reaches zero, its active managed payload must also be released.

### `return`

1. Evaluate the return value.

2. If the returned expression is a bare variable (`return x;`) and its type is managed - `retain` it **before** releasing scopes (so it survives the release of its own local variable).

3. Release **every** active scope of the function (parameters + all nested blocks).

4. Build the `ret` instruction.

The same rule applies when the returned value is an enum: the enum itself must survive the release of the local variable, and its active managed payload remains owned by the returned enum.

### `break` / `continue`

Release every scope opened **since** entering the loop/switch (not counting the loop's own declaration scope, e.g. `for (let i = ...)`), before jumping to the target. The exact threshold (`scope_depth`) is recorded in `ControlFrame` at the point the loop/switch is entered.

### User function call (by value)

1. Evaluate the argument.

2. Build a copy:

   * string: deep copy;
   * vector/struct: shallow copy;
   * enum: new enum object with its active payload copied according to the payload type.

3. If the source expression was **not** a bare variable (i.e. it already carried its own +1 per the base rule), release the original - copying already consumed its contents and nothing else needs it anymore.

4. The copied value is handed to the callee as a brand new, fully independent owner (parameter), released by the callee on exit.

For enum arguments, copying only duplicates the enum container. Managed payloads remain shared through their own reference counts.

### Built-in functions (`vector_size`, `str_len`, `vector_stringify`, `vector_push`, ...)

These functions do **not** go through the argument-copying mechanism above - they call `visit_expression` on the argument directly and only "peek" at the value (length, data pointer), never storing it anywhere. If the argument was a `FieldAccess`/`Index`/fresh expression (i.e. something carrying a +1 per the base rule), the function must release that +1 after use, or it leaks. Reading a bare variable requires no release (nothing was retained).

The same rule applies to built-ins that inspect or otherwise temporarily access enum values: if the expression produces a new owned enum reference, that reference must be released after the built-in has finished using it.

## 5. A quick sanity check

Practical test for "is this balanced": compare the behavior of `f(x)` (bare variable) vs. `f(obj.field)` / `f(arr[i])` / `f(literal)` - the source object's refcount after the call should be identical in both cases. If it differs, some place isn't balancing retain/release correctly.

For enums, also compare:

```raptor
let a = MyEnum::Data(vector);
let b = a;
```

with:

```raptor
let b = make_enum();
```

The first case should retain/share the same enum object, while the second produces a new owned enum result. If the enum contains a managed payload, its payload reference count must remain balanced across copies and releases.

## 6. Known limitations

### 6.1. Reference cycles - memory leak

Plain refcounting **does not detect cycles**. Two objects holding onto each other (directly, or through a vector or enum payload) will never reach refcount 0, even once nothing external can reach them anymore.

For example, a cycle can be created through structs and vectors:

```raptor
struct Node {
    i64 value,
    Node[] next
};

fn main() {
    let a = Node { value: 1, next: [] };
    let b = Node { value: 2, next: [] };

    a.next = [b];   // retains b
    b.next = [a];   // retains a
}

// end of main: release(a), release(b) bring both rc from 2 down to 1 - never to 0.

// a, b, and their next vectors leak forever.
```

The same problem applies if the cycle passes through an enum payload:

```raptor
struct Node {
    i64 value,
    Link link
};

enum Link {
    Next(Node)
}
```

If managed objects form a cycle through enum payloads, reference counting alone cannot detect that the objects are no longer reachable.

This is a known, accepted weakness of the current model (no weak references / cycle collector). Future fix, if needed: weak references for designated fields, or a separate cycle collector run periodically.

### 6.2. Intermediate expressions with no assignment

If the result of a +1-producing expression (field read, indexing, function call, enum payload access) is used in the middle of a larger expression and never lands anywhere covered by the rules in section 4, that +1 may never be released. Any new place in the compiler that "peeks" at a managed value without storing it (similar to `vector_size`/`str_len`) must manually release that value if `expr_needs_release` returns `true` for the source expression.

This applies equally to enum values and to managed values stored inside enum payloads.
