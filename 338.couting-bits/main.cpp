#include <iostream>
#include <vector>

class Solution {
public:
    std::vector<int> countBits(int num) {
        int base = 2;
        int next_base = 4;
        auto count_vec = std::vector<int>();
        
        count_vec.push_back(0);
        if (num == 0) {
            return count_vec;
        }
        count_vec.push_back(1);
        if (num == 1) {
            return count_vec;
        }
        // num >= 2
        for (int i=2; i <= num; i++) {
            if ((i >= base) and (i < next_base)) {
                count_vec.push_back(count_vec[i^base]+1);
            } else {
                base <<= 1;
                next_base <<= 1;
                count_vec.push_back(count_vec[i^base]+1);
            }
        }
        return count_vec;
    }
};


int main() {
	Solution s = Solution();
	auto results = s.countBits(15);
	for (auto i : results) {
        std::cout<<i;
	}
    std::cout<<std::endl;
    return 0;
}
