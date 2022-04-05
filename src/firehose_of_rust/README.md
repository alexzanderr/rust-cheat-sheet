# Firehose of Rust

## First of all, credits
credit goes to: `Jack O'Connor`

materials included in this cheat sheet
- [`slides`](https://jacko.io/firehose_of_rust)
- [`presentation`](https://www.youtube.com/watch?v=FSyfZVuD32Y)



# A Dangling Pointer in Rust
```rust
let my_int_ptr: &i32;
{
    let my_int: i32 = 5;
    my_int_ptr = &my_int;
}
dbg!(*my_int_ptr);
```

after run
```shell
error[E0597]: `my_int` does not live long enough
 --> examples/firehose_dangling_pointer.rs:7:22
  |
7 |         my_int_ptr = &my_int;
  |                      ^^^^^^^ borrowed value does not live long enough
8 |     }
  |     - `my_int` dropped here while still borrowed
9 |     dbg!(*my_int_ptr);
  |          ----------- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `cheat-sheet` due to previous error
```

solution to fix this:
```shell
you dont make to things that dont live long enough
```

# A borrowing view
```rust
let my_string: String =
    "abcdefghijklmnopqrstuvwxy".to_string();
let my_string_view: &str = (my_string + "z").as_str();
dbg!(my_string_view);
```
after run
```shell
error[E0716]: temporary value dropped while borrowed
 --> src/main.rs:5:9
  |
5 |         (my_string + "z").as_str();
  |         ^^^^^^^^^^^^^^^^^         - temporary value is freed at the end of this statement
  |         |
  |         creates a temporary which is freed while still in use
6 |     dbg!(my_string_view);
  |          -------------- borrow later used here
  |
  = note: consider using a `let` binding to create a longer lived value
```

solution to fix this: you put the temp value into a variable
```rust
let my_string: String =
    "abcdefghijklmnopqrstuvwxy".to_string();
let temp_value_not_temp_anymore = my_string + "z";
let my_string_view: &str = temp_value_not_temp_anymore.as_str();
// what if i drop `temp_value_not_temp_anymore` here
// drop(temp_value_not_temp_anymore);
dbg!(my_string_view);
```

# A long lived container
```rust
let mut my_vector: Vec<&str> = Vec::new();
{
    let my_string = "hello world".to_string();
    my_vector.push(&my_string);
}
dbg!(my_vector);
```
after run
```shell
error[E0597]: `my_string` does not live long enough
 --> src/main.rs:5:24
  |
5 |         my_vector.push(&my_string);
  |                        ^^^^^^^^^^ borrowed value does not live long enough
6 |     }
  |     - `my_string` dropped here while still borrowed
7 |     dbg!(my_vector);
  |          --------- borrow later used here
```

# An invalid function
```rust
fn my_push_back(v: &mut Vec<&str>, s: &str) {
    v.push(s);
}

fn main() {
    let mut my_vector: Vec<&str> = Vec::new();
    {
        let my_string = "hello world".to_string();
        my_push_back(&mut my_vector, &my_string);
    }
    dbg!(my_vector);
}
```

after run
```shell
error[E0623]: lifetime mismatch
 --> src/main.rs:2:12
  |
1 | fn my_push_back(v: &mut Vec<&str>, s: &str) {
  |                             ----      ---- these two types are declared with different lifetimes...
2 |     v.push(s);
  |            ^ ...but data from `s` flows into `v` here
```
rust analyses the function first. it assumes the s reference doesnt live long enough to be put in the `v` container and that is very corect. it could really happen, as presented in the above code.

solution to fix this: add explicit lifetimes
```rust
fn my_push_back<'a>(v: &mut Vec<&'a str>, s: &'a str) {
    v.push(s);
}

fn main() {
    let mut my_vector: Vec<&str> = Vec::new();
    {
        let my_string = "hello world".to_string();
        my_push_back(&mut my_vector, &my_string);
    }
    dbg!(my_vector);
}
```

NOTE: this doesnt fix the code, these explicit lifetimes only fix the function
the error still remains
```shell
error[E0597]: `my_string` does not live long enough
  --> src/main.rs:9:38
   |
9  |         my_push_back(&mut my_vector, &my_string);
   |                                      ^^^^^^^^^^ borrowed value does not live long enough
10 |     }
   |     - `my_string` dropped here while still borrowed
11 |     dbg!(my_vector);
   |          --------- borrow later used here
```
same thing as first example with pointer, cannot have reference to something that was deallocated from the memory

explanation: the elements in the vector and new element that will arive in the vector `must` have the same lifetime, meaning: they must have the same life span

think of 2 people with different ages. for example, 4 this to work, both people must die at the same time or have both 70 years of lifespan for example.

you cannot have 2 people with one dying sooner than the other. same principle to references in rust


# Mutable aliasing
```rust
let mut my_int = 5;
let reference1 = &mut my_int;
let reference2 = &mut my_int;
*reference1 += 1;
*reference2 += 1;
assert_eq!(my_int, 7);
```

after run
```shell
error[E0499]: cannot borrow `my_int` as mutable more than once at a time
 --> src/main.rs:4:22
  |
3 |     let reference1 = &mut my_int;
  |                      ----------- first mutable borrow occurs here
4 |     let reference2 = &mut my_int;
  |                      ^^^^^^^^^^^ second mutable borrow occurs here
5 |     *reference1 += 1;
  |     ---------------- first borrow later used here
```

# Multiple references into an array
```rust
let mut char_array: [char; 2] = ['a', 'b'];
let first_element = &mut char_array[0];
let second_element = &char_array[1];
*first_element = *second_element;
assert_eq!(char_array[0], 'b');
```
after run
```shell
error[E0502]: cannot borrow `char_array[_]` as immutable because it is also borrowed as mutable
 --> src/main.rs:6:26
  |
5 |     let first_element = &mut char_array[0];
  |                         ------------------ mutable borrow occurs here
6 |     let second_element = &char_array[1];
  |                          ^^^^^^^^^^^^^^ immutable borrow occurs here
7 |     *first_element = *second_element;
  |     -------------------------------- mutable borrow later used here
```

solution to this:
1. `split_at_mut`
```rust
let mut char_array: [char; 2] = ['a', 'b'];
let (first_slice, rest_slice) =
    char_array.split_at_mut(1);
let first_element = &mut first_slice[0];
let second_element = &rest_slice[0];
*first_element = *second_element;
assert_eq!(char_array[0], 'b');
```

2. `unsafe code`
```rust
let mut char_array: [char; 2] = ['a', 'b'];
let first_element: *mut char = &mut char_array[0];
let second_element: *const char = &char_array[1];
unsafe {
    *first_element = *second_element;
}
assert_eq!(char_array[0], 'b');
```

# Invalidating a reference by reallocating
```rust
fn push_int_twice(v: &mut Vec<i32>, n: &i32) {
    v.push(*n);
    v.push(*n);
}

fn main() {
    let mut my_vector = vec![0];
    let my_int_reference = &my_vector[0];
    push_int_twice(&mut my_vector, my_int_reference);
}
```
after run
```shell
error[E0502]: cannot borrow `my_vector` as mutable because it is also borrowed as immutable
  --> src/main.rs:12:20
   |
11 |     let my_int_reference = &my_vector[0];
   |                             --------- immutable borrow occurs here
12 |     push_int_twice(&mut my_vector, my_int_reference);
   |                    ^^^^^^^^^^^^^^  ---------------- immutable borrow later used here
   |                    |
   |                    mutable borrow occurs here
```


# Magical multi-threading
```rust
// serial for loop
let mut v: Vec<i32> = vector_of_ints();
for x in &mut v {
    *x += 1;
}


// serial for_each
let mut v: Vec<i32> = vector_of_ints();
v.iter_mut().for_each(|x| {
    *x += 1;
});


// Rayon parallel for_each
let mut v: Vec<i32> = vector_of_ints();
v.par_iter_mut().for_each(|x| {
    *x += 1;
});
```
this code is very fine, every `x` is incremented on its `thread`, fine, no data race


# Tragical multithreading
```rust
// serial for loop
let mut v: Vec<i32> = vector_of_ints();
let mut sum = 0;
for x in &mut v {
    *x += 1;
    sum += *x;
}
// very fine


// serial for_each
let mut v: Vec<i32> = vector_of_ints();
let mut sum = 0;
v.iter_mut().for_each(|x| {
    *x += 1;
    sum += *x;
});
// very fine


// Rayon parallel for_each
let mut v: Vec<i32> = vector_of_ints();
let mut sum = 0;
v.par_iter_mut().for_each(|x| {
    *x += 1;
    sum += *x;
});
// not fine
// compile error, data race
```

after run
```shell
   error[E0594]: cannot assign to `sum`, as it is a captured variable in a `Fn` closure
  --> src/main.rs:62:9
   |
62 |         sum += *x;
   |         ^^^^^^^^^ cannot assign
```

## How does Rust know? about that sum error

### Serial Iterator
```rust
fn for_each<F>(self, f: F)
where
    Self: Sized,
    F: FnMut(Self::Item),
{
    #[inline]
    fn call<T>(
        mut f: impl FnMut(T),
    ) -> impl FnMut((), T) {
        move |(), item| f(item)
    }
    self.fold((), call(f));
}
```
`Fn` is not `FnMut`, in the prev example rust expected `FnMut`

### ParallelIterator
```rust
fn for_each<OP>(self, op: OP)
where
    OP: Fn(Self::Item) + Sync + Send,
{
    for_each::for_each(self, &op)
}
```

# Synchronizing shared state


```rust
// using AtomicI32
let mut v: Vec<i32> = vector_of_ints();
let sum: AtomicI32 = AtomicI32::new(0);
v.par_iter_mut().for_each(|x| {
    *x += 1;
    sum.fetch_add(*x, Ordering::Relaxed);
});


// using Mutex<i32>
let mut v: Vec<i32> = vector_of_ints();
let sum: Mutex<i32> = Mutex::new(0);
v.par_iter_mut().for_each(|x| {
    *x += 1;
    let mut guard: MutexGuard<i32> = sum.lock().unwrap();
    *guard += *x;
});
```
this code is very fine

# Moving a string
```rust
let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
let s2 = s1; // its moved,
let mut v = Vec::new();
v.push(s2); // its moved
// here v goes out of scope, only the destructor of v is called
```
one string allocation, one string destructor, how ?
well, look at the comments

# Copying a string
```rust
let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
let s2 = s1.clone();
let mut v = Vec::new();
v.push(s2.clone());
```
three string allocations, three string destructors, because you copy that data inside the `String`

#  Accessing a moved-from object
```rust
let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
let s2 = s1;
dbg!(s1);
```
after run
```shell
error[E0382]: use of moved value: `s1`
  --> src/main.rs:26:14
   |
24 |         let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
   |             -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
25 |         let s2 = s1;
   |                  -- value moved here
26 |         dbg!(s1);
   |              ^^ value used here after move
```
s1 its gone, you cannot use it anymore


# Moving a borrowed object
```rust
let s1 = "abcde".to_string();
let my_view = s1.as_str();
let s2 = s1;
dbg!(my_view);
```
after run
```shell
error[E0505]: cannot move out of `s1` because it is borrowed
  --> src/main.rs:31:18
   |
30 |         let my_view = s1.as_str();
   |                       -- borrow of `s1` occurs here
31 |         let s2 = s1;
   |                  ^^ move out of `s1` occurs here
32 |         dbg!(my_view);
   |              ------- borrow later used here
```


# Moving a string again
```rust
let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
let s2 = s1;
let mut v = Vec::new();
v.push(s2);
let s3 = v[0];
```
after run
```shell
error[E0507]: cannot move out of index of `Vec<String>`
  --> src/main.rs:86:18
   |
86 |         let s3 = v[0];
   |                  ^^^^
   |                  |
   |                  move occurs because value has type `String`, which does not implement the `Copy` trait
   |                  help: consider borrowing here: `&v[0]`
```

### another example
```rust
fn f(s1: &mut String) {
    let s2 = *s1;
    dbg!(s2);
}

fn g() {
    let mut s1 = "foo".to_string();
    f(&mut s1);
    dbg!(s1);
}
```

```shell
error[E0507]: cannot move out of `*s1` which is behind a mutable reference
 --> src/main.rs:7:14
  |
7 |     let s2 = *s1;
  |              ^^^
  |              |
  |              move occurs because `*s1` has type `String`, which does not implement the `Copy` trait
  |              help: consider borrowing here: `&*s1`
```

## how to fix
### mem::swap
```rust
fn f(s1: &mut String) {
    let mut s2 = "".to_string();
    mem::swap(s1, &mut s2);
    dbg!(s2);
}

fn g() {
    let mut s1 = "foo".to_string();
    f(&mut s1);
    dbg!(s1);
}
```
s1 == ""
s2 == "foo"

### Option::take
```rust
fn f(s1: &mut Option<String>) {
    let s2 = s1.take().unwrap();
    dbg!(s2);
}

fn g() {
    let mut s1: Option<String> =
        Some("foo".to_string());
    f(&mut s1);
    dbg!(s1);
}
```
s1 == None
s2 == "foo"

### Vec::remove
```rust
fn f(v: &mut Vec<String>) {
    let s2 = v.remove(0);
    dbg!(s2);
}

fn g() {
    let mut v = vec![
        "foo".to_string(),
        "bar".to_string(),
        "baz".to_string(),
    ];
    f(&mut v);
    dbg!(v);
}
```
v == ["bar", "baz"]
s2 == "foo"

# The drop function
```rust
let file = File::open("/dev/null")?;
drop(file);
```

## Surprise: drop is the empty function
```rust
pub fn drop<T>(_x: T) {}
```

# Putting it all together
`Arc<Mutex<String>>`

# A mutex on the stack
```rust
let my_string: Mutex<String> = Mutex::new(String::new());
let mut thread_handles = Vec::new();
for _ in 0..10 {
    let thread_handle = thread::spawn(|| {
        let mut guard: MutexGuard<String> =
            my_string.lock().unwrap();
        guard.push_str("some characters");
    });
    thread_handles.push(thread_handle);
}
for thread_handle in thread_handles {
    thread_handle.join().unwrap();
}
```
after run
```shell
error[E0373]: closure may outlive the current function, but it borrows `my_string`, which is owned by the current function
  --> src/main.rs:9:43
   |
9  |         let thread_handle = thread::spawn(|| {
   |                                           ^^ may outlive borrowed value `my_string`
10 |             let mut guard: MutexGuard<String> =
11 |                 my_string.lock().unwrap();
   |                 --------- `my_string` is borrowed here
   |
```

## How does Rust know?

for_each
```rust
fn for_each<OP>(self, op: OP)
where
    OP: Fn(Self::Item) + Sync + Send,
{
    for_each::for_each(self, &op)
}
```

spawn
```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new()
        .spawn(f)
        .expect("failed to spawn thread")
}
```

# A mutex on the heap
```rust
let my_string: Arc<Mutex<String>> =
    Arc::new(Mutex::new(String::new()));
let mut thread_handles = Vec::new();
for _ in 0..10 {
    let arc_clone = my_string.clone();
    let thread_handle = thread::spawn(move || {
        let mut guard: MutexGuard<String> =
            arc_clone.lock().unwrap();
        guard.push_str("some characters");
    });
    thread_handles.push(thread_handle);
}
for thread_handle in thread_handles {
    thread_handle.join().unwrap();
}
```

## forgetting the mutex
```rust
let my_string: Arc<String> = Arc::new(String::new());
let mut thread_handles = Vec::new();
for _ in 0..10 {
    let mut arc_clone = my_string.clone();
    let thread_handle = thread::spawn(move || {
        arc_clone.push_str("some characters");
    });
    thread_handles.push(thread_handle);
}
for thread_handle in thread_handles {
    thread_handle.join().unwrap();
}
```
after run
```shell
error[E0596]: cannot borrow data in an `Arc` as mutable
  --> src/main.rs:46:13
   |
46 |             arc_clone.push_str("some characters");
   |             ^^^^^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<String>`
```

## Writing under a read lock
```rust
let my_string: Arc<RwLock<String>> =
    Arc::new(RwLock::new(String::new()));
let mut thread_handles = Vec::new();
for _ in 0..10 {
    let arc_clone = my_string.clone();
    let thread_handle = thread::spawn(move || {
        let mut guard: RwLockReadGuard<String> =
            arc_clone.read().unwrap();
        guard.push_str("some characters");
    });
    thread_handles.push(thread_handle);
}
for thread_handle in thread_handles {
    thread_handle.join().unwrap();
}
```
after run
```shell
error[E0596]: cannot borrow data in a dereference of `RwLockReadGuard<'_, String>` as mutable
  --> src/main.rs:65:13
   |
65 |             guard.push_str("some characters");
   |             ^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `RwLockReadGuard<'_, String>`
```

## Rust giveth and Rust taketh away
```rust
let my_string: Arc<Mutex<String>> =
    Arc::new(Mutex::new(String::new()));

let mut thread_handles = Vec::new();

for _ in 0..10 {
    let arc_clone = my_string.clone();
    let thread_handle = thread::spawn(move || {
        let mut guard = arc_clone.lock().unwrap();
        let smuggled_ptr: &mut String = &mut *guard;
        drop(guard);
        smuggled_ptr.push_str("some characters");
    });
    thread_handles.push(thread_handle);
}

for thread_handle in thread_handles {
    thread_handle.join().unwrap();
}
```
after run
```shell
error[E0505]: cannot move out of `guard` because it is borrowed
  --> src/main.rs:84:18
   |
83 |             let smuggled_ptr: &mut String = &mut *guard;
   |                                                   ----- borrow of `guard` occurs here
84 |             drop(guard);
   |                  ^^^^^ move out of `guard` occurs here
85 |             smuggled_ptr.push_str("some characters");
   |             ------------ borrow later used here
```
simple error for such a complicated thing huh ?

# Special Thanks
to `Jack O'Connor`