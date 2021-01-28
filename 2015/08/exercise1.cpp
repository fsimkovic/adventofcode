#include <fstream>
#include <iostream>
#include <string>
using namespace std;

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		string line;

		int i, nchars = 0, nvalid = 0;

		while (getline(file, line)) {
			i = 1;
			while (i < line.size() - 1) {
				if (line[i] == '\\') {
					if (line[i+1] == 'x')
						i += 3;
					else if ((line[i+1] == '\\') || (line[i+1] == '\"'))
						i += 1;
				}
				nvalid++;
				i++;
			}
			nchars += line.size();
		}
		cout << "Char Diff: " << nchars - nvalid << endl;
	} else {
		cout << "Unable to open the file!" << endl;
	}

	return 0;
}
