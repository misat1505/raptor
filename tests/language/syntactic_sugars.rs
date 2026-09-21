use std::io::BufReader;

use crate::common::assert_same_output;

#[test]
fn if_matches_binds_inner_value() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

let suspended = AccountStatus::Suspended("too many failed logins");

if (suspended matches AccountStatus::Suspended(reason)) {
    println("suspended: " + reason);
}

let deleted = AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 });

if (deleted matches AccountStatus::Deleted(timestamp)) {
    println(timestamp.day as str + "-" + timestamp.month as str + "-" + timestamp.year as str);
}
        "#
        .as_bytes(),
    );

    assert_same_output(text, "suspended: too many failed logins\n17-9-2026\n");
}

#[test]
fn if_matches_with_else() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

let statuses = [
    AccountStatus::Active,
    AccountStatus::Suspended("spam"),
    AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 })
];

for (let i = 0; i < vector_size(&statuses); i += 1) {
    if (statuses[i] matches AccountStatus::Active) {
        println("active");
    } else {
        println("not active");
    }
}
        "#
        .as_bytes(),
    );

    assert_same_output(text, "active\nnot active\nnot active\n");
}

#[test]
fn if_matches_without_else_skips_block() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

let status = AccountStatus::Active;

if (status matches AccountStatus::Suspended(reason)) {
    println("should not print: " + reason);
}

println("done");
        "#
        .as_bytes(),
    );

    assert_same_output(text, "done\n");
}

#[test]
fn if_not_matches_with_else() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

let statuses = [
    AccountStatus::Active,
    AccountStatus::Suspended("spam"),
    AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 })
];

for (let i = 0; i < vector_size(&statuses); i += 1) {
    if (statuses[i] not matches AccountStatus::Active) {
        println("inactive");
    } else {
        println("active");
    }
}
        "#
        .as_bytes(),
    );

    assert_same_output(text, "active\ninactive\ninactive\n");
}

#[test]
fn if_not_matches_without_else() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

let statuses = [
    AccountStatus::Active,
    AccountStatus::Suspended("spam"),
    AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 })
];

if (statuses[0] not matches AccountStatus::Deleted) {
    println("first is not deleted");
}

if (statuses[2] not matches AccountStatus::Deleted) {
    println("should not print");
}

println("done");
        "#
        .as_bytes(),
    );

    assert_same_output(text, "first is not deleted\ndone\n");
}

#[test]
fn if_matches_binding_mutates_original() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

struct Account {
    str name,
    AccountStatus status
};

let account = Account {
    name: "Charlie",
    status: AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 })
};

if (account.status matches AccountStatus::Deleted(timestamp)) {
    timestamp.day = 1;
}

match (account.status) {
    AccountStatus::Deleted(timestamp) {
        println(timestamp.day as str);
    },
    rest {}
}
        "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n");
}

#[test]
fn while_matches_sums_until_mismatch() {
    let text = BufReader::new(
        r#"
enum Slot {
    Full(i64),
    Empty
};

let slots = [
    Slot::Full(1),
    Slot::Full(2),
    Slot::Full(3),
    Slot::Empty,
    Slot::Full(99)
];

let idx = 0;
let sum = 0;

while (slots[idx] matches Slot::Full(value)) {
    sum += value;
    idx += 1;
}

println(sum as str);
println(idx as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "6\n3\n");
}

#[test]
fn while_matches_break_and_continue() {
    let text = BufReader::new(
        r#"
enum Slot {
    Full(i64),
    Empty
};

let slots = [
    Slot::Full(1),
    Slot::Full(2),
    Slot::Full(3),
    Slot::Full(4),
    Slot::Full(5),
    Slot::Empty
];

let idx = 0;
let sum = 0;

while (slots[idx] matches Slot::Full(value)) {
    idx += 1;

    if (value == 2) {
        continue;
    }

    if (value == 4) {
        break;
    }

    sum += value;
}

println(sum as str);
println(idx as str);
        "#
        .as_bytes(),
    );

    // 1 -> sum=1; 2 -> continue; 3 -> sum=4; 4 -> break
    assert_same_output(text, "4\n4\n");
}

#[test]
fn while_matches_and_not_matches_zero_iterations() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

let status = AccountStatus::Active;

while (status matches AccountStatus::Deleted(timestamp)) {
    println("should not print (matches)");
    break;
}

while (status not matches AccountStatus::Active) {
    println("should not print (not matches)");
    break;
}

println("done");
        "#
        .as_bytes(),
    );

    assert_same_output(text, "done\n");
}

#[test]
fn while_not_matches_finds_first_match() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

let statuses = [
    AccountStatus::Active,
    AccountStatus::Suspended("spam"),
    AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 })
];

let idx = 0;

while (statuses[idx] not matches AccountStatus::Deleted) {
    idx += 1;
}

println(idx as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "2\n");
}

#[test]
fn while_not_matches_with_bounds_check() {
    let text = BufReader::new(
        r#"
enum AccountStatus {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp {
    i64 day,
    i64 month,
    i64 year
};

struct Account {
    str name,
    AccountStatus status
};

let accounts = [
    Account { name: "Alice", status: AccountStatus::Active },
    Account { name: "Bob", status: AccountStatus::Suspended("too many failed logins") }
];

let idx = 0;

while (accounts[idx].status not matches AccountStatus::Deleted) {
    idx += 1;
    if (idx >= vector_size(&accounts)) {
        println("There is no deleted user.");
        break;
    }
}

println(idx as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "There is no deleted user.\n2\n");
}

#[test]
fn while_matches_nested_break_exits_only_inner_loop() {
    let text = BufReader::new(
        r#"
enum Slot {
    Full(i64),
    Empty
};

let slots = [
    Slot::Full(1),
    Slot::Full(2),
    Slot::Full(3),
    Slot::Empty
];

let start = 0;
let total = 0;

while (slots[start] matches Slot::Full(first)) {
    total += first;

    let idx = start + 1;
    while (slots[idx] matches Slot::Full(value)) {
        total += value;
        idx += 1;
    }

    start += 1;
}

println(total as str);
        "#
        .as_bytes(),
    );

    // start=0: 1 + (2+3) = 6
    // start=1: 2 + 3     = 5
    // start=2: 3 + 0     = 3
    assert_same_output(text, "14\n");
}
