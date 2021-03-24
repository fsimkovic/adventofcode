#include <iostream>
#include <fstream>
#include "lib.h"

using namespace std;

void fixLights(int arr[], int &m, int &n);

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		int m = 100, n = 100, steps = 100;
		int arr[m*n];

		ingestData(file, arr);
		fixLights(arr, m, n);

		for (; steps > 0; steps--) {
			animateLights(arr, m, n);
			fixLights(arr, m, n);
		}

		cout << countLights(arr, m, n) << endl;

	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}

void fixLights(int arr[], int &m, int &n) {
	arr[0 * m + 0] = ON;
	arr[0 * m + (n - 1)] = ON;
	arr[(m - 1) * m + 0] = ON;
	arr[(m - 1) * m + (n - 1)] = ON;
}
