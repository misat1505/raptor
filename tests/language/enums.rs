use std::io::BufReader;

use crate::common::assert_same_output;

#[test]
fn tasks_example() {
    let text = BufReader::new(
        r##"
struct Deadline {
    str date
};

fn deadline_repr(&Deadline deadline): str {
    return "Deadline { date: \"" + deadline.date + "\" }";
}

enum Task {
    InProgress(Deadline),
    Completed(str),
    Aborted
};

fn task_repr(&Task task): str {
    match (task) {
        Task::InProgress(deadline) {
            return "Task::InProgress(" + deadline_repr(&deadline) + ")";
        },
        Task::Completed(msg) {
            return "Task::Completed(\"" + msg + "\")";
        },
        Task::Aborted {
            return "Task::Aborted";
        }
    }
}

struct TaskRepository {
    Task[] tasks
};

fn task_repository_repr(&TaskRepository repo): str {
    let text = "TaskRepository { tasks: [";
    for (let i = 0; i < vector_size(&repo.tasks); i += 1) {
        if (i != 0) text += ", ";
        text += task_repr(&repo.tasks[i]);
    }
    text += "] }";
    return text;
}

let repo = TaskRepository {
    tasks: [
        Task::InProgress(Deadline { date: "today" }),
        Task::Completed("completed"),
        Task::Aborted
    ]
};

println(task_repository_repr(&repo));
    "##
        .as_bytes(),
    );

    assert_same_output(
        text,
        "TaskRepository { tasks: [Task::InProgress(Deadline { date: \"today\" }), Task::Completed(\"completed\"), Task::Aborted] }\n",
    );
}

#[test]
fn enum_unit_variants() {
    let text = BufReader::new(
        r##"
enum Status {
    Pending,
    Running,
    Finished
};

fn status_repr(&Status status): str {
    match (status) {
        Status::Pending {
            return "pending";
        },
        Status::Running {
            return "running";
        },
        Status::Finished {
            return "finished";
        }
    }
}

let status1 = Status::Pending;
println(status_repr(&status1));

let status2 = Status::Running;
println(status_repr(&status2));

let status3 = Status::Finished;
println(status_repr(&status3));
    "##
        .as_bytes(),
    );

    assert_same_output(text, "pending\nrunning\nfinished\n");
}

#[test]
fn enum_primitive_payloads() {
    let text = BufReader::new(
        r##"
enum Value {
    Number(i64),
    Text(str),
    Flag(bool)
};

fn value_repr(&Value value): str {
    match (value) {
        Value::Number(n) {
            return "number=" + n as str;
        },
        Value::Text(text) {
            return "text=" + text;
        },
        Value::Flag(flag) {
            if (flag) {
                return "flag=true";
            }

            return "flag=false";
        }
    }
}

let val1 = Value::Number(42);
println(value_repr(&val1));

let val2 = Value::Text("hello");
println(value_repr(&val2));

let val3 = Value::Flag(true);
println(value_repr(&val3));

let val4 = Value::Flag(false);
println(value_repr(&val4));
    "##
        .as_bytes(),
    );

    assert_same_output(text, "number=42\ntext=hello\nflag=true\nflag=false\n");
}

#[test]
fn enum_struct_payload() {
    let text = BufReader::new(
        r##"
struct User {
    str name,
    i64 age
};

enum Response {
    Ok(User),
    NotFound,
    Error(str)
};

fn response_repr(&Response response): str {
    match (response) {
        Response::Ok(user) {
            return "OK: " + user.name + ", " + user.age as str;
        },
        Response::NotFound {
            return "NOT_FOUND";
        },
        Response::Error(message) {
            return "ERROR: " + message;
        }
    }
}

let res1 = Response::Ok(User {
    name: "Alice",
    age: 30
});
println(response_repr(&res1));

let res2 = Response::NotFound;
println(response_repr(&res2));

let res3 = Response::Error("database failure");
println(response_repr(&res3));
    "##
        .as_bytes(),
    );

    assert_same_output(text, "OK: Alice, 30\nNOT_FOUND\nERROR: database failure\n");
}

#[test]
fn enum_vector() {
    let text = BufReader::new(
        r##"
enum Token {
    Number(i64),
    Word(str),
    End
};

fn token_repr(&Token token): str {
    match (token) {
        Token::Number(value) {
            return "N" + value as str;
        },
        Token::Word(value) {
            return "W" + value;
        },
        Token::End {
            return "E";
        }
    }
}

let tokens: Token[] = [
    Token::Number(10),
    Token::Word("hello"),
    Token::Number(20),
    Token::Word("world"),
    Token::End
];

for (let i = 0; i < vector_size(&tokens); i += 1) {
    println(token_repr(&tokens[i]));
}
    "##
        .as_bytes(),
    );

    assert_same_output(text, "N10\nWhello\nN20\nWworld\nE\n");
}

#[test]
fn enum_string_payload_lifetime() {
    let text = BufReader::new(
        r##"
enum Message {
    Text(str),
    Empty
};

fn make_message(str text): Message {
    return Message::Text(text);
}

fn message_text(&Message message): str {
    match (message) {
        Message::Text(text) {
            return text;
        },
        Message::Empty {
            return "empty";
        }
    }
}

let message = make_message("hello world");

println(message_text(&message));
println(message_text(&message));
    "##
        .as_bytes(),
    );

    assert_same_output(text, "hello world\nhello world\n");
}

#[test]
fn enum_nested_payloads() {
    let text = BufReader::new(
        r##"
enum Result {
    Success(str),
    Failure(str)
};

enum Operation {
    Done(Result),
    Skipped
};

fn operation_repr(&Operation operation): str {
    match (operation) {
        Operation::Done(result) {
            match (result) {
                Result::Success(value) {
                    return "success: " + value;
                },
                Result::Failure(error) {
                    return "failure: " + error;
                }
            }
        },
        Operation::Skipped {
            return "skipped";
        }
    }
}

let op1 = Operation::Done(
    Result::Success("42")
);
println(operation_repr(&op1));

let op2 = Operation::Done(
    Result::Failure("invalid input")
);
println(operation_repr(&op2));

let op3 = Operation::Skipped;
println(operation_repr(&op3));
    "##
        .as_bytes(),
    );

    assert_same_output(text, "success: 42\nfailure: invalid input\nskipped\n");
}

#[test]
fn enum_with_vector_payload() {
    let text = BufReader::new(
        r##"
enum Data {
    Numbers(i64[]),
    Text(str)
};

fn data_sum(&Data data): i64 {
    match (data) {
        Data::Numbers(numbers) {
            let sum: i64 = 0;

            for (let i = 0; i < vector_size(&numbers); i += 1) {
                sum += numbers[i];
            }

            return sum;
        },
        Data::Text {
            return 0;
        }
    }
}

let data = Data::Numbers([10, 20, 30, 40]);

println(data_sum(&data) as str);
    "##
        .as_bytes(),
    );

    assert_same_output(text, "100\n");
}

#[test]
fn enum_repeated_creation_and_replacement() {
    let text = BufReader::new(
        r##"
enum State {
    Text(str),
    Number(i64),
    Empty
};

fn state_repr(&State state): str {
    match (state) {
        State::Text(text) {
            return "text:" + text;
        },
        State::Number(number) {
            return "number:" + number as str;
        },
        State::Empty {
            return "empty";
        }
    }
}

let state = State::Text("first");
println(state_repr(&state));

state = State::Text("second");
println(state_repr(&state));

state = State::Number(123);
println(state_repr(&state));

state = State::Text("third");
println(state_repr(&state));

state = State::Empty;
println(state_repr(&state));
    "##
        .as_bytes(),
    );

    assert_same_output(text, "text:first\ntext:second\nnumber:123\ntext:third\nempty\n");
}

#[test]
fn enum_struct_vector_nested() {
    let text = BufReader::new(
        r##"
struct Item {
    str name
};

enum Result {
    Item(Item),
    Empty
};

struct Container {
    Result[] results
};

fn item_name(&Result result): str {
    match (result) {
        Result::Item(item) {
            return item.name;
        },
        Result::Empty {
            return "empty";
        }
    }
}

let container = Container {
    results: [
        Result::Item(Item { name: "one" }),
        Result::Empty,
        Result::Item(Item { name: "two" }),
        Result::Item(Item { name: "three" })
    ]
};

for (let i = 0; i < vector_size(&container.results); i += 1) {
    println(item_name(&container.results[i]));
}
    "##
        .as_bytes(),
    );

    assert_same_output(text, "one\nempty\ntwo\nthree\n");
}

#[test]
fn enum_return_values() {
    let text = BufReader::new(
        r##"
enum Result {
    Success(str),
    Failure(str)
};

fn get_result(bool success): Result {
    if (success) {
        return Result::Success("everything is fine");
    }

    return Result::Failure("something went wrong");
}

fn print_result(&Result result): void {
    match (result) {
        Result::Success(message) {
            println("SUCCESS: " + message);
        },
        Result::Failure(message) {
            println("FAILURE: " + message);
        }
    }
}

let first = get_result(true);
let second = get_result(false);

print_result(&first);
print_result(&second);
    "##
        .as_bytes(),
    );

    assert_same_output(text, "SUCCESS: everything is fine\nFAILURE: something went wrong\n");
}

#[test]
fn enum_empty_vector_payload() {
    let text = BufReader::new(
        r##"
enum Data {
    Numbers(i64[]),
    Text(str)
};

fn numbers_size(&Data data): i64 {
    match (data) {
        Data::Numbers(numbers) {
            return vector_size(&numbers);
        },
        Data::Text {
            return 0;
        }
    }
}

let data = Data::Numbers([]);

println(numbers_size(&data) as str);
    "##
        .as_bytes(),
    );

    assert_same_output(text, "0\n");
}

#[test]
fn enum_empty_struct_vector_payload() {
    let text = BufReader::new(
        r##"
struct Data {
    str data
};

enum MyEnum {
    Empty,
    Complex(Data[])
};

let value = MyEnum::Complex([]);

match (value) {
    MyEnum::Empty {
        println("empty");
    },
    MyEnum::Complex(data) {
        println(vector_size(&data) as str);
    }
}
    "##
        .as_bytes(),
    );

    assert_same_output(text, "0\n");
}

#[test]
fn enum_empty_vector_payload_with_struct_variant() {
    let text = BufReader::new(
        r##"
struct Data {
    str data
};

enum MyEnum {
    Empty,
    Numbers(i64[]),
    Complex(Data[])
};

fn enum_repr(&MyEnum value): str {
    match (value) {
        MyEnum::Empty {
            return "empty";
        },
        MyEnum::Numbers(numbers) {
            return "numbers:" + vector_size(&numbers) as str;
        },
        MyEnum::Complex(data) {
            return "complex:" + vector_size(&data) as str;
        }
    }
}

let a = MyEnum::Numbers([]);
let b = MyEnum::Complex([]);

println(enum_repr(&a));
println(enum_repr(&b));
    "##
        .as_bytes(),
    );

    assert_same_output(text, "numbers:0\ncomplex:0\n");
}

#[test]
fn enum_empty_struct_vector_passed_through_function() {
    let text = BufReader::new(
        r##"
struct Data {
    str data
};

enum MyEnum {
    Complex(Data[])
};

fn create_empty(): MyEnum {
    return MyEnum::Complex([]);
}

fn get_size(&MyEnum value): i64 {
    match (value) {
        MyEnum::Complex(data) {
            return vector_size(&data);
        }
    }
}

let value = create_empty();

println(get_size(&value) as str);
println(get_size(&value) as str);
    "##
        .as_bytes(),
    );

    assert_same_output(text, "0\n0\n");
}

#[test]
fn enum_vector_payload_reference_mutation() {
    let text = BufReader::new(
        r##"
enum Data {
    Numbers(i64[])
};

fn add_number(&Data data, i64 value): void {
    match (data) {
        Data::Numbers(numbers) {
            vector_push(&numbers, value);
        }
    }
}

fn print_numbers(&Data data): void {
    match (data) {
        Data::Numbers(numbers) {
            for (let i = 0; i < vector_size(&numbers); i += 1) {
                println(numbers[i] as str);
            }
        }
    }
}

let numbers = [10, 20];

let data = Data::Numbers(numbers);

add_number(&data, 30);
print_numbers(&data);
    "##
        .as_bytes(),
    );

    assert_same_output(text, "10\n20\n30\n");
}

#[test]
fn enum_vector_payload_reference_access() {
    let text = BufReader::new(
        r##"
enum Data {
    Numbers(i64[])
};

fn modify(&Data data): void {
    match (data) {
        Data::Numbers(numbers) {
            numbers[0] = 999;
        }
    }
}

fn first(&Data data): i64 {
    match (data) {
        Data::Numbers(numbers) {
            return numbers[0];
        }
    }
}

let numbers = [10, 20, 30];
let data = Data::Numbers(numbers);

println(first(&data) as str);

modify(&data);

println(first(&data) as str);
    "##
        .as_bytes(),
    );

    assert_same_output(text, "10\n999\n");
}

#[test]
fn enum_payload_preserves_reference() {
    let text = BufReader::new(
        r##"
struct Data {
    str value
};

enum Wrapper {
    Data(Data)
};

fn modify(&Wrapper wrapper): void {
    match (wrapper) {
        Wrapper::Data(data) {
            data.value = "modified";
        }
    }
}

let original = Data {
    value: "original"
};

let wrapper = Wrapper::Data(original);

modify(&wrapper);

match (wrapper) {
    Wrapper::Data(data) {
        println(data.value);
    }
}
    "##
        .as_bytes(),
    );

    assert_same_output(text, "modified\n");
}
