#include <unordered_map>
#include <ranges>
#include <algorithm>
#include <optional>

class Solution {
public:
    vector<vector<string>> o{};
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        for (const auto& str : strs) {
            if (o.empty()) {
                o.emplace_back(vector<string>{str});
                continue;
            }

            if (const auto& res = has_subgroup_anagram(str); res.has_value()) {
                res.value()->emplace_back(str);
                continue;
            }

            o.emplace_back(vector<string>{str});
        }

        return o;
    }

    auto has_subgroup_anagram(std::string_view str) -> std::optional<vector<string>*> {
        for (auto& group : o) {
            if (is_anagram(group[0], str)) {
                return &group;
            }
        }

        return std::nullopt;
    }

    auto is_anagram(std::string_view a, std::string_view b) -> bool {
        if (a.size() != b.size()) {
            return false;
        }

        std::array<int, std::numeric_limits<char8_t>::max()> fq{};
        for (std::size_t i = 0; i < a.size(); i += 1) {
            fq[static_cast<char8_t>(a[i])] += 'a';
            fq[static_cast<char8_t>(b[i])] -= 'a';
        }

        return std::ranges::all_of(fq, [](int cnt){ return cnt == 0; });
    }
};
