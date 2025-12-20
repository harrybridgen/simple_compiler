fib = [10];

x = 0;
vx = 1;
dx ::= x + vx;

fib[0] ::= 0;
fib[1] ::= 1;

loop {
    if x >= fib - 2 {
        break;
    }

    i := x;
    a := i;
    b := i + 1;

    fib[i+2] ::= fib[a] + fib[b];

    x = dx;
}

println fib[9];
