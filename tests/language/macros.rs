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
    Role role,
    i64[] scores
};

struct UserRepository derives Debug {
    User[] users
};

let users = [
    User {
        name: "Alice",
        status: AccountStatus::Active,
        role: Role::Admin,
        scores: [10, 20, 30]
    },
    User {
        name: "Bob",
        status: AccountStatus::Suspended("too many failed logins"),
        role: Role::User,
        scores: [40, 50]
    },
    User {
        name: "Charlie",
        status: AccountStatus::Deleted(Timestamp {
            day: 17,
            month: 9,
            year: 2026
        }),
        role: Role::Moderator,
        scores: [60, 70, 80, 90]
    }
];

let user_repository = UserRepository { users };

match (user_repository.users[2].status) {
    AccountStatus::Deleted(timestamp) {
        println(
            timestamp.day as str
            + "-"
            + timestamp.month as str
            + "-"
            + timestamp.year as str
        );
    },
    rest {}
}

for (let i = 0; i < vector_size(&user_repository.users); i += 1) {
    println(user_debug(&user_repository.users[i]));
}
        "#
        .as_bytes(),
    );

    assert_same_output(
        text,
        "17-9-2026\nUser { name: \"Alice\", status: AccountStatus::Active, role: Role::Admin, scores: [10, 20, 30] }\nUser { name: \"Bob\", status: AccountStatus::Suspended(\"too many failed logins\"), role: Role::User, scores: [40, 50] }\nUser { name: \"Charlie\", status: AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 }), role: Role::Moderator, scores: [60, 70, 80, 90] }\n",
    );
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
    Role role,
    i64[] scores
};

struct UserRepository derives Json {
    User[] users
};

let users = [
    User {
        name: "Alice",
        status: AccountStatus::Active,
        role: Role::Admin,
        scores: [10, 20, 30]
    },
    User {
        name: "Bob",
        status: AccountStatus::Suspended("too many failed logins"),
        role: Role::User,
        scores: [40, 50]
    },
    User {
        name: "Charlie",
        status: AccountStatus::Deleted(Timestamp {
            day: 17,
            month: 9,
            year: 2026
        }),
        role: Role::Moderator,
        scores: [60, 70, 80, 90]
    }
];

let user_repository = UserRepository { users };

println(user_repository_json_encode(&user_repository));
        "#
        .as_bytes(),
    );

    assert_same_output(
        text,
        "{\"users\":[{\"name\":\"Alice\",\"status\":\"Active\",\"role\":\"Admin\",\"scores\":[10,20,30]},{\"name\":\"Bob\",\"status\":{\"Suspended\":\"too many failed logins\"},\"role\":\"User\",\"scores\":[40,50]},{\"name\":\"Charlie\",\"status\":{\"Deleted\":{\"day\":17,\"month\":9,\"year\":2026}},\"role\":\"Moderator\",\"scores\":[60,70,80,90]}]}\n",
    );
}

#[test]
fn clone_macro() {
    let text = BufReader::new(
        r#"
enum AccountStatus derives Debug, Clone {
    Active,
    Suspended(str),
    Deleted(Timestamp)
};

struct Timestamp derives Debug, Clone {
    i64 day,
    i64 month,
    i64 year
};

enum Role derives Debug, Clone {
    Admin,
    Moderator,
    User
};

struct User derives Debug, Clone {
    str name,
    AccountStatus status,
    Role role,
    i64[] scores
};

struct UserRepository derives Debug, Clone {
    User[] users
};

let original = UserRepository {
    users: [
        User {
            name: "Alice",
            status: AccountStatus::Active,
            role: Role::Admin,
            scores: [10, 20, 30]
        },
        User {
            name: "Bob",
            status: AccountStatus::Suspended("too many failed logins"),
            role: Role::User,
            scores: [40, 50]
        },
        User {
            name: "Charlie",
            status: AccountStatus::Deleted(Timestamp {
                day: 17,
                month: 9,
                year: 2026
            }),
            role: Role::Moderator,
            scores: [60, 70, 80, 90]
        }
    ]
};

let cloned = user_repository_clone(&original);

original.users[0].name = "Changed";
original.users[0].scores[0] = 999;

match (original.users[2].status) {
    AccountStatus::Deleted(timestamp) {
        timestamp.day = 1;
    },
    rest {}
}

original.users[2].role = Role::Admin;

println(user_repository_debug(&original));
println(user_repository_debug(&cloned));
        "#
        .as_bytes(),
    );

    assert_same_output(
        text,
        "UserRepository { users: [User { name: \"Changed\", status: AccountStatus::Active, role: Role::Admin, scores: [999, 20, 30] }, User { name: \"Bob\", status: AccountStatus::Suspended(\"too many failed logins\"), role: Role::User, scores: [40, 50] }, User { name: \"Charlie\", status: AccountStatus::Deleted(Timestamp { day: 1, month: 9, year: 2026 }), role: Role::Admin, scores: [60, 70, 80, 90] }] }\nUserRepository { users: [User { name: \"Alice\", status: AccountStatus::Active, role: Role::Admin, scores: [10, 20, 30] }, User { name: \"Bob\", status: AccountStatus::Suspended(\"too many failed logins\"), role: Role::User, scores: [40, 50] }, User { name: \"Charlie\", status: AccountStatus::Deleted(Timestamp { day: 17, month: 9, year: 2026 }), role: Role::Moderator, scores: [60, 70, 80, 90] }] }\n",
    );
}
