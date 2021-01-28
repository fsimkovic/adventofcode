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
		long x1 = 0, y1 = 0, x2 = 0, y2 = 0;
		set<string> houses;
		houses.insert(to_string(x1) + "-" + to_string(y1));
		string line, tmp;
		bool flag = false;
		long *x, *y;

		while(getline(file, line)) {
			for (auto chr: line) {
				x = flag ? &x2 : &x1;
				y = flag ? &y2 : &y1;

				switch (chr) {
					case moveUp:
						(*x)++;
						break;
					case moveDown:
						(*x)--;
						break;
					case moveLeft:
						(*y)--;
						break;
					case moveRight:
						(*y)++;
						break;
					default:
						cout << "unknown chr: " << chr << endl;
						break;
				}
				flag = !flag;
				houses.insert(to_string(*x) + "-" + to_string(*y));
			}
		};
		cout << "Houses visited: " << houses.size() << endl;
	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}
