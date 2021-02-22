#include <iostream>
#include <algorithm>
#include <fstream>
#include <string>
#include <vector>
using namespace std;


vector<int> seek(int capacity, vector<int> available, int count) {

	if (capacity == 0) {
		vector<int> output = {count};
		return output;
	} else if (capacity < 0) {
		vector<int> output;
		return output;
	}

	vector<int> paths;
	int original = capacity;
	while (available.size()) {
		capacity -= available.back();
		available.pop_back();
		for (auto &i: seek(capacity, available, count + 1))
			if (i > 0)
				paths.push_back(i);
		capacity = original;
	};
	return paths;
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

		vector<int> paths = seek(capacity, capacities, 0);
		sort(paths.begin(), paths.end());
		int minVal = paths.front(), count = 0;
		for (auto &i: paths)
			if (i == minVal)
				++count;
			else
				break;
		cout << count << endl;

	} else {
		cout << "Unable to open file!" << endl;
	}
	return 0;
}
