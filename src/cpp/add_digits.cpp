class Solution {
public:
    int addDigits(int num) {
        int r = num;
        while (r >= 10) {
            int tmp = r;
            r = 0;
            while (tmp > 0) {
                r += (tmp % 10);
                tmp /= 10;
            }
        }
        return r;
    }
};

// https://leetcode.com/problems/add-digits/description/
