#include <fstream>
#include <iostream>
using namespace std;

int main() {
	ifstream file;
	file.open("input1.txt");
	if (file.is_open()) {
		string line, tmp;
		int total = 0;

		while (getline(file, line)) {
			for (auto &chr: line) {
				if (isdigit(chr) || (chr == '-'))
					tmp += chr;
				else if (tmp.size() > 0) {
					total += stoi(tmp);
					tmp.clear();
				} else
					tmp.clear();
			}
		}
		cout << "Sum: " << total << endl;
	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}
