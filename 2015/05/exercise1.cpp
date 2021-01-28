#include <iostream>
#include <fstream>

bool isVowel(char chr);
bool isInvalidSeq(char prev, char cur);

int main() {
	std::ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		std::string line;
		long niceStringCount = 0;
		while (std::getline(file, line)) {
			char prev = '0';
			int vowels = 0;
			bool hasDuplicate = false, isValid = true;
			for (auto cur: line) {
				if (isInvalidSeq(prev, cur)) {
					isValid = false;
					break;
				}
				if (cur == prev) hasDuplicate = true;
				if (isVowel(cur)) ++vowels;
				prev = cur;
			}
			niceStringCount += (isValid && hasDuplicate && (vowels > 2));
		}
		std::cout << "Nice strings: " << niceStringCount << std::endl;
	} else {
		std::cout << "Unable to open file!" << std::endl;
	}

	return 0;
}

bool isVowel(char chr) {
	return ((chr == 'a') || (chr == 'e') || (chr == 'i') || (chr == 'o') || (chr == 'u'));
}

bool isInvalidSeq(char prev, char cur) {
	return (
		((prev == 'a') && (cur == 'b'))
		|| ((prev == 'c') && (cur == 'd'))
		|| ((prev == 'p') && (cur == 'q'))
		|| ((prev == 'x') && (cur == 'y'))
       );
}
