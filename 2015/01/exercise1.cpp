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

		while( getline(file, line)) {
			for (auto chr : line) {
				if (chr == '(') {
					floor++;
				} else {
					floor--;
				}
			}
		}
		cout << "Current floor: " << floor << endl;
		file.close();
	} else {
		cout  << "File not found!" << endl;
	}
	return 0;
}
