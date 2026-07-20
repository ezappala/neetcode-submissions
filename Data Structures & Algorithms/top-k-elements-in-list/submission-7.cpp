#include <unordered_map>
#include <map>
#include <ranges>
#include <algorithm>

class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        // std::unordered_map<int, int> fq;
        // for (const auto& num : nums) {
        //     if (auto [_iter, was_inserted] = fq.try_emplace(num, 1); !was_inserted) {
        //         fq[num] += 1;
        //     }
        // }

        // auto vec = std::vector<std::pair<int, int>>(fq.begin(), fq.end());
        // std::ranges::sort(vec, std::ranges::greater(), &std::pair<int, int>::second);
        // return vec | std::views::keys 
        //     | std::views::take(k) 
        //     | std::ranges::to<std::vector>();

        std::ranges::sort(nums);
        auto runs = nums 
            | std::views::chunk_by(std::ranges::equal_to{})
            | std::ranges::to<std::vector>();
        
        std::ranges::sort(runs, std::ranges::greater(), [](auto const& run) {return run.size();});

        return runs
            | std::views::take(k)
            | std::views::transform([](auto const& run) { return run.front(); })
            | std::ranges::to<std::vector>();
    }
};
