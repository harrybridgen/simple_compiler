import std.vector;

v := struct Vector2;

v.x = 3;
v.y = 4;

println v.mag2;   # 25 #
println v.mag;    # 5 #

# move the vector #
v.vx = 1;
v.vy = 2;

v.x = v.dx;
v.y = v.dy;

println v.x;      # 4 #

