fn main() {


    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");
    println!("The value of expression is {}", {
        let x = 3;
        x + 1
    });

    println!("Fibonacci 10 is {}", fib(10));

    fn fib2(n: i32) -> i32 {
        if n <= 2 {
            return 1;
        }
        fib(n - 1) + fib(n - 2)
    }
    println!("Fibonacci 10 is {}", fib2(10));

    let fib3 = fib;
    println!("Fibonacci 10 is {}", fib3(10));
}

fn fib(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    }

    // return fib(n-1) + fib(n-2);
    fib(n - 1) + fib(n - 2)
}
