#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;


void breakIfEmpty() {
    uvec2 loop_bound = uvec2(0u);
    bool loop_init = true;
    while(true) {
        if (all(equal(loop_bound, uvec2(4294967295u)))) { break; }
        loop_bound += uvec2(loop_bound.y == 4294967295u, 1u);
        if (!loop_init) {
            if (true) {
                break;
            }
        }
        loop_init = false;
    }
    return;
}

void breakIfEmptyBody(bool a) {
    bool b = false;
    bool c = false;
    uvec2 loop_bound_1 = uvec2(0u);
    bool loop_init_1 = true;
    while(true) {
        if (all(equal(loop_bound_1, uvec2(4294967295u)))) { break; }
        loop_bound_1 += uvec2(loop_bound_1.y == 4294967295u, 1u);
        if (!loop_init_1) {
            b = a;
            bool _e2 = b;
            c = (a != _e2);
            bool _e5 = c;
            if ((a == _e5)) {
                break;
            }
        }
        loop_init_1 = false;
    }
    return;
}

void breakIf(bool a_1) {
    bool d = false;
    bool e = false;
    uvec2 loop_bound_2 = uvec2(0u);
    bool loop_init_2 = true;
    while(true) {
        if (all(equal(loop_bound_2, uvec2(4294967295u)))) { break; }
        loop_bound_2 += uvec2(loop_bound_2.y == 4294967295u, 1u);
        if (!loop_init_2) {
            bool _e5 = e;
            if ((a_1 == _e5)) {
                break;
            }
        }
        loop_init_2 = false;
        d = a_1;
        bool _e2 = d;
        e = (a_1 != _e2);
    }
    return;
}

void breakIfSeparateVariable() {
    uint counter = 0u;
    uvec2 loop_bound_3 = uvec2(0u);
    bool loop_init_3 = true;
    while(true) {
        if (all(equal(loop_bound_3, uvec2(4294967295u)))) { break; }
        loop_bound_3 += uvec2(loop_bound_3.y == 4294967295u, 1u);
        if (!loop_init_3) {
            uint _e5 = counter;
            if ((_e5 == 5u)) {
                break;
            }
        }
        loop_init_3 = false;
        uint _e3 = counter;
        counter = (_e3 + 1u);
    }
    return;
}

void main() {
    return;
}

