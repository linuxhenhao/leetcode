#include <vector>
#include <iostream>
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};
class Solution {
    TreeNode* target;
    int distance;
    std::vector<int> node_values;
public:
    std::vector<int> distanceK(TreeNode* root, TreeNode* target, int K) {
        this->target = target;
        this->distance = K;
        searchDistance(root);
        return node_values;
    }

    int searchDistance(TreeNode* root) {
        if (root == target) {
            add_to_node_values(root, distance);
            return distance - 1;
        }
        else {
            if (root == nullptr) {
                return -1;
            }
            else {
                int target_distance;
                target_distance = searchDistance(root->left);
                if (target_distance >= 0) {
                    add_to_node_values(root, target_distance);
                    return target_distance - 1;
                }
                target_distance = searchDistance(root->right);
                if (target_distance >= 0) {
                    add_to_node_values(root, target_distance);
                    return target_distance - 1;
                }
                return -1;
            }
        }
    }

    void add_to_node_values(TreeNode* root, int height) {
        if (root == nullptr) {
            return;
        }
        if (height == 0 and root != target) {
            node_values.push_back(root->val);
        } else {
            if (root == target and height != distance) {
                return;
            }
            add_to_node_values(root->left, height-1);
            add_to_node_values(root->right, height-1);
        }
    }
};

int main() {

    TreeNode root = TreeNode(0);
    TreeNode node2 = TreeNode(2);
    TreeNode node1 = TreeNode(1);
    TreeNode node3 = TreeNode(3);
    root.left = &node2;
    root.right = &node1;
    node1.right = &node3;
    auto s = Solution();
    auto result = s.distanceK(&root, &node3, 3);
    for(auto value: result) {
        std::cout<<value<<'\t'<<std::endl;
    }
    return 0;
}