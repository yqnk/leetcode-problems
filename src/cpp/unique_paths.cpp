#include <iostream>
#include <vector>

void printVV(std::vector<std::vector<int>>& adj) {
    for (const auto& vec : adj) {
        for (const auto& value : vec) {
            std::cout << value << " ";
        }
        std::cout << std::endl;
    }
}

int uniquePaths(int m, int n) {
    // Pascal's Triangle
    std::vector<std::vector<int>> dp(m, std::vector<int>(n, 1)); // dp[i][j] = dp[i-1][j] + dp[i][j-1]
    for (int i = 1; i < m; ++i) {
        for (int j = 1; j < n ; ++j) {
            dp[i][j] = dp[i-1][j] + dp[i][j-1];
        }
    }
    // printVV(dp);
    return dp[m-1][n-1];
}

int main() {
    uniquePaths(3, 4);
    return 0;
}

// https://leetcode.com/problems/unique-paths/description/