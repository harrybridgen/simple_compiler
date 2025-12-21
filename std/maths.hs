# ========================================= #
#   Reactive Standard Mathematics Library   #
# ========================================= #
#                                           #
# This module provides basic integer math utilities. #
#                                                    #
# All functions are:                                 #
# - pure                                             #
# - side-effect free                                 #
# - safe for use in reactive expressions (::=)       #
#                                                    #
# Values are 32-bit signed integers.                 #
#                                                    #
# Import with:                                       #
#     import std.maths;                              #
#                                                    #
# ================================================== #


# Absolute value #
func abs(x) {
    if x < 0 {
        return -x;
    }
    return x;
}


# Clamp x into the inclusive range [lo, hi] #
func clamp(x, lo, hi) {
    if x < lo {
        return lo;
    }
    if x > hi {
        return hi;
    }
    return x;
}


# Square a number #
func square(x) {
    return x * x;
}

# Cube a number #
func cube(x) {
    return x * x * x;
}


# Minimum of two values #
func min(x, y) {
    if x > y {
        return y;
    }
    return x;
}


# Maximum of two values #
func max(x, y) {
    if x > y {
        return x;
    }
    return y;
}

# 1 for positive, 0 for 0, -1 for negative #
func sign(x) {
    if x > 0 { return 1; }
    if x < 0 { return -1; }
    return 0;
}

# 1 for even, 0 for odd #
func iseven(x) {
    if abs(x) % 2 == 0 {
        return 1;
    }
    return 0;
}

# 1 for odd, 0 for even #
func isodd(x){
    if abs(x) % 2 != 0 {
        return 1;
    }
    return 0;
}

# Safe positive modulo #
func mod(a, b){
    if b == 0 { return 0; }
    a = abs(a);
    b = abs(b);
    return a % b;
}

# Integer square root (floor) #
func sqrt(x) {
    if x < 0 {
        return 0;
    }

    i = 0;
    di ::= i + 1;

    loop {
        if (i + 1) * (i + 1) > x {
            break;
        }
        i = di;
    }

    return i;
}