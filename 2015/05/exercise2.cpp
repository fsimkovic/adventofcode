#include <iostream>
#include <fstream>
#include <string>
#include <set>
using namespace std;

void construct(string *tmp, char a, char b);
void store(set<string> *pairs, string *tmp, int *pairCount);
void reset(set<string> *pairs, int *tripletCount, int *pairCount, int *i);

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		string line, tmp;
		int niceStringCount = 0, tripletCount, pairCount, i, move;
		set<string> pairs;

		while (getline(file, line)) {
			reset(&pairs, &tripletCount, &pairCount, &i);
			do {
				tripletCount += (line[i] == line[i+2]);

				construct(&tmp, line[i], line[i+1]);
				store(&pairs, &tmp, &pairCount);

				move = ((line[i] == line[i+1]) && (line[i] == line[i+2])) ? 2: 1;
				i += move;

			} while (i < line.length() - 2);

			construct(&tmp, line[i], line[i+1]);
			store(&pairs, &tmp, &pairCount);

			if ((tripletCount > 0) && (pairCount - pairs.size() > 0))
				niceStringCount++;
		}
		std::cout << "Nice strings: " << niceStringCount << std::endl;

	} else {
		cout << "Unable to open file!" << endl;

	}
	return 0;
}

void construct(string *tmp, char a, char b) {
	tmp->clear();
	*tmp = *tmp + a + b;
}

void store(set<string> *pairs, string *tmp, int *pairCount) {
	pairs->insert(*tmp);
	(*pairCount)++;
}

void reset(set<string> *pairs, int *tripletCount, int *pairCount, int *i){
	pairs->clear();
	*tripletCount = 0;
	*pairCount = 0;
	*i = 0;
}
