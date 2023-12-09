#include <iostream>
#include <fstream>
#include <string>
#include <cctype>
#include <cmath>
#include <unordered_set>

using std::string;
using std::unordered_set;


int get_value(string card, int j) {
    unordered_set<int> winners;
    int curr = 0, result = 0;
    bool start = false;
    int i = 0;
    for (; i < card.length(); i++) {
        char c = card[i];
        if (c == ':') {
            start = true;
            continue;
        }
        if (!start) continue;
        if (std::isspace(c)) {
            if (curr > 0) {
                winners.insert(curr);
            }
            curr = 0;
            continue;
        }
        if (c == '|') break;
        curr = curr * 10 + (c - '0');
    }

    curr = 0;

    unordered_set<int> used;
    for (i = i + 1; i < card.length(); i++) {
        char c = card[i];
        if (std::isspace(c)) {
            if (curr > 0) {
                if (used.count(curr) > 0) {
                    
                    std::cout << "REPEAT" << curr << " " << used.count(curr) << std::endl;
                } 
                if (winners.count(curr) > 0) {
                   // std::cout << curr << std::endl;
                    result += 1;
                    used.insert(curr);
                }
                
            }
            curr = 0;
            continue;
        }
        curr = curr * 10 + (c - '0');
    }

    if (winners.count(curr) > 0) {
        result += 1;
    }

    return result > 0 ? pow(2, result - 1): 0;
}

int main() {
    std::ifstream file("4.input");
    string line;
    int sum = 0;
    int j = 0;
    while (std::getline(file, line)) {sum += get_value(line, j); j += 1;}
    std::cout << sum << std::endl;
    return 0;
}
