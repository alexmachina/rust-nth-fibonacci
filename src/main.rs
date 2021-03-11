use std::io;

fn main() {
    loop {
        println!("What fibonacci number?");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Oops");

        let mut n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut fib_a = 0;
        let mut fib_b = 1;

        while n != 0 {
            let fib_c = fib_a + fib_b;
            fib_a = fib_b;
            fib_b = fib_c;
            n -= 1;
        }

        println!("{}", fib_b);
    }
}
