#include <string>
#include <cstdlib>
#include <iostream>

using namespace std;

class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.length() != t.length()) {
            return false;
        }

        int histogram[256]{0};
        for (char c : s) { // (auto c : s)
            ++histogram[c];
        }
        for (char c : t) { // (auto c : t)
            --histogram[c];
        }
        for (int i = 0; i < 256; i++) {
            if (histogram[i] != 0) {
                return false;
            }
        }
        return true;
    }
};

int main(void) {
    Solution solution;
    string s = "";
    string t = "";
    bool ans = solution.isAnagram(s, t);
    if (ans) {
        exit(EXIT_SUCCESS);
    } else {
        exit(EXIT_FAILURE);
    }
}

// https://leetcode.com/problems/valid-anagram/description/