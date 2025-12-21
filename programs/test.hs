struct Mat2 {
    m;
}

func mat2(a00, a01, a10, a11) {
    A := struct Mat2; # the := here is very important! #
    A.m = [2];
    A.m[0] = [2];
    A.m[1] = [2];

    A.m[0][0] = a00;
    A.m[0][1] = a01;
    A.m[1][0] = a10;
    A.m[1][1] = a11;

    return A;
}

func mat2mul(A, B) {
    C := struct Mat2;
    C.m = [2];
    C.m[0] = [2];
    C.m[1] = [2];

    C.m[0][0] ::= A.m[0][0]*B.m[0][0] + A.m[0][1]*B.m[1][0];
    C.m[0][1] ::= A.m[0][0]*B.m[0][1] + A.m[0][1]*B.m[1][1];

    C.m[1][0] ::= A.m[1][0]*B.m[0][0] + A.m[1][1]*B.m[1][0];
    C.m[1][1] ::= A.m[1][0]*B.m[0][1] + A.m[1][1]*B.m[1][1];

    return C;
}

A = mat2(1, 2, 3, 4);
B = mat2(5, 6, 7, 8);
C = mat2mul(A, B);

println C.m[0][0];
println C.m[0][1];
println C.m[1][0];
println C.m[1][1];

