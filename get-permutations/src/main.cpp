#include <bits/stdc++.h>
using namespace std;

int N = 10;
vector<string> res;
void rec(int done, string val) {
    if (done == (1<<N) - 1) {
        res.push_back(val);
        return;
    }

    for (int i = 0; i < N; i++) {
        if (1<<i&done)
            continue;
        char ch = 'A' + i;
        rec(1<<i|done, val + ch);
    }
}

int main() {
    rec(0, "");
    cout<<"Size: "<<res.size()<<endl;
    for (int i = 0; i < 5; i++) {
        cout<<res[i]<<endl;
    }
}
