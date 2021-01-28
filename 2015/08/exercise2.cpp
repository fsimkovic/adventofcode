#include <fstream>
#include <iostream>
#include <string>
using namespace std;

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		string line;

		int nchars = 0, nencod = 0;

		while (getline(file, line)) {
			for (auto chr: line) {
				if ((chr == '\"') || (chr == '\\'))
					nencod++;
				nencod++;
			}
			nencod += 2;
			nchars += line.size();
		}
		cout << "Char Diff: " << nencod - nchars << endl;
	} else {
		cout << "Unable to open the file!" << endl;
	}

	return 0;
}
