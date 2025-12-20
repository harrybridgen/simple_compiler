size = 31;
grid = [size];
next = [size];

center = size / 2;

i = 0;
di ::= i + 1;

# initialize grid #
loop {
    if i >= size {
        break;
    }

    if i == center {
        grid[i] = 100;
    } else {
        grid[i] = 0;
    }

    i = di;
}

# build reactive stencil #
k = 1;
dk ::= k + 1;

loop {
    if k >= size - 1 {
        break;
    }

    idx := k;

    next[idx] ::= (grid[idx - 1] + grid[idx] + grid[idx + 1]) / 3;

    k = dk;
}

# simulation loop #
loop {

    p = 0;
    dp ::= p + 1;

    loop {
        if p >= size {
            break;
        }

        if p > 0 && p < size - 1 {
            grid[p] = next[p];
        }

        print grid[p];
        p = dp;
    }

    println 0;
}
