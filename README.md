# Rust Prime Number Sieve

This project is an exercise in learning more about writing Rust, by
implementing a prime number sieve.

## Background

A prime number sieve is a method of improving the runtime performance of a
program that finds new prime numbers. A number `n` is prime if and only if
it is not evenly divisible by any of the numbers `2..n-1`. However, for large
values this becomes very inefficient.

A prime number sieve instead only stores the previously identified primes,
and checks each successive value against the elements in this collection.
This reduces the number of calculations required to check the primality of
a new value.

## Lessons Learned

This project taught me about some interesting features in Rust:
*  Traits (namely, the Iterator trait)
*  Options, Results, and pattern matching
*  The std::time module

### Traits

After defining a struct, we can make it iterable by implementing a trait. This
is done by implementing some code that looks like the following.

```rust
impl Iterator for Foo {
    type Item = Bar;

    fn next(&mut self) -> Option<Bar> {
        // ...
    }
}
```

The neat part about this is that it will then allow us to traverse the results
using methods like `take(..)`, `map(..)`, `filter(..)`, and so on within a
`for ... in ...` loop.

## Options, Results, and Pattern Matching

Once I had a basic sieve built, I was curious to see if I could time the
duration of the calculation process. This meant adding a dependency on the
`std::time` module, and adjusting the type of object returned by the iterator.

Once I had added a new `SieveResult` struct to the program, I needed to use
the `std::time::SystemTime.elapsed()` method to find how much time had elapsed
while searching for a new prime number. This presented a minor problem
(read: chance to learn something new), as this method returns a `Result` object.

Assuming the duration variable was an `Option(u32)`, I could either assign the
timer duration, or `None` based on whether the `elapsed()` method returned an
`Ok` or an `Err`. This is done using code like this:

```rust
let ns:Option<u32> = match start_time.elapsed() {
    Ok(elapsed) => Some(elapsed.subsec_nanos()),
    Err(_) => None,
};
```

In the main function, we can then destructure the result and use a `match`
statement to invoke `println!` differently, depending on whether or not
the timer worked correctly.

```rust
let SieveResult {prime: p, duration_ns: ns} = res;
match ns {
    Some(t) => println!("{0} is prime: found in {1}ns", p, t),
    None => println!("{} is prime: Error while calculating timing!", p),
}
```
