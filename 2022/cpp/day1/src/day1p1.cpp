#include "lib.h"

#include <iostream>

int main(void) {
    auto elf_cals = get_elf_calories("input");

    if (!elf_cals.empty()) {
        std::cout << elf_cals[0] << "\n";
    } else {
        std::cout << "Error: no calories" << "\n";
    }

    return 0;
}
