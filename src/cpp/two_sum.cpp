#include <vector>
#include <map>

using namespace std;

class Solution
{
public:
    vector<int> twoSum(vector<int> &nums, int target)
    {
        vector<int> result;
        map<int, int> map;

        for (int i = 0; i < nums.size(); i++)
        {
            int complement = target - nums[i];
            if (map.find(complement) != map.end())
            {
                result.push_back(map[complement]);
                result.push_back(i);
                return result;
            }
            map[nums[i]] = i;
        }

        return result;
    }
};

// https://leetcode.com/problems/two-sum/description/