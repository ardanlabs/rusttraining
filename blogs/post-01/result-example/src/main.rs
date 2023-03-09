struct User {
    name: String,
    age: isize,
}

struct Error {
    msg: String,
}

fn query_user(worked: bool) -> Result<User, Error> {
    if worked {
        return Ok(User {
            name: "Bill".to_string(),
            age: 53,
        });
    }

    return Err(Error {
        msg: "unable to fetch user".to_string(),
    });
}

fn main() {
    match query_user(true) {
        Ok(usr) => println!("Name:{0} Age:{1}", usr.name, usr.age),
        Err(err) => println!("{0}", err.msg),
    }

    match query_user(false) {
        Ok(usr) => println!("Name:{0} Age:{1}", usr.name, usr.age),
        Err(err) => println!("{0}", err.msg),
    }
}
