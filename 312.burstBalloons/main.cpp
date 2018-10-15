#include <iostream>
#include <vector>


typedef struct {
    std::vector* key;
    int value;
} pair;


class Solution {
    public:
        int maxCoins(std::vector<int>& nums) {
            pair[] result_storage[nums.size()]
            return maxCoins_n(nums, nums.size());
        }


        void quick_sort(std::vector<int>& nums, int start, int stop) {
            // stop is one step further than the index of last element
            if (stop-start <= 1) {
                return;
            }
            // more than one element
            int i=start;
            int j=stop-1;
            auto flag = nums[start];

            while (true) {
                if(j<i){
                    break;
                } else {
                    if (nums[i]>flag) {
                        auto tmp = nums[j];
                        nums[j] = nums[i];
                        nums[i] = tmp;
                        j--;
                    } else {
                        i++;
                    }
                }
            } 
            // next iter
            auto tmp = nums[j];
            nums[j] = nums[start];
            nums[start] = tmp;
            quick_sort(nums, start, i-1);
            quick_sort(nums, i, stop);
        }

    hash
};

int main() {
    Solution s = Solution();
    auto vec = std::vector<int>({7, 13, 24, 9, 5, 10, 22, 13});
    s.quick_sort(vec, 0, 9);
    for (auto i: vec) {
        std::cout<<i<<'\t';
    }
    return 0;
}