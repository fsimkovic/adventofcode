#include <iostream>
#include <map>
#include <string>

#define ON 1
#define OFF 0

using namespace std;

void ingestData(ifstream &file, int arr[]) {
	int p = 0;
	string token;
	while (getline(file, token))
		for (auto &chr: token)
			arr[p++] = chr == '#' ? ON : OFF;
}

void animateLights(int arr[], int &m, int &n) {
	int pos, cur, sum, tmp;
	map<int, int> mem;

	for (int i = 0; i < m; i++) {
		for (int j = 0; j < n; j++) {
			sum = 0;

			if (i - 1 >= 0) {
				tmp = (i - 1) * m;
				sum += (j - 1) < 0 ? 0 : arr[tmp + j - 1];  	// one up, one left
				sum += arr[tmp + j];  				// one up
				sum += (j + 1) < n ? arr[tmp + j + 1] : 0;  	// one up, one right
			}
			if (i + 1 < m) {
				tmp = (i + 1) * m;
				sum += (j - 1) < 0 ? 0 : arr[tmp + j - 1];  	// one down, one left
				sum += arr[tmp + j];  				// one down
				sum += (j + 1) < n ? arr[tmp + j + 1] : 0;  	// one down, one right
			}
			tmp = i * m;
			sum += (j - 1) < 0 ? 0 : arr[tmp + j - 1];  		// one left
			sum += (j + 1) < n ? arr[tmp + j + 1] : 0;  		// one right

			if (arr[tmp + j] == 1)
				pos = ((sum == 2) || (sum == 3)) ? ON : OFF;
			else
				pos = (sum == 3) ? ON : OFF;

			if (pos != arr[tmp + j])
				mem[tmp + j] = pos;
		}
	}

	for (auto &pair: mem) arr[pair.first] = pair.second;
}

int countLights(int arr[], int &m, int &n) {
	int total = 0;
	for (int i = 0; i < m * n; i++)
		total += arr[i];
	return total;
}
