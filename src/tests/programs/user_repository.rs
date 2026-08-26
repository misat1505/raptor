use std::io::BufReader;

use crate::tests::common::helpers::assert_same_output;

#[test]
fn user_repository() {
    let text = BufReader::new(
        r##"
struct User {
    i64 id,
    str name
};

fn user_stringify(&User user): str {
    return "User { id: " + user.id as str + ", name: \"" + user.name + "\" }"; 
}

struct UserRepository {
    User[] users
};

fn user_repository_create_user(&UserRepository repo, User user): void {
    vector_push(&repo.users, user);
}

fn user_repository_stringify(&UserRepository repo): str {
    let text = "UserRepository { users: [ ";

    for (let i = 0; i < vector_size(&repo.users); i += 1) {
        if (i != 0) text += ", ";
        text += user_stringify(&repo.users[i]);
    }

    text += " ] }";

    return text;
}

let repo = UserRepository { users: [] };

let name = "Elon";
let user = User { id: 420, name };

println(user_repository_stringify(&repo));

user_repository_create_user(&repo, user);

println(user_repository_stringify(&repo));
    "##
        .as_bytes(),
    );

    assert_same_output(
        text,
        "UserRepository { users: [  ] }\nUserRepository { users: [ User { id: 420, name: \"Elon\" } ] }",
    );
}
