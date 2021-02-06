
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
using namespace std;

class Reindeer {
	private:
	const string name;
	const int speed;
	const int duration;
	const int cycleRate;

	int distance = 0;
	int seconds = 0;
	int score = 0;

	public:
	Reindeer(string name, int speed, int duration, int sleep): name(name), speed(speed), duration(duration), cycleRate(duration+sleep) {};
	void move() {
		if (seconds % cycleRate < duration)
			distance += speed;
		seconds++;
	}
	void awardPoint() {
		++score;
	}
	int getScore() {
		return score;
	}
	int getDistance() {
		return distance;
	}
	string getName() {
		return name;
	}

};


void ingestData(ifstream &file, vector<Reindeer> &reindeers);


int main() {
	ifstream file;
	file.open("input1.txt");
	if (file.is_open()) {
		vector<Reindeer> reindeers;
		ingestData(file, reindeers);

		int dist = 0;
		for (int i=0; i<2503; i++) {
			for (auto &r: reindeers) {
				r.move();
				dist = max(dist, r.getDistance());
			}
			for (auto &r: reindeers)
				if (r.getDistance() == dist)
					r.awardPoint();
		}
		
		for (auto &r: reindeers)
			cout << r.getName() << ": " << r.getScore() << endl;

	} else {
		cout << "Unable to open file!" << endl;
	}
	return 0;
}

void ingestData(ifstream &file, vector<Reindeer> &reindeers) {
	string line, token, name;
	stringstream stream;
	int count, speed, duration, sleep;
	while (getline(file, line)) {
		stream.clear();
		stream << line;
		count = 0;
		while (getline(stream, token, ' ')) {
			if (count == 0)
				name = token;
			else if (count == 3)
				speed = stoi(token);
			else if (count == 6)
				duration = stoi(token);
			else if (count == 13)
				sleep = stoi(token);
			++count;
		}
		Reindeer reindeer = Reindeer(name, speed, duration, sleep);
		reindeers.push_back(reindeer);
	}
}
