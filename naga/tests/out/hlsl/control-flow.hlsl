void switch_default_break(int i)
{
    do {
        break;
    } while(false);
}

void switch_case_break()
{
    switch(0) {
        case 0: {
            break;
        }
        default: {
            break;
        }
    }
    return;
}

void loop_switch_continue(int x)
{
    uint2 loop_bound = uint2(0u, 0u);
    while(true) {
        bool should_continue = false;
        switch(x) {
            case 1: {
                should_continue = true;
                break;
            }
            default: {
                break;
            }
        }
        if (should_continue) {
            continue;
        }
        if (all(loop_bound == uint2(4294967295u, 4294967295u))) { break; }
        loop_bound += uint2(loop_bound.y == 4294967295u, 1u);
    }
    return;
}

void loop_switch_continue_nesting(int x_1, int y, int z)
{
    uint2 loop_bound_1 = uint2(0u, 0u);
    while(true) {
        bool should_continue_1 = false;
        switch(x_1) {
            case 1: {
                should_continue_1 = true;
                break;
            }
            case 2: {
                switch(y) {
                    case 1: {
                        should_continue_1 = true;
                        break;
                    }
                    default: {
                        uint2 loop_bound_2 = uint2(0u, 0u);
                        while(true) {
                            bool should_continue_2 = false;
                            switch(z) {
                                case 1: {
                                    should_continue_2 = true;
                                    break;
                                }
                                default: {
                                    break;
                                }
                            }
                            if (should_continue_2) {
                                continue;
                            }
                            if (all(loop_bound_2 == uint2(4294967295u, 4294967295u))) { break; }
                            loop_bound_2 += uint2(loop_bound_2.y == 4294967295u, 1u);
                        }
                        break;
                    }
                }
                if (should_continue_1) {
                    break;
                }
                break;
            }
            default: {
                break;
            }
        }
        if (should_continue_1) {
            continue;
        }
        bool should_continue_3 = false;
        do {
            should_continue_3 = true;
            break;
        } while(false);
        if (should_continue_3) {
            continue;
        }
        if (all(loop_bound_1 == uint2(4294967295u, 4294967295u))) { break; }
        loop_bound_1 += uint2(loop_bound_1.y == 4294967295u, 1u);
    }
    uint2 loop_bound_3 = uint2(0u, 0u);
    while(true) {
        bool should_continue_4 = false;
        do {
            do {
                should_continue_4 = true;
                break;
            } while(false);
            if (should_continue_4) {
                break;
            }
        } while(false);
        if (should_continue_4) {
            continue;
        }
        if (all(loop_bound_3 == uint2(4294967295u, 4294967295u))) { break; }
        loop_bound_3 += uint2(loop_bound_3.y == 4294967295u, 1u);
    }
    return;
}

void loop_switch_omit_continue_variable_checks(int x_2, int y_1, int z_1, int w)
{
    int pos_1 = 0;

    uint2 loop_bound_4 = uint2(0u, 0u);
    while(true) {
        bool should_continue_5 = false;
        switch(x_2) {
            case 1: {
                pos_1 = 1;
                break;
            }
            default: {
                break;
            }
        }
        if (all(loop_bound_4 == uint2(4294967295u, 4294967295u))) { break; }
        loop_bound_4 += uint2(loop_bound_4.y == 4294967295u, 1u);
    }
    uint2 loop_bound_5 = uint2(0u, 0u);
    while(true) {
        bool should_continue_6 = false;
        switch(x_2) {
            case 1: {
                break;
            }
            case 2: {
                switch(y_1) {
                    case 1: {
                        should_continue_6 = true;
                        break;
                    }
                    default: {
                        switch(z_1) {
                            case 1: {
                                pos_1 = 2;
                                break;
                            }
                            default: {
                                break;
                            }
                        }
                        break;
                    }
                }
                if (should_continue_6) {
                    break;
                }
                break;
            }
            default: {
                break;
            }
        }
        if (should_continue_6) {
            continue;
        }
        if (all(loop_bound_5 == uint2(4294967295u, 4294967295u))) { break; }
        loop_bound_5 += uint2(loop_bound_5.y == 4294967295u, 1u);
    }
    return;
}

[numthreads(1, 1, 1)]
void main(uint3 global_id : SV_DispatchThreadID)
{
    int pos = (int)0;

    DeviceMemoryBarrierWithGroupSync();
    GroupMemoryBarrierWithGroupSync();
    do {
        pos = 1;
    } while(false);
    int _e4 = pos;
    switch(_e4) {
        case 1: {
            pos = 0;
            break;
        }
        case 2: {
            pos = 1;
            break;
        }
        case 3:
        case 4: {
            pos = 2;
            break;
        }
        case 5: {
            pos = 3;
            break;
        }
        default:
        case 6: {
            pos = 4;
            break;
        }
    }
    switch(0u) {
        case 0u: {
            break;
        }
        default: {
            break;
        }
    }
    int _e11 = pos;
    switch(_e11) {
        case 1: {
            pos = 0;
            break;
        }
        case 2: {
            pos = 1;
            return;
        }
        case 3: {
            pos = 2;
            return;
        }
        case 4: {
            return;
        }
        default: {
            pos = 3;
            return;
        }
    }
}
