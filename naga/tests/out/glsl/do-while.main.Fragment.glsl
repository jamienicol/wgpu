#version 310 es

precision highp float;
precision highp int;


void fb1_(inout bool cond) {
    uvec2 loop_bound = uvec2(0u);
    bool loop_init = true;
    while(true) {
        if (all(equal(loop_bound, uvec2(4294967295u)))) { break; }
        loop_bound += uvec2(loop_bound.y == 4294967295u, 1u);
        if (!loop_init) {
            bool _e1 = cond;
            if (!(_e1)) {
                break;
            }
        }
        loop_init = false;
        continue;
    }
    return;
}

void main_1() {
    bool param = false;
    param = false;
    fb1_(param);
    return;
}

void main() {
    main_1();
}

