#include <unordered_map>
#include <map>
#include <ranges>
#include <algorithm>

template <class Comp = std::ranges::less, class Proj = std::identity>
struct sort_action : std::ranges::range_adaptor_closure<sort_action<Comp, Proj>> {
    [[no_unique_address]] Comp comp{};
    [[no_unique_address]] Proj proj{};

    constexpr sort_action() = default;

    constexpr sort_action(Comp comp, Proj proj)
        : comp(std::move(comp)), proj(std::move(proj)) {}

    template <std::ranges::random_access_range R>
        requires std::sortable<std::ranges::iterator_t<R>, Comp, Proj>
    constexpr R&& operator()(R&& range) const {
        std::ranges::sort(range, comp, proj);
        return std::forward<R>(range);
    }
};

template <class Comp, class Proj = std::identity>
sort_action(Comp, Proj) -> sort_action<Comp, Proj>;

class Solution {
   public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        return nums | sort_action{} | std::views::chunk_by(std::ranges::equal_to{}) |
               std::ranges::to<std::vector>() |
               sort_action{std::ranges::greater{}, [](auto const& run) { return run.size(); }} |
               std::views::take(k) |
               std::views::transform([](auto const& run) { return run.front(); }) |
               std::ranges::to<std::vector>();
    }
};
