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
    return x;
}
