#include <unordered_map>
#include <map>
#include <ranges>
#include <algorithm>

class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        /*
            input: some integers, the number of integers to return
            transform: count each integer's frequency
            output: return k number of frequent integers
        */

        std::unordered_map<int, int> fq;
        for (const auto& num : nums) {
            if (auto [_iter, was_inserted] = fq.try_emplace(num, 1); !was_inserted) {
                fq[num] += 1;
            }
        }

        auto vec = std::vector<std::pair<int, int>>(fq.begin(), fq.end());
        std::ranges::sort(vec, std::ranges::greater(), &std::pair<int, int>::second);
        return vec | std::views::transform([](std::pair<int, int> a) {return a.first;}) 
            | std::views::take(k) 
            | std::views::reverse 
            | std::ranges::to<std::vector>();
    }
};
