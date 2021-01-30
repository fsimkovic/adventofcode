
#include <iostream>
#include <string>
using namespace std;

string lookAndSay(string &input);

int main() {
	string data = "1113122113";
	for (int i=0; i<50; ++i)
		data = lookAndSay(data);
	cout << "Final Length: " << data.size() << endl;

	return 0;
}

string lookAndSay(string &input) {
	string output = "";
	int i = 0, c, l = input.length();
	char current;
	while (i<l) {
		c = 0;
		current = input[i];
		while ((current == input[i]) && (i<l)) {
				++c;
				++i;
		}
		output += to_string(c) + current;
	}
	return output;
}
