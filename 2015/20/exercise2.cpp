#include <iostream>
#include <array>
using namespace std;

int main() {
	int target = 33100000, house = 1, elf, count;
	const int limit = 1000000;

	array<int, limit> arr;
	arr.fill(0);

	for (elf = 1; elf < limit; elf++) {
		count = 0;
		for (house = elf; house < limit and count < 50; house += elf) {
			arr[house] += elf * 11;
			count++;
		}
	}

	for (house = 1; house < limit; house++) {
		if (arr[house] >= target)
			break;
	}

	cout << "Answer: " << house << endl;

	return 0;
}
