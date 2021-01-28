#include <iostream>
#include <fstream>
#include <array>
#include <vector>
using namespace std;

#define HEIGHT 1000
#define WIDTH 1000

enum class OpCode {toggle, on, off};

void resetGrid(int *grid);
void parseInstruction(string *instruction, OpCode *op, string *start, string *end);
void switchLights(int *grid, OpCode *curop, string *start, string *end);
void splitCoord(string *coord, int *x, int *y);

int main() {
	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		OpCode curop;
		string line, start, end;
		const int size = HEIGHT * WIDTH;
		int lit = 0;

		int grid[size];
		resetGrid(grid);

		while(getline(file, line)) {
			parseInstruction(&line, &curop, &start, &end);
			switchLights(grid, &curop, &start, &end);
		}

		for (int i=0; i<size;i++){
			lit += grid[i];
		}
		cout << "House lit: " << lit << endl;
	} else {
		cout << "Unable to open file!" << endl;
	}
	return 0;
}

void parseInstruction(string *instruction, OpCode *op, string *start, string *end) {
	vector<string> tokens;
	string tmp;

	for (auto chr: *instruction) {
		if (chr == ' ') {
			tokens.push_back(tmp);
			tmp.clear();
		} else {
			tmp += chr;
		}
	}
	*start = tokens[tokens.size() - 2];
	*end = tmp;

	if (tokens[0] == "toggle")
		*op = OpCode::toggle;
	else if (tokens[1] == "on")
		*op = OpCode::on;
	else
		*op = OpCode::off;


}

void switchLights(int *grid, OpCode *curop, string *start, string *end) {
	int x1, y1, x2, y2, i, j;
	splitCoord(start, &x1, &y1);
	splitCoord(end, &x2, &y2);
	int *p;

	for (i=x1; i<x2+1; i++) {
		for (j=y1; j<y2+1; j++) {
			p = &(grid[i*WIDTH+j]);
			switch(*curop) {
				case OpCode::toggle:
					*p = not *p;
					break;
				case OpCode::on:
					*p = 1;
					break;
				case OpCode::off:
					*p = 0;
					break;
			}
		}
	}
}

void splitCoord(string *coord, int *x, int *y) {
	string tmp;
	for (auto chr: *coord) {
		if (chr == ',') {
			*x = stoi(tmp);
			tmp.clear();
		} else {
			tmp += chr;
		}
	}
	*y = stoi(tmp);

}

void resetGrid(int *grid) {
	for (int i=0; i<HEIGHT; i++) {
		for (int j=0; j<WIDTH; j++) {
			grid[i*WIDTH+j] = 0;
		}
	}
}
