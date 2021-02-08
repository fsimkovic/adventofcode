
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "Ingredient.h"

int openFile(std::ifstream &file) {
	file.open("input1.txt");
	if (file.is_open()) return 0;
	std::cout << "Unable to open file!" << std::endl;
	return 1;
}

void readFile(std::ifstream &file, std::vector<Ingredient> &ingredients) {
	std::string line, token;
	std::stringstream stream;
	int count;
	while(getline(file, line)) {
		stream.clear();
		stream << line;
		count = 0;
		Ingredient ig;
		while(getline(stream, token, ' ')) {
			switch(count) {
				case 0:
					token.pop_back();
					ig.name = token;
					break;
				case 2:
					token.pop_back();
					ig.capacity = stoi(token);
					break;
				case 4:
					token.pop_back();
					ig.durability = stoi(token);
					break;
				case 6:
					token.pop_back();
					ig.flavor = stoi(token);
					break;
				case 8:
					token.pop_back();
					ig.texture = stoi(token);
					break;
				case 10:
					ig.calories = stoi(token);
					break;
				default:
					break;
			}
			++count;
		}
		ingredients.push_back(ig);
	}
}
