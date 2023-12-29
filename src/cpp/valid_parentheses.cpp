#include <string>
#include <vector>

using namespace std;

class Solution
{
public:
    bool isValid(string s)
    {
        vector<char> stack;

        for (size_t i = 0; i < s.size(); i++)
        {
            if (s[i] == '(' || s[i] == '[' || s[i] == '{')
            {
                stack.push_back(s[i]);
            }
            else
            {
                if (stack.empty())
                {
                    return false;
                }

                char top = stack.back();
                stack.pop_back();

                if (s[i] == ')' && top != '(')
                {
                    return false;
                }
                else if (s[i] == ']' && top != '[')
                {
                    return false;
                }
                else if (s[i] == '}' && top != '{')
                {
                    return false;
                }
            }
        }

        return stack.empty();
    }
};