#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cctype>
#include <cmath>
#include <vector>
#include <algorithm>
#include <unordered_set>
#include <unordered_map>

using namespace std;

vector<long long> seeds;

int main() {
    std::ifstream file("5.input");
    string seed_line;

    std::getline(file, seed_line);
    bool start = false;
    long long curr = 0;
    for (char c : seed_line) {
        if (c == ':') {
            start = true;
            continue;
        }
        if (!start) continue;
        if (std::isspace(c)) {
            if (curr > 0) {
                seeds.emplace_back(curr);
            }
            curr = 0;
            continue;
        }
        curr = curr * 10 + (c - '0');
    }

    seeds.emplace_back(curr);

    string blank;

    std::getline(file, blank);

    string map_name, mapping;
    vector<long long> result = seeds;
    while (std::getline(file, map_name)) {
        while(std::getline(file, mapping)) {
            if (mapping == "") {
                break;
            }
            stringstream ss(mapping);
            long long dest, source, range;
            ss >> dest >> source >> range;

            for (int i = 0; i < result.size(); i++) {
                long long s = seeds[i];
                if (s >= source && s <= source + range) {
                    result[i] = dest + (s - source);
                }
            }
        }
        seeds = result;

    }

    cout << *min_element(seeds.begin(), seeds.end()) << endl;
    return 0;
}
