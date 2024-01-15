#include <iostream>
#include <vector>

using namespace std;

void printVV(vector<vector<int>>& adj) {
    for (const auto& vec : adj) {
        for (const auto& value : vec) {
            std::cout << value << " ";
        }
        std::cout << std::endl;
    }
}

int longestPath(vector<vector<int>>& adj, int current, vector<int>& paths) {
    if (paths[current] != -1) {
        return paths[current];
    }

    int maxLength = 1; // starts at 1, counts itself
    for (int neighbor : adj[current]) {
        int neighborLength = longestPath(adj, neighbor, paths) + 1;
        maxLength = max(maxLength, neighborLength);
    }

    paths[current] = maxLength;
    return maxLength;
}

int lengthOfLIS(vector<int>& nums) {
    int r = 1;
    // graph way
    vector<vector<int>> adj(nums.size());
    for (int i = 0; i < nums.size(); ++i) {
        for (int j = i; j < nums.size(); j++) {
            if (nums[i] < nums[j]) {
                adj[i].push_back(j);
            }
        }
    }

    // store LIS
    vector<int> paths(nums.size(), -1);

    // find longest path in graph
    for (int i = 0; i < adj.size(); ++i) {
        r = max(r, longestPath(adj, i, paths));
    }
    return r;
}

int main() {
    vector<int> test1 = { 10, 9, 2, 5, 3, 7, 101, 18 };
    std::cout << lengthOfLIS(test1) << std::endl;
    return 0;
}
