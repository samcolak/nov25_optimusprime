
#![allow(while_true)]

use std::{time::{Duration, Instant}};


async fn is_prime(n: i32, factors: &Vec<i32>) -> bool {

    match n {

        0 => false,
        
        1..=3 => true,
                
        k => {

            if (k & 1) == 1 {
        
                // if we have a potential prime (_p == true...) - check if it really is a prime..

                if (((n - 1) % 6) == 0) || (((n + 1) % 6) == 0) {

                    let mut _found = false;
                    let _m = f32::sqrt(n as f32) as i32;

                    for _f in factors {
                        if _f > &_m {
                            break;
                        }
                        if n % _f == 0 {
                            _found = true;
                            break;
                        }
                    }

                    !_found

                } else {

                    false
                    
                }

            } else {

                false

            }

        }

    }

}


#[tokio::main]
async fn main() {

    let mut _n = 2;
    let mut _inc = 1;
    let mut _top = 0;

    let mut _factors: Vec<i32> = Vec::new();

    let _start = Instant::now();
    let _duration = Duration::from_secs(30);

    while true {

        if is_prime(_n, &_factors).await {
            _top = _n;
            _factors.push(_n);
        }

        _n += _inc;

        if (_n > 6) && (_inc == 1) {
            _inc = 2;
        }

        if _start.elapsed() > _duration {
            break;
        }

    };

    println!("highest prime = {_top}");

}
