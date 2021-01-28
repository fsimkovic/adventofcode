#include <fstream>
#include <iostream>
#include <string>
using namespace std;

int main() {

	ifstream file;
	file.open("input1.txt");
	if (file.is_open()) {
		string line;
		long floor = 0;
		long pos = 0;
		while (getline(file, line)) {
			for (auto chr: line) {
				pos++;
				if (chr == '(') {
					floor++;
				}else {
					floor--;
				}
				if (floor < 0) {
					cout << "Basement reached: " << pos << endl;
					break;
				}
			}
		}
		file.close();
	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}
