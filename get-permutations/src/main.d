import std.conv;

int N = 10;
string[] res;
void rec(int done, string val) {
    if (done == (1<<N) - 1) {
        res ~= val;
        return;
    }

    for (int i = 0; i < N; i++) {
        if (1<<i&done)
            continue;
        char ch = to!char('A' + i);
        rec(1<<i|done, val ~ ch);
    }
}

int main() {
    import std.stdio;
    import core.memory;
    GC.disable();
    rec(0, "");
    writeln("Size: " ~ text(res.length));
    for (int i = 0; i < 5; i++) {
        writeln(res[i]);
    }
    return 0;
}
