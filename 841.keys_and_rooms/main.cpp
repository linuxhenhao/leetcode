#include <vector>

class Solution {
    bool visited[1000];
    std::vector<int> available_keys;
public:
    Solution() {
        for(int i=0; i<1000; i++) {
            visited[i] = false;
        }
    }
    bool canVisitAllRooms(std::vector<std::vector<int>>& rooms) {
        
        }

    void visit_room(std::vector<std::vector<int>>& rooms, int room_index){
            auto keys = rooms[room_index];
            for(int key: keys) {
                if (visited[key] != true) {
                    visited[key] == true;
                    available_keys.push_back();
                }
            }  
};