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

		long total = 0, a, b, c, s1, s2, s3;

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

			s1 = a * b;
			s2 = a * c;
			s3 = b * c;

			total = total + 2 * (s1 + s2 + s3) + min(min(s1, s2), s3);
		}
		cout << "Total paper required: " << total << endl;
	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}
