#include "utils.h"

#include <algorithm>
#include <functional>
#include <iostream>

using std::vector;
using std::uint64_t;

using std::cout;
using std::endl;
using std::sort;

vector<uint64_t> get_elf_calories(std::string_view input) {
    auto lines = get_lines(input);

    vector<uint64_t> elf_cals;
    elf_cals.push_back(0);

    int i = 0;

    for (const auto line: lines) {
        if (line.empty()) {
            elf_cals.push_back(0);
            i++;
        } else {
            auto value = atoi(line.c_str());
            elf_cals[i] += value;
        }
    }
    sort(elf_cals.begin(), elf_cals.end(), std::greater<uint64_t>());

    return elf_cals;
}
