
#![allow(while_true)]


fn is_prime(n: i32) -> bool {

    match n {

        0 => return false,
        
        1 | 2 => return true,
                
        k => {

            if k % 2 == 0 {
                return false;
            } else if (k % 5 == 0) && (k != 5) {
                return false
            } else {
                let _m = (n-1) / 2;
                if _m > 1 {
                    let mut _t = 3;
                    while _t <= _m {
                        if k % _t == 0 {
                            return false;
                        }
                        _t += 2; // ignore even numbers as factors...
                    }
                }
            }

        }

    };

    true

}


fn main() {

    let mut _n = 1;
    let mut _inc = 1;

    while true {

        if is_prime(_n) {
            println!("{_n} is prime");
        }

        _n += _inc;

        if (_n > 6) && (_inc == 1) {
            _inc = 2;
        }

    };

}
