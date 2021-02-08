#include "Lib.h"
#include <iostream>
#include <vector>
#include <fstream>
#include <string>
#include <map>
using namespace std;

bool match(map<string, int> &expected, map<string, int> &person);

int main() {
	ifstream file;
	if (openFile(file) == 1)
		return 1;

	vector<map<string, int>> persons;
	readFile(file, persons);

	map<string, int> reference = {
		{"children", 3},
		{"cats",  7},
		{"samoyeds", 2},
		{"pomeranians", 3},
		{"akitas", 0},
		{"vizslas", 0},
		{"goldfish", 5},
		{"trees", 3},
		{"cars", 2},
		{"perfumes", 1},
	};

	for (auto &p: persons)
		if (match(reference, p))
			cout << "Match: Sue " << p["_id"] << endl;

	return 0;
}

bool match(map<string, int> &expected, map<string, int> &person) {
	for (auto &f: expected)
		if ((person.count(f.first)) && (f.second != person[f.first]))
			return false;
	return true;
}
