
mod fetch_rust_homepage;
mod demo4;
mod fib;

fn main() {
    // fetch_rust_homepage::fetch_rust_homepage_then_convert_to_markdown();

    // for arg in std::env::args() {
    //     println!("{}", arg);
    // }

    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    let output = &args[2];
    println!("url is {:?}, outpu is {}", url, output);

    println!("main: {:p}", main as *const ());

    demo1();

    demo2();

    demo3();

    demo4::pb::pb_ex1();
    fib::call_fib();
}

////////////////////////////////
fn demo3() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello Rust!".into()));
    process_event(&event3);

    println!("event1: {:?}, event2: {:?}", event1, event2);
}

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, message)) => println!("broadcast {:?} ", message),
    }

    if let Event::Message((uid, _, msg)) = event {
        println!("user {:?} boradcast {:?}", uid.0, msg);
    }
}

////////////////////////////////
fn demo2() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

////////////////////////////////
fn demo1() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(3, cube));
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}
