# EAGER ASSIGNMENTS # 
x = 0;
vx = 1; 

# LAZY ASSIGNMENTS #
dx := x + vx; 

loop {
    x = dx;
    print x;
}