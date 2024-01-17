#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int profit = 0;
        int buy = prices[0];
        for (int i = 1; i < prices.size(); i++) {
            profit = std::max(profit, prices[i] - buy);
            buy = std::min(buy, prices[i]);
        }
        return profit;
    }
};

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/