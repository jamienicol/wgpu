void fb1_(inout bool cond)
{
    uint2 loop_bound = uint2(0u, 0u);
    bool loop_init = true;
    while(true) {
        if (!loop_init) {
            bool _e1 = cond;
            if (!(_e1)) {
                break;
            }
        }
        loop_init = false;
        continue;
        if (all(loop_bound == uint2(4294967295u, 4294967295u))) { break; }
        loop_bound += uint2(loop_bound.y == 4294967295u, 1u);
    }
    return;
}

void main_1()
{
    bool param = (bool)0;

    param = false;
    fb1_(param);
    return;
}

void main()
{
    main_1();
}
