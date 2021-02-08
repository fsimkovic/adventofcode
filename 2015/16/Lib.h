
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

int openFile(std::ifstream &file) {
	file.open("input1.txt");
	if (file.is_open()) return 0;
	std::cout << "Unable to open file!" << std::endl;
	return 1;
}

void readFile(std::ifstream &file, std::vector<std::map<std::string, int>> &persons) {
	std::string line, token, name, field;
	std::stringstream stream;
	int count;
	while(getline(file, line)) {
		stream.clear();
		stream << line;
		count = 0;
		std::map<std::string, int> data;
		while(getline(stream, token, ' ')) {
			if (count == 0) {
			} else if (count == 1) {
				token.pop_back();
				data["_id"] = std::stoi(token);
			} else if (count % 2 == 0) {
				token.pop_back();
				field = token;
			} else {
				data[field] = std::stoi(token);
			}
			++count;
		}
		persons.push_back(data);
	}
}
