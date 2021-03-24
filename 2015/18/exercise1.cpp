#include <iostream>
#include <fstream>
#include "lib.h"

using namespace std;

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		int m = 100, n = 100, steps = 100;
		int arr[m*n];

		ingestData(file, arr);

		for (; steps > 0; steps--)
			animateLights(arr, m, n);

		cout << countLights(arr, m, n) << endl;

	} else {
		cout << "Unable to open file!" << endl;
	}

	return 0;
}
