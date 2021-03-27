// 1. read file & store instructions --> vector<string>
// 2. create registers and init with value 0
// 2. hold pointer to current instruction
// 3. process instructions until pointer exceeds array bounds

#include <iostream>
#include <string>
#include <vector>

#include "lib.h"
using namespace std;

int main() {
	int a, b;

	cout << "Enter value for register 'a': ";
	cin >> a;

	cout << "Enter value for register 'b': ";
	cin >> b;

	vector<string> instructions;
	getInstructions(instructions);
	processInstructions(instructions, a, b);
	cout << "Register 'a': " << a << ", Register 'b': " << b << endl;

	return 0;
}
