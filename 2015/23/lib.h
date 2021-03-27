// 1. read file & store instructions --> vector<string>
// 2. create registers and init with value 0
// 2. hold pointer to current instruction
// 3. process instructions until pointer exceeds array bounds

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <string>
using namespace std;

void getInstructions(vector<string> &instructions);
void processInstructions(vector<string> &instructions, int &a, int &b);
int getSteps(string &instruction, int start);

void getInstructions(vector<string> &instructions) {
	string token;
	ifstream file;
	file.open("input1.txt");
	while (getline(file, token))
		instructions.push_back(token);
}

void processInstructions(vector<string> &instructions, int &a, int &b) {
	int p = 0, steps;
	string instruction;

	while ((p >= 0) && (p < instructions.size())) {
		instruction = instructions[p];

		if (instruction[0] == 'i') {
			(instruction[4] == 'a' ? a : b)++;
		} else if (instruction[0] == 'h') {
			(instruction[4] == 'a' ? a : b) /= 2;
		} else if (instruction[0] == 't') {
			(instruction[4] == 'a' ? a : b) *= 3;
		} else if (instruction[0] == 'j') {
			if (instruction[1] == 'm') {
				p += getSteps(instruction, 4);
				continue;
			} else if (instruction[2] == 'e') {
				if ((instruction[4] == 'a' ? a : b) % 2 == 0){
					p += getSteps(instruction, 7);
					continue;
				}
			} else {
				if ((instruction[4] == 'a' ? a : b) == 1) {
					p += getSteps(instruction, 7);
					continue;
				}
			}
		}
		p++;
	}
}

int getSteps(string &instruction, int start) {
	string tmp;
	for (int i = start; i < instruction.size(); i++)
		tmp += instruction[i];
	return stoi(tmp);
}
