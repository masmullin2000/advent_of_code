#include "lib.h"

#include <iostream>
#include <numeric>

int main(void) {
    auto elf_cals = get_elf_calories("input");

    if (elf_cals.size() >= 3) {
        auto rc = std::accumulate(elf_cals.begin(), elf_cals.begin() +3, 0);
        std::cout << rc << "\n";
    } else {
        std::cout << "Error: no calories" << "\n";
    }

    return 0;
    
}
