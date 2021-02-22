#include <iostream>
#include <algorithm>
#include <fstream>
#include <string>
#include <vector>
using namespace std;

int seek(int capacity, vector<int> available) {
	if (capacity == 0)
		return 1;
	else if (capacity < 0)
		return 0;

	int count = 0;
	int original = capacity;
	while (available.size()) {
		capacity -= available.back();
		available.pop_back();
		count += seek(capacity, available);
		capacity = original;
	};
	return count;
};


int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		string token;
		vector<int> capacities;

		int capacity = 150;

		while (getline(file, token)) {
			capacities.push_back(stoi(token));
		}
		sort(capacities.begin(), capacities.end());
		cout << seek(capacity, capacities) << endl;

	} else {
		cout << "Unable to open file!" << endl;
	}
	return 0;
}
