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
        "UserRepository { users: [  ] }\nUserRepository { users: [ User { id: 420, name: \"Elon\" } ] }\n",
    );
}

#[test]
fn user_repository2() {
    let text = BufReader::new(
        r##"
struct User {
    u64 id,
    str name
};

fn user_init(u64 id, str name): User {
    return User { id, name };
}

fn user_repr(&User user): str {
    let text = "User { id: " + user.id as str + ", name: \"" + user.name + "\" }";
    return text;
}

struct UserRepository {
    User[] users
};

fn user_repository_init(): UserRepository {
    return UserRepository { users: [] };
}

fn user_repository_add_user(&UserRepository repo, User user): void {
    vector_push(&repo.users, user);
}

fn user_repository_repr(&UserRepository repo): str {
    let is_empty = vector_size(&repo.users) == 0;
    let text = "UserRepository { users: [";
    if (!is_empty) text += " ";

    for (let i = 0; i < vector_size(&repo.users); i += 1) {
        if (i != 0) text += ", ";
        text += user_repr(&repo.users[i]);
    }

    if (!is_empty) text += " ";
    text += "] }";

    return text;
}

let repo = UserRepository { users: [] };
println(user_repository_repr(&repo));

let users_reference = repo.users;

let user1 = user_init(1 as u64, "Elon");

println(user_repr(&user1));

user_repository_add_user(&repo, user1);
println(user_repository_repr(&repo));

vector_push(&users_reference, User { id: 2 as u64, name: "Fred" });
println(user_repository_repr(&repo));
    "##
        .as_bytes(),
    );

    assert_same_output(
        text,
        "UserRepository { users: [] }\nUser { id: 1, name: \"Elon\" }\nUserRepository { users: [ User { id: 1, name: \"Elon\" } ] }\nUserRepository { users: [ User { id: 1, name: \"Elon\" }, User { id: 2, name: \"Fred\" } ] }\n",
    );
}

#[test]
fn users_and_posts() {
    let text = BufReader::new(
        r##"
struct Description {
  u64 version,
  str text
};

struct Post {
  u64 id,
  Description description
};

struct User {
  u64 id,
  str name,
  Post[][] posts
};

let description = Description {
  version: 1 as u64,
  text: "description"
};

let post = Post {
  id: 123 as u64,
  description: description
};

let user = User {
  id: 2137 as u64,
  name: "Elon",
  posts: [[post]]
};

fn user_get_user(&User user): User {
  return user;
}

fn user_get_user2(&User[] user): User {
  return user[0];
}

fn user_get_user_array(&User user): User[] {
  return [user];
}

fn user_get_user_array2(&User[] user): User[] {
  return [user[0]];
}

let arr = [user];
user_get_user_array(&arr[0]);
user_get_user_array2(&arr);

println(user.name);
println(user.posts[0][0].id as str);
println(user.posts[0][0].description.text);

user.posts[0][0].description.text = "new description";
println(user.posts[0][0].description.text);
    "##
        .as_bytes(),
    );

    assert_same_output(text, "Elon\n123\ndescription\nnew description\n");
}
