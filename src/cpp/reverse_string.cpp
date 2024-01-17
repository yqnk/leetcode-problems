#include <vector>
#include <algorithm>

void reverseString(std::vector<char>& s) {
    std::reverse(s.begin(), s.end());
    //for (int i = 0; i < s.size() / 2; ++i) {
        //std::swap(s[i], s[s.size() - i - 1]);
    //}
}

// https://leetcode.com/problems/reverse-string/description/