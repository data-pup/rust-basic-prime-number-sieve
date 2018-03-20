#![feature(duration_extras)]
use std::time::SystemTime;

// This struct represents the state of the prime number sieve.
struct PrimeSieve {
    curr_val: u32,
    prev_primes: Vec<u32>,
}

// This struct represents the result of a sieve calculation.
struct SieveResult {
    prime: u32,
    duration_ns: Option<u32>,
}

impl Iterator for PrimeSieve { // Make the sieve iterable.
    type Item = SieveResult;

    fn next(&mut self) -> Option<SieveResult> {
        let start_time = SystemTime::now(); // Mark the start time.
        let mut i = self.curr_val;          // Initialize a mutable counter.

        loop { // Loop until a new prime is found.
            let mut is_prime = true;
            for prev in &self.prev_primes {
                if i % prev == 0 {    // The value is not prime if divisible
                    is_prime = false; // by a previous prime value. Set the
                    break;            // flag and break the `for` loop.
                }
            }
            if !is_prime { i = i + 1; } // Increment if the value is not prime.
            else         { break; }     // Break the loop if a prime was found.
        }

        self.curr_val = i + 1;    // Increment the counter past the new prime.
        self.prev_primes.push(i); // Add the new prime value to the vector.

        // Attempt to calculate the duration of the calculation. If
        // a timer error occurred, set the duration field to None.
        let ns:Option<u32> = match start_time.elapsed() {
            Ok(elapsed) => Some(elapsed.subsec_nanos()),
            Err(_) => None,
        };

        let result = SieveResult { prime: i, duration_ns:ns };
        Some(result) // Return the sieve result.
    }
}

fn init_sieve() -> PrimeSieve {
    PrimeSieve {
        curr_val: 2,
        prev_primes: Vec::new(),
    }
}

fn main() {
    println!("Initializing prime number sieve...");
    let sieve = init_sieve();
    for res in sieve.take(100) {
        let SieveResult {prime: p, duration_ns: ns} = res;
        match ns {
            Some(t) => println!("{0} is prime: found in {1}ns", p, t),
            None => println!("{} is prime: Error while calculating timing!", p),
        }
    }
}
