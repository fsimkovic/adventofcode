// Genetic algorithm

#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <map>
#include <time.h>
#include <stdlib.h>
#include <stdio.h>
using namespace std;

void addSelf(map<string, map<string, int>> &happiness);
void ingestData(ifstream &file, map<string, map<string, int>> &happiness);
void optimizeHappiness(vector<string> &seats, map<string, map<string, int>> &happiness, int maxCount);
int computeHappiness(vector<string> &seats, map<string, map<string, int>> &happiness);
void randomSwap(vector<string> &seats, int nSize);
void printOverview(vector<string> &seats, map<string, map<string, int>> &happiness);

int main() {
	srand(time(NULL));

	ifstream file;
	file.open("input1.txt");

	if (file.is_open()) {
		map<string, map<string, int>> happiness;
		vector<string> seats;

		ingestData(file, happiness);
		addSelf(happiness);
		for (auto &p: happiness)
			seats.push_back(p.first);

		optimizeHappiness(seats, happiness, 10000);
		printOverview(seats, happiness);

	} else {
		cout << "Unable to open file!" << endl;
	}
	return 0;
}

void optimizeHappiness(vector<string> &seats, map<string, map<string, int>> &happiness, int maxCount) {
	int cost = computeHappiness(seats, happiness), newCost, nSeats = seats.size();
	vector<string> newSeats;
	newSeats.assign(seats.begin(), seats.end());

	while (maxCount > 0) {
		randomSwap(newSeats, nSeats);
		newCost = computeHappiness(newSeats, happiness);
		if (cost > newCost) {
			newSeats.assign(seats.begin(), seats.end());
		} else {
			seats.assign(newSeats.begin(), newSeats.end());
			cost = newCost;
		}
		--maxCount;
	}
}

int computeHappiness(vector<string> &seats, map<string, map<string, int>> &happiness) {
	int cost = 0, size = seats.size(), i = 0;
	string tmp1, tmp2, tmp3;
	while (i < size) {
		tmp1 = seats[i == 0 ? size - 1: i - 1];
		tmp2 = seats[i];
		tmp3 = seats[i == size - 1 ? 0 : i + 1];
		cost += happiness[tmp2][tmp1] + happiness[tmp2][tmp3];
		++i;
	}
	return cost;
}

void ingestData(ifstream &file, map<string, map<string, int>> &happiness) {
	string line, token, name1, name2;
	stringstream stream;
	int count, score, direction;
	while (getline(file, line)) {
		stream.clear();
		stream << line;
		count = 0;
		while (getline(stream, token, ' ')) {
			if (count == 0)
				name1 = token;
			else if (count == 2)
				direction = token == "gain" ? 1: -1;
			else if (count == 3)
				score = stoi(token);
			else if (count == 10) {
				token.pop_back();
				name2 = token;
			}
			++count;
		}
		happiness[name1][name2] = score * direction;
	}
}

void printOverview(vector<string> &seats, map<string, map<string, int>> &happiness) {
	for (auto p: seats)
		cout << p << " ";
	cout << endl;
	cout << "Happiness Score: " << computeHappiness(seats, happiness) << endl;
}

void randomSwap(vector<string> &seats, int nSize) {
	int p1 = rand() % nSize;
	int p2 = rand() % nSize;
	string tmp = seats[p1];
	seats[p1] = seats[p2];
	seats[p2] = tmp;
}
void addSelf(map<string, map<string, int>> &happiness) {
	string me = "Self";
	for (auto &p: happiness)
		happiness[me][p.first] = happiness[p.first][me] = 0;
}
