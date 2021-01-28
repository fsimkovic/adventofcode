#include <fstream>
#include <iostream>
#include <string>
#include <set>
using namespace std;

const char moveUp = '^';
const char moveDown = 'v';
const char moveLeft = '<';
const char moveRight = '>';

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		long x = 0, y = 0;
		set<string> houses;
		houses.insert(to_string(x) + "-" + to_string(y));
		string line, tmp;
		while(getline(file, line)) {
			for (auto chr: line) {
				switch (chr) {
					case moveUp:
						++x;
						break;
					case moveDown:
						--x;
						break;
					case moveLeft:
						--y;
						break;
					case moveRight:
						++y;
						break;
					default:
						cout << "unknown chr: " << chr << endl;
						break;
				}
				houses.insert(to_string(x) + "-" + to_string(y));
			}
		};
		cout << "Houses visited: " << houses.size() << endl;
	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}
