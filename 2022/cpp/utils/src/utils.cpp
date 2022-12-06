#include <iostream>
#include <fstream>

#include "utils.h"

#ifndef BOOST
std::vector<std::string> get_lines(std::string_view input) {
    std::ifstream file(input.data());

    std::vector<std::string> rc;
    while (!file.eof()) {
        std::string line;
        std::getline(file, line);
        rc.push_back(line);
    }

    return rc;
}
#else
#include <boost/iostreams/device/mapped_file.hpp>

using boost::iostreams::mapped_file;

const char *find_memchr(const char *b, const char *e, char c)
{
  return static_cast<const char*>(memchr(b, c, e-b));
}

std::vector<std::string> get_lines(std::string_view input) {
    mapped_file file(input.data(), boost::iostreams::mapped_file::readonly);

    auto curr = file.const_data();
    auto last = curr + file.size();

    std::vector<std::string> rc;
    while (curr && curr != last) {
        auto f = curr;
        if (NULL != (curr = find_memchr(f, last, '\n'))) {
            rc.emplace_back(std::string(f, curr));
            curr += 1;
        }
    }

    return rc;
}
#endif
