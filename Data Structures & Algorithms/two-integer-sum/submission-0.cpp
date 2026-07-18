#include <ranges>
#include <utility>

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        std::unordered_map<int, int> a{};

        for (const auto& [i, x] : std::views::enumerate(nums)) {
            int diff = target - x;
            if (!a.contains(diff)) {
                a.emplace(x, i);
                continue;
            }

            return {a[diff], static_cast<int>(i)};
        }

        std::unreachable();
    }
};
