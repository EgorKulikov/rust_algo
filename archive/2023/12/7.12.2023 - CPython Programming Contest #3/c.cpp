#include <cstdio>
#include <vector>

using namespace std;

int main() {
    int n;
    scanf("%d", &n);

    auto is_prime = vector<bool>(n + 1, true);
    bool first = true;
    int id = 0;
    is_prime.at(1) = false;
    for (int i = 2; i <= n; i++) {
        if (is_prime.at(i)) {
            id += 1;
            if (is_prime.at(id)) {
                if (first) {
                    first = false;
                } else {
                    printf(" ");
                }
                printf("%d", i);
            }
            for (int j = 2 * i; j <= n; j += i) {
                is_prime.at(j) = false;
            }
        }
    }
    printf("\n");
}
