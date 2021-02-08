#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include "Lib.h"

using namespace std;

int calcScore(int i, int j, int k, int l, vector<Ingredient> &ingredients);

int main() {
	ifstream file;
	if (openFile(file) == 1)
		return 1;

	vector<Ingredient> ingredients;
	int capacity = 100;

	readFile(file, ingredients);

	int i, j, k, l, score = 0;
	for (i = 0; i <= capacity; i++) {
		for (j = 0; j <= capacity-i; j++) {
			for (k = 0; k <= capacity-i-j; k++) {
				l = capacity - i - j - k;
				score = max(score, calcScore(i, j, k, l, ingredients));
			}
		}
	}
	cout << "Score: " << score << endl;

	
	return 0;
}

int calcScore(int i, int j, int k, int l, vector<Ingredient> &ingredients) {
	int calories	= ingredients[0].calories * i 	+ ingredients[1].calories * j 	+ ingredients[2].calories * k 	+ ingredients[3].calories * l;
	if (calories != 500) return 0;

	int capacity   	= ingredients[0].capacity * i 	+ ingredients[1].capacity * j 	+ ingredients[2].capacity * k 	+ ingredients[3].capacity * l;
	int durability 	= ingredients[0].durability * i + ingredients[1].durability * j + ingredients[2].durability * k + ingredients[3].durability * l;
	int flavor 	= ingredients[0].flavor * i 	+ ingredients[1].flavor * j 	+ ingredients[2].flavor * k 	+ ingredients[3].flavor * l;
	int texture 	= ingredients[0].texture * i 	+ ingredients[1].texture * j 	+ ingredients[2].texture * k 	+ ingredients[3].texture * l;
	return max(0, capacity) * max(0, durability) * max(0, flavor)  * max(0, texture);
}
