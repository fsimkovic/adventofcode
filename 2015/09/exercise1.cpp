#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <map>
#include <set>
#include <vector>
using namespace std;

void ingestDists(ifstream &file, map<string, map<string, int>> &dists);
void calcDist(string curCity, const int &totalCities, int curDist, vector<string> &visited, map<string, map<string, int>> &dists, vector<int> &combined);

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		map<string, map<string, int>> dists;
		ingestDists(file, dists);

		set<string> cities;
		for (const auto &item: dists) {
			cities.insert(item.first);
			for (const auto &elem:dists[item.first])
				cities.insert(elem.first);
		}
		const int totalCities = cities.size();

		vector<int> combined;
		for (const auto &item: dists) {
			vector<string> visited = {item.first};
			calcDist(item.first, totalCities, 0, visited, dists, combined);
		}
		int minDist = combined[0];
		for (const auto &v: combined)
			minDist = min(minDist, v);
		cout << "Min. Dist: " << minDist << endl;

	} else {
		cout << "Unable to open file!" << endl;
	}
	return 0;
}

void calcDist(string curCity, const int &totalCities, int curDist, vector<string> &visited, map<string, map<string, int>> &dists, vector<int> &combined) {


	if (visited.size() == totalCities){
		combined.push_back(curDist);
	} else {

		bool skip;
		for (const auto &dest: dists[curCity]) {
			skip = false;
			for (auto el: visited)
				if (dest.first == el)
					skip = true;
			if (skip) continue;

			vector<string> tmp;
			for (auto el: visited)
				tmp.push_back(el);
			tmp.push_back(dest.first);
			calcDist(dest.first, totalCities, curDist + dest.second, tmp, dists, combined);
		}
	}
}


void ingestDists(ifstream &file, map<string, map<string, int>> &dists) {
	stringstream lineStream;
	string line, token, src, dest;
	int i;

	while(getline(file, line)) {
		lineStream.clear();
		lineStream.str(line);
		
		i = 0;
		while (getline(lineStream, token, ' ')) {
			if ((token == "to") || (token == "="))
				continue;

			switch(i) {
				case 0:
					src = token;
					break;
				case 1:
					dest = token;
					break;
				case 2:
					dists[src][dest] = stoi(token);
					dists[dest][src] = stoi(token);
					break;
			}
			i++;
		}
	}
}
