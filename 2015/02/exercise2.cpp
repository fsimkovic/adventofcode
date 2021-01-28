#include <iostream>
#include <fstream>
#include <stack>
#include <string>
using namespace std; 

int main() {
	fstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		string line, tmp;
		stack<long> cont;

		long total = 0, a, b, c, s1, s2;

		while (getline(file, line)) {
			for (auto chr: line) {
				if (chr == 'x') {
					cont.push(stol(tmp));
					tmp.clear();
				} else {
					tmp.push_back(chr);
				}
			}
			cont.push(stol(tmp));
			tmp.clear();

			a = cont.top();
			cont.pop();
			b = cont.top();
			cont.pop();
			c = cont.top();
			cont.pop();

			if (a < b) {
				s1 = a;
				s2 = b < c ? b: c;
			} else {
				s1 = b;
				s2 = a < c ? a: c;
			}

			total = total + 2 * (s1 + s2) + a * b * c;
		}
		cout << "Total ribbon required: " << total << endl;
	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}
