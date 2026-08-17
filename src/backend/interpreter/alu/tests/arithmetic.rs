use std::assert_eq;

use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::backend::interpreter::alu::value::Value;

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
        assert_eq!(ALU::add(val1.clone(), val2.clone()).unwrap(), expected[idx]);
    }
}

#[test]
fn add_fail() {
    assert_eq!(
        ALU::add(Value::I64(6532475327647647762), Value::I64(6532475327647647762))
            .err()
            .unwrap()
            .message(),
        String::from("Overflow occurred when performing addition on i64s.")
    );
    assert_eq!(
        ALU::add(Value::I64(1), Value::F64(2.0)).err().unwrap().message(),
        String::from("Cannot perform addition between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn subtract() {
    let data = [(Value::I64(1), Value::I64(2)), (Value::F64(1.5), Value::F64(2.5))];

    let expected = [Value::I64(-1), Value::F64(-1.0)];

    for idx in 0..data.len() {
        let (val1, val2) = &data[idx];
        assert_eq!(ALU::subtract(val1.clone(), val2.clone()).unwrap(), expected[idx]);
    }
}

#[test]
fn subtract_fail() {
    assert_eq!(
        ALU::subtract(Value::I64(-6532475327647647762), Value::I64(6532475327647647762))
            .err()
            .unwrap()
            .message(),
        String::from("Overflow occurred when performing subtraction on i64s.")
    );
    assert_eq!(
        ALU::subtract(Value::I64(1), Value::F64(2.0)).err().unwrap().message(),
        String::from("Cannot perform subtraction between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::subtract(Value::String(String::from("a")), Value::String(String::from("a")))
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
        assert_eq!(ALU::multiplication(val1.clone(), val2.clone()).unwrap(), expected[idx]);
    }
}

#[test]
fn multiplication_fail() {
    assert_eq!(
        ALU::multiplication(Value::I64(6532475327647647762), Value::I64(6532475327647647762))
            .err()
            .unwrap()
            .message(),
        String::from("Overflow occurred when performing multiplication on i64s.")
    );
    assert_eq!(
        ALU::multiplication(Value::I64(1), Value::F64(2.0)).err().unwrap().message(),
        String::from("Cannot perform multiplication between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::multiplication(Value::String(String::from("a")), Value::String(String::from("a")))
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
        assert_eq!(ALU::division(val1.clone(), val2.clone()).unwrap(), expected[idx]);
    }
}

#[test]
fn division_fail() {
    assert_eq!(
        ALU::division(Value::I64(6532475327647647762), Value::I64(0)).err().unwrap().message(),
        String::from("Overflow occurred when performing division on i64s.")
    );
    assert_eq!(
        ALU::division(Value::I64(1), Value::F64(2.0)).err().unwrap().message(),
        String::from("Cannot perform division between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::division(Value::String(String::from("a")), Value::String(String::from("a")))
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
        assert_eq!(ALU::modulo(val1.clone(), val2.clone()).unwrap(), expected[idx]);
    }
}

#[test]
fn modulo_fail() {
    assert_eq!(
        ALU::modulo(Value::I64(1), Value::I64(0)).err().unwrap().message(),
        String::from("Overflow occurred when performing modulo on i64s.")
    );
    assert_eq!(
        ALU::modulo(Value::I64(1), Value::F64(2.0)).err().unwrap().message(),
        String::from("Cannot perform modulo between values of type 'i64' and 'f64'.")
    );
    assert_eq!(
        ALU::modulo(Value::F64(1.0), Value::F64(2.0)).err().unwrap().message(),
        String::from("Cannot perform modulo between values of type 'f64' and 'f64'.")
    );
}

#[test]
fn division_float_edge_cases() {
    assert_eq!(
        ALU::division(Value::F64(1.0), Value::F64(0.0)).err().unwrap().message(),
        String::from("Invalid result when performing division on f64s.")
    );
    assert_eq!(
        ALU::division(Value::F64(0.0), Value::F64(0.0)).err().unwrap().message(),
        String::from("Invalid result when performing division on f64s.")
    );
}
