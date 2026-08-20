use std::assert_eq;

use crate::backend::interpreter::alu::value::Value;
use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::common::span::Span;

#[test]
fn add() {
    let data = [
        (Value::I64(1), Value::I64(2)),
        (Value::F64(1.5), Value::F64(2.5)),
        (Value::String(String::from("Papollo")), Value::String(String::from("2137"))),
    ];

    let expected = [Value::I64(3), Value::F64(4.0), Value::String(String::from("Papollo2137"))];

    for idx in 0..data.len() {
        let (val1, val2) = &data[idx];
        assert_eq!(ALU::add(val1.clone(), val2.clone(), Span::default()).unwrap(), expected[idx]);
    }
}

#[test]
fn add_fail() {
    assert_eq!(
        ALU::add(Value::I64(6532475327647647762), Value::I64(6532475327647647762), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Overflow occurred when performing addition on i64 and i64.")
    );
    assert_eq!(
        ALU::add(Value::I64(1), Value::F64(2.0), Span::default()).err().unwrap().message(),
        String::from("Cannot perform addition between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn subtract() {
    let data = [(Value::I64(1), Value::I64(2)), (Value::F64(1.5), Value::F64(2.5))];

    let expected = [Value::I64(-1), Value::F64(-1.0)];

    for idx in 0..data.len() {
        let (val1, val2) = &data[idx];
        assert_eq!(ALU::subtract(val1.clone(), val2.clone(), Span::default()).unwrap(), expected[idx]);
    }
}

#[test]
fn subtract_fail() {
    assert_eq!(
        ALU::subtract(Value::I64(-6532475327647647762), Value::I64(6532475327647647762), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Overflow occurred when performing subtraction on i64 and i64.")
    );
    assert_eq!(
        ALU::subtract(Value::I64(1), Value::F64(2.0), Span::default()).err().unwrap().message(),
        String::from("Cannot perform subtraction between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::subtract(Value::String(String::from("a")), Value::String(String::from("a")), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform subtraction between values of type 'str' and 'str'.")
    );
}

#[test]
fn multiplication() {
    let data = [(Value::I64(1), Value::I64(2)), (Value::F64(1.5), Value::F64(2.5))];

    let expected = [Value::I64(2), Value::F64(3.75)];

    for idx in 0..data.len() {
        let (val1, val2) = &data[idx];
        assert_eq!(ALU::multiplication(val1.clone(), val2.clone(), Span::default()).unwrap(), expected[idx]);
    }
}

#[test]
fn multiplication_fail() {
    assert_eq!(
        ALU::multiplication(Value::I64(6532475327647647762), Value::I64(6532475327647647762), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Overflow occurred when performing multiplication on i64 and i64.")
    );
    assert_eq!(
        ALU::multiplication(Value::I64(1), Value::F64(2.0), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform multiplication between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::multiplication(Value::String(String::from("a")), Value::String(String::from("a")), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform multiplication between values of type 'str' and 'str'.")
    );
}

#[test]
fn division() {
    let data = [(Value::I64(1), Value::I64(2)), (Value::F64(1.5), Value::F64(2.5))];

    let expected = [Value::I64(0), Value::F64(0.6)];

    for idx in 0..data.len() {
        let (val1, val2) = &data[idx];
        assert_eq!(ALU::division(val1.clone(), val2.clone(), Span::default()).unwrap(), expected[idx]);
    }
}

#[test]
fn division_fail() {
    assert_eq!(
        ALU::division(Value::I64(6532475327647647762), Value::I64(0), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Overflow occurred when performing division on i64 and i64.")
    );
    assert_eq!(
        ALU::division(Value::I64(1), Value::F64(2.0), Span::default()).err().unwrap().message(),
        String::from("Cannot perform division between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::division(Value::String(String::from("a")), Value::String(String::from("a")), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform division between values of type 'str' and 'str'.")
    );
}

#[test]
fn modulo() {
    let data = [
        (Value::I64(7), Value::I64(3)),
        (Value::I64(-7), Value::I64(3)),
        (Value::I64(0), Value::I64(5)),
    ];

    let expected = [Value::I64(1), Value::I64(-1), Value::I64(0)];

    for idx in 0..data.len() {
        let (val1, val2) = &data[idx];
        assert_eq!(ALU::modulo(val1.clone(), val2.clone(), Span::default()).unwrap(), expected[idx]);
    }
}

#[test]
fn modulo_fail() {
    assert_eq!(
        ALU::modulo(Value::I64(1), Value::I64(0), Span::default()).err().unwrap().message(),
        String::from("Overflow occurred when performing modulo on i64 and i64.")
    );
    assert_eq!(
        ALU::modulo(Value::I64(1), Value::F64(2.0), Span::default()).err().unwrap().message(),
        String::from("Cannot perform modulo between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::modulo(Value::F64(1.0), Value::F64(2.0), Span::default()).err().unwrap().message(),
        String::from("Cannot perform modulo between values of type 'f64' and 'f64'.")
    );
}

#[test]
fn division_float_edge_cases() {
    assert_eq!(
        ALU::division(Value::F64(1.0), Value::F64(0.0), Span::default()).err().unwrap().message(),
        String::from("Invalid result when performing division on f64s.")
    );
    assert_eq!(
        ALU::division(Value::F64(0.0), Value::F64(0.0), Span::default()).err().unwrap().message(),
        String::from("Invalid result when performing division on f64s.")
    );
}

#[test]
fn add_integer_types() {
    let data = [
        (Value::I8(1), Value::I8(2), Value::I8(3)),
        (Value::I16(1), Value::I16(2), Value::I16(3)),
        (Value::I32(1), Value::I32(2), Value::I32(3)),
        (Value::I64(1), Value::I64(2), Value::I64(3)),
        (Value::U8(1), Value::U8(2), Value::U8(3)),
        (Value::U16(1), Value::U16(2), Value::U16(3)),
        (Value::U32(1), Value::U32(2), Value::U32(3)),
        (Value::U64(1), Value::U64(2), Value::U64(3)),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::add(val1, val2, Span::default()).unwrap(), expected);
    }
}

#[test]
fn add_overflow_all_integer_types() {
    let data = [
        (
            Value::I8(i8::MAX),
            Value::I8(1),
            "Overflow occurred when performing addition on i8 and i8.",
        ),
        (
            Value::I16(i16::MAX),
            Value::I16(1),
            "Overflow occurred when performing addition on i16 and i16.",
        ),
        (
            Value::I32(i32::MAX),
            Value::I32(1),
            "Overflow occurred when performing addition on i32 and i32.",
        ),
        (
            Value::I64(i64::MAX),
            Value::I64(1),
            "Overflow occurred when performing addition on i64 and i64.",
        ),
        (
            Value::U8(u8::MAX),
            Value::U8(1),
            "Overflow occurred when performing addition on u8 and u8.",
        ),
        (
            Value::U16(u16::MAX),
            Value::U16(1),
            "Overflow occurred when performing addition on u16 and u16.",
        ),
        (
            Value::U32(u32::MAX),
            Value::U32(1),
            "Overflow occurred when performing addition on u32 and u32.",
        ),
        (
            Value::U64(u64::MAX),
            Value::U64(1),
            "Overflow occurred when performing addition on u64 and u64.",
        ),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::add(val1, val2, Span::default()).err().unwrap().message(), expected);
    }
}

#[test]
fn add_char_and_string_variants() {
    let data = [
        (Value::Char('a'), Value::Char('b'), Value::String(String::from("ab"))),
        (Value::Char('a'), Value::String(String::from("bc")), Value::String(String::from("abc"))),
        (Value::String(String::from("ab")), Value::Char('c'), Value::String(String::from("abc"))),
        (
            Value::String(String::from("ab")),
            Value::String(String::from("cd")),
            Value::String(String::from("abcd")),
        ),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::add(val1, val2, Span::default()).unwrap(), expected);
    }
}

#[test]
fn add_float_invalid_results() {
    assert_eq!(
        ALU::add(Value::F64(f64::INFINITY), Value::F64(1.0), Span::default())
            .err()
            .unwrap()
            .message(),
        "Invalid result when performing addition on f64s."
    );

    assert_eq!(
        ALU::add(Value::F64(f64::NAN), Value::F64(1.0), Span::default()).err().unwrap().message(),
        "Invalid result when performing addition on f64s."
    );
}

#[test]
fn subtract_integer_types() {
    let data = [
        (Value::I8(3), Value::I8(2), Value::I8(1)),
        (Value::I16(3), Value::I16(2), Value::I16(1)),
        (Value::I32(3), Value::I32(2), Value::I32(1)),
        (Value::I64(3), Value::I64(2), Value::I64(1)),
        (Value::U8(3), Value::U8(2), Value::U8(1)),
        (Value::U16(3), Value::U16(2), Value::U16(1)),
        (Value::U32(3), Value::U32(2), Value::U32(1)),
        (Value::U64(3), Value::U64(2), Value::U64(1)),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::subtract(val1, val2, Span::default()).unwrap(), expected);
    }
}

#[test]
fn subtract_underflow_all_integer_types() {
    let data = [
        (
            Value::I8(i8::MIN),
            Value::I8(1),
            "Overflow occurred when performing subtraction on i8 and i8.",
        ),
        (
            Value::I16(i16::MIN),
            Value::I16(1),
            "Overflow occurred when performing subtraction on i16 and i16.",
        ),
        (
            Value::I32(i32::MIN),
            Value::I32(1),
            "Overflow occurred when performing subtraction on i32 and i32.",
        ),
        (
            Value::I64(i64::MIN),
            Value::I64(1),
            "Overflow occurred when performing subtraction on i64 and i64.",
        ),
        (Value::U8(0), Value::U8(1), "Overflow occurred when performing subtraction on u8 and u8."),
        (
            Value::U16(0),
            Value::U16(1),
            "Overflow occurred when performing subtraction on u16 and u16.",
        ),
        (
            Value::U32(0),
            Value::U32(1),
            "Overflow occurred when performing subtraction on u32 and u32.",
        ),
        (
            Value::U64(0),
            Value::U64(1),
            "Overflow occurred when performing subtraction on u64 and u64.",
        ),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::subtract(val1, val2, Span::default()).err().unwrap().message(), expected);
    }
}

#[test]
fn subtract_float_invalid_results() {
    assert_eq!(
        ALU::subtract(Value::F64(f64::INFINITY), Value::F64(f64::INFINITY), Span::default())
            .err()
            .unwrap()
            .message(),
        "Invalid result when performing subtraction on f64s."
    );

    assert_eq!(
        ALU::subtract(Value::F64(f64::NAN), Value::F64(1.0), Span::default())
            .err()
            .unwrap()
            .message(),
        "Invalid result when performing subtraction on f64s."
    );
}

#[test]
fn multiplication_integer_types() {
    let data = [
        (Value::I8(2), Value::I8(3), Value::I8(6)),
        (Value::I16(2), Value::I16(3), Value::I16(6)),
        (Value::I32(2), Value::I32(3), Value::I32(6)),
        (Value::I64(2), Value::I64(3), Value::I64(6)),
        (Value::U8(2), Value::U8(3), Value::U8(6)),
        (Value::U16(2), Value::U16(3), Value::U16(6)),
        (Value::U32(2), Value::U32(3), Value::U32(6)),
        (Value::U64(2), Value::U64(3), Value::U64(6)),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::multiplication(val1, val2, Span::default()).unwrap(), expected);
    }
}

#[test]
fn multiplication_overflow_all_integer_types() {
    let data = [
        (
            Value::I8(i8::MAX),
            Value::I8(2),
            "Overflow occurred when performing multiplication on i8 and i8.",
        ),
        (
            Value::I16(i16::MAX),
            Value::I16(2),
            "Overflow occurred when performing multiplication on i16 and i16.",
        ),
        (
            Value::I32(i32::MAX),
            Value::I32(2),
            "Overflow occurred when performing multiplication on i32 and i32.",
        ),
        (
            Value::I64(i64::MAX),
            Value::I64(2),
            "Overflow occurred when performing multiplication on i64 and i64.",
        ),
        (
            Value::U8(u8::MAX),
            Value::U8(2),
            "Overflow occurred when performing multiplication on u8 and u8.",
        ),
        (
            Value::U16(u16::MAX),
            Value::U16(2),
            "Overflow occurred when performing multiplication on u16 and u16.",
        ),
        (
            Value::U32(u32::MAX),
            Value::U32(2),
            "Overflow occurred when performing multiplication on u32 and u32.",
        ),
        (
            Value::U64(u64::MAX),
            Value::U64(2),
            "Overflow occurred when performing multiplication on u64 and u64.",
        ),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::multiplication(val1, val2, Span::default()).err().unwrap().message(), expected);
    }
}

#[test]
fn multiplication_float_invalid_results() {
    assert_eq!(
        ALU::multiplication(Value::F64(f64::MAX), Value::F64(2.0), Span::default())
            .err()
            .unwrap()
            .message(),
        "Invalid result when performing multiplication on f64s."
    );

    assert_eq!(
        ALU::multiplication(Value::F64(f64::NAN), Value::F64(2.0), Span::default())
            .err()
            .unwrap()
            .message(),
        "Invalid result when performing multiplication on f64s."
    );
}

#[test]
fn division_integer_types() {
    let data = [
        (Value::I8(7), Value::I8(3), Value::I8(2)),
        (Value::I16(7), Value::I16(3), Value::I16(2)),
        (Value::I32(7), Value::I32(3), Value::I32(2)),
        (Value::I64(7), Value::I64(3), Value::I64(2)),
        (Value::U8(7), Value::U8(3), Value::U8(2)),
        (Value::U16(7), Value::U16(3), Value::U16(2)),
        (Value::U32(7), Value::U32(3), Value::U32(2)),
        (Value::U64(7), Value::U64(3), Value::U64(2)),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::division(val1, val2, Span::default()).unwrap(), expected);
    }
}

#[test]
fn division_by_zero_all_integer_types() {
    let data = [
        (Value::I8(1), Value::I8(0), "Overflow occurred when performing division on i8 and i8."),
        (Value::I16(1), Value::I16(0), "Overflow occurred when performing division on i16 and i16."),
        (Value::I32(1), Value::I32(0), "Overflow occurred when performing division on i32 and i32."),
        (Value::I64(1), Value::I64(0), "Overflow occurred when performing division on i64 and i64."),
        (Value::U8(1), Value::U8(0), "Overflow occurred when performing division on u8 and u8."),
        (Value::U16(1), Value::U16(0), "Overflow occurred when performing division on u16 and u16."),
        (Value::U32(1), Value::U32(0), "Overflow occurred when performing division on u32 and u32."),
        (Value::U64(1), Value::U64(0), "Overflow occurred when performing division on u64 and u64."),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::division(val1, val2, Span::default()).err().unwrap().message(), expected);
    }
}

#[test]
fn division_signed_min_by_minus_one() {
    let data = [
        (
            Value::I8(i8::MIN),
            Value::I8(-1),
            "Overflow occurred when performing division on i8 and i8.",
        ),
        (
            Value::I16(i16::MIN),
            Value::I16(-1),
            "Overflow occurred when performing division on i16 and i16.",
        ),
        (
            Value::I32(i32::MIN),
            Value::I32(-1),
            "Overflow occurred when performing division on i32 and i32.",
        ),
        (
            Value::I64(i64::MIN),
            Value::I64(-1),
            "Overflow occurred when performing division on i64 and i64.",
        ),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::division(val1, val2, Span::default()).err().unwrap().message(), expected);
    }
}

#[test]
fn division_float_invalid_results() {
    let data = [
        (Value::F64(1.0), Value::F64(0.0)),
        (Value::F64(0.0), Value::F64(0.0)),
        (Value::F64(f64::INFINITY), Value::F64(2.0)),
        (Value::F64(f64::NAN), Value::F64(2.0)),
    ];

    for (val1, val2) in data {
        assert_eq!(
            ALU::division(val1, val2, Span::default()).err().unwrap().message(),
            "Invalid result when performing division on f64s."
        );
    }
}

#[test]
fn modulo_integer_types() {
    let data = [
        (Value::I8(7), Value::I8(3), Value::I8(1)),
        (Value::I16(7), Value::I16(3), Value::I16(1)),
        (Value::I32(7), Value::I32(3), Value::I32(1)),
        (Value::I64(7), Value::I64(3), Value::I64(1)),
        (Value::U8(7), Value::U8(3), Value::U8(1)),
        (Value::U16(7), Value::U16(3), Value::U16(1)),
        (Value::U32(7), Value::U32(3), Value::U32(1)),
        (Value::U64(7), Value::U64(3), Value::U64(1)),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::modulo(val1, val2, Span::default()).unwrap(), expected);
    }
}

#[test]
fn modulo_unsigned_underflow_cases() {
    let data = [
        (Value::U8(1), Value::U8(2), Value::U8(1)),
        (Value::U16(1), Value::U16(2), Value::U16(1)),
        (Value::U32(1), Value::U32(2), Value::U32(1)),
        (Value::U64(1), Value::U64(2), Value::U64(1)),
    ];

    // Remainder itself does not underflow for a non-zero divisor.
    // These cases verify the mathematical result for unsigned integers.
    for (val1, val2, expected) in data {
        assert_eq!(ALU::modulo(val1, val2, Span::default()).unwrap(), expected);
    }
}

#[test]
fn modulo_signed_min_by_minus_one() {
    let data = [
        (
            Value::I8(i8::MIN),
            Value::I8(-1),
            "Overflow occurred when performing modulo on i8 and i8.",
        ),
        (
            Value::I16(i16::MIN),
            Value::I16(-1),
            "Overflow occurred when performing modulo on i16 and i16.",
        ),
        (
            Value::I32(i32::MIN),
            Value::I32(-1),
            "Overflow occurred when performing modulo on i32 and i32.",
        ),
        (
            Value::I64(i64::MIN),
            Value::I64(-1),
            "Overflow occurred when performing modulo on i64 and i64.",
        ),
    ];

    for (val1, val2, expected) in data {
        assert_eq!(ALU::modulo(val1, val2, Span::default()).err().unwrap().message(), expected);
    }
}

#[test]
fn all_operations_reject_mixed_numeric_types() {
    assert_eq!(
        ALU::add(Value::I32(1), Value::I64(2), Span::default()).err().unwrap().message(),
        "Cannot perform addition between values of type 'i32' and 'i64'."
    );

    assert_eq!(
        ALU::subtract(Value::U32(1), Value::U64(2), Span::default()).err().unwrap().message(),
        "Cannot perform subtraction between values of type 'u32' and 'u64'."
    );

    assert_eq!(
        ALU::multiplication(Value::I32(2), Value::F64(2.0), Span::default())
            .err()
            .unwrap()
            .message(),
        "Cannot perform multiplication between values of type 'i32' and 'f64'."
    );

    assert_eq!(
        ALU::division(Value::U32(4), Value::I64(2), Span::default()).err().unwrap().message(),
        "Cannot perform division between values of type 'u32' and 'i64'."
    );

    assert_eq!(
        ALU::modulo(Value::I32(4), Value::U32(2), Span::default()).err().unwrap().message(),
        "Cannot perform modulo between values of type 'i32' and 'u32'."
    );
}
