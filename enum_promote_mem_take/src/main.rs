use std::mem;

#[derive(Debug)]
enum User{
    Reader {name :String},
    Writer {name :String},
    Admin {name: String}
}

fn promote(u: &mut User) {
    use User::*;

    *u = match u {
        Reader {name} => Writer {name: mem::take(name)},
        Writer {name} => Admin {name: mem::take(name)},
        Admin {name: _} => return,
    }
}

fn kuku(v: &mut Vec<i32>) {
    let w: &[i32] = &[1, 2];
    *v = w.to_owned();
    println!("{:?}", w);
}

fn main() {
    let mut user = User::Reader {name: "John".to_owned()};
    println!("{:?}", user);

    promote(&mut user);
    println!("{:?}", user);

    promote(&mut user);
    println!("{:?}", user);

    let mut v = vec![1, 2, 3];
    kuku(&mut v);
    println!("{:?}", v);
}
