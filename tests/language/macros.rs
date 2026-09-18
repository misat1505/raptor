use std::io::BufReader;

use crate::common::assert_same_output;

#[test]
fn debug_macro() {
    let text = BufReader::new(
        r#"
enum AccountStatus derives Debug {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp derives Debug {
    i64 day,
    i64 month,
    i64 year
};

enum Role derives Debug {
    Admin,
    Moderator,
    User
};

struct User derives Debug {
    str name,
    AccountStatus status,
    Role role
};

fn pretty_timestamp(&Timestamp timestamp): str {
    let day = timestamp.day as str;
    if (timestamp.day < 10) day = "0" + timestamp.day as str;

    let month = timestamp.month as str;
    if (timestamp.month < 10) month = "0" + timestamp.month as str;

    return day as str + "-" + month as str + "-" + timestamp.year as str;
}

let users = [
    User {
        name: "Alice",
        status: AccountStatus::Active,
        role: Role::Admin
    },
    User {
        name: "Bob",
        status: AccountStatus::Suspended("too many failed logins"),
        role: Role::User
    },
    User {
        name: "Charlie",
        status: AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 }),
        role: Role::Moderator
    }
];

let idx = 2;
match (users[idx].status) {
    AccountStatus::Deleted(timestamp) { println(pretty_timestamp(&timestamp)); },
    rest println(account_status_debug(&users[idx].status) + " doesn't contain data of type 'Timestamp'.");
}

for (let i = 0; i < vector_size(&users); i += 1) {
    println(user_debug(&users[i]));
}
        "#
        .as_bytes(),
    );

    assert_same_output(text, "17-09-2026\nUser { name: \"Alice\", status: AccountStatus::Active, role: Role::Admin }\nUser { name: \"Bob\", status: AccountStatus::Suspended(\"too many failed logins\"), role: Role::User }\nUser { name: \"Charlie\", status: AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 }), role: Role::Moderator }\n");
}

#[test]
fn json_macro() {
    let text = BufReader::new(
        r#"
enum AccountStatus derives Json {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp derives Json {
    i64 day,
    i64 month,
    i64 year
};

enum Role derives Json {
    Admin,
    Moderator,
    User
};

struct User derives Json {
    str name,
    AccountStatus status,
    Role role
};

struct UserRepository derives Json {
    User[] users
};

let users = [
    User {
        name: "Alice",
        status: AccountStatus::Active,
        role: Role::Admin
    },
    User {
        name: "Bob",
        status: AccountStatus::Suspended("too many failed logins"),
        role: Role::User
    },
    User {
        name: "Charlie",
        status: AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 }),
        role: Role::Moderator
    }
];


let user_repository = UserRepository { users };

println(user_repository_json_encode(&user_repository));
        "#
        .as_bytes(),
    );

    assert_same_output(text, "{\"users\":[{\"name\":\"Alice\",\"status\":\"Active\",\"role\":\"Admin\"},{\"name\":\"Bob\",\"status\":{\"Suspended\":\"too many failed logins\"},\"role\":\"User\"},{\"name\":\"Charlie\",\"status\":{\"Deleted\":{\"day\":17,\"month\":9,\"year\":2026}},\"role\":\"Moderator\"}]}\n");
}
