#include <unordered_set>
#include <unordered_map>
#include <ranges>
#include <string>
#include <algorithm>

namespace rg = std::ranges;

class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.length() != t.length()) return false;
        std::unordered_map<char, int32_t> a{};
        for (const auto& c : s) {
            if (const auto& exists = a.emplace(c, 1); !exists.second) {
                a[c] += 1;
            }
        }

        std::unordered_map<char, int32_t> b{};
        for (const auto& c : t) {
            if (const auto& exists = b.emplace(c, 1); !exists.second) {
                b[c] += 1;
            }
        }

        for (const auto& [k, v] : a) {
            if (!b.contains(k) || b[k] != v) {
                return false;
            }
        }

        return true;
    }
};
