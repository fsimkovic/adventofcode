#include <iostream>
#include <array>
using namespace std;

int main() {
	int target = 33100000, house = 1, elf;
	const int limit = 800000;

	array<int, limit> arr;
	arr.fill(0);

	for (elf = 1; elf < limit; elf++)
		for (house = elf; house < limit; house += elf)
			arr[house] += elf * 10;

	for (house = 1; house < limit; house++) {
		if (arr[house] >= target)
			break;
	}

	cout << "Answer: " << house << endl;

	return 0;
}
