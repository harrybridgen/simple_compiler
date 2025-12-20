datasize = 10;
data = [datasize];

i = 0;
di := i + 1;

loop {
    if i >= datasize {
        break;
    }

    data[i] = i;
    i = di;
}

winsize = 5;
win = [winsize];

offset = 0;
doffset := offset + 1;

k = 0;
dk := k + 1;

loop {
    if k >= winsize {
        break;
    }

    
    win[k] = data[offset + k];

    k = dk;
}

loop {

    p = 0;
    dp := p + 1;

    loop {
        if p >= winsize {
            break;
        }

        print win[p];
        p = dp;
    }

    println -1;
    offset = doffset;

    if offset > datasize - winsize {
        break;
    }
}
