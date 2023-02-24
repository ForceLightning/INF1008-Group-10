#include <stdio.h>
#include <iostream>
#include "StackADT.h"
#include "StackADT.cpp"
#include <string>
#include <cstdlib>
#include <ctime>
#include <iomanip>

enum TestCase {
	TC_NOTSELECTED,
	TC_AUTOGEN,
	TC_MANUAL,
};

enum DataType {
	DT_NOTSELECTED,
	DT_INT,
	DT_FLOAT,
	DT_DOUBLE,
	DT_CHAR,
	DT_STRING
};

int main(void) {

	//seeding
	srand(time(0));

	//void* theStack;
	StackADT<int>* intStack = NULL;
	StackADT<float>* floatStack = NULL;
	StackADT<double>* doubleStack = NULL;
	StackADT<char>* charStack = NULL;
	StackADT<std::string>* stringStack = NULL;
	bool mainLoop = true;
	//theStack = charStack;
	while (mainLoop) {
		TestCase testCase = TC_NOTSELECTED;
		DataType dataType = DT_NOTSELECTED;
		std::string option = "";
		int stackSize = 0; //to determine the size of stack to init

		std::cout << "This is a demo of the stack ADT with queue ADT as the base. What would you like to do?" << std::endl;
		std::cout << "1) Auto generate 10 stacks of random length(1-10), random amount of data(1-10) and data." << std::endl;
		std::cout << "2) Manually input test cases." << std::endl;
		std::cout << "Enter $ if you wish to stop the process." << std::endl;

		//giving user options to showcase algorithm*************************************
		while (testCase == TC_NOTSELECTED) {
			std::getline(std::cin, option);
			if (option == "$") {
				mainLoop = false;
				break;
			}

			//just take the 1st char in the string then set the option
			switch (option[0]) {
			case '1':
				testCase = TC_AUTOGEN;
				break;
			case '2':
				testCase = TC_MANUAL;
				break;
			default:
				std::cout << "Please enter a valid option." << std::endl;
				testCase = TC_NOTSELECTED;//safety
				break;
			}
			option = ""; //clear string, safety

		}
		if (!mainLoop) {
			break;
		}

		//data type option ****************************************************************
		std::cout << "What data type would you like to use?" << std::endl;
		std::cout << "1) int" << std::endl;
		std::cout << "2) float" << std::endl;
		std::cout << "3) double" << std::endl;
		std::cout << "4) char" << std::endl;
		std::cout << "5) string" << std::endl;

		while (dataType == DT_NOTSELECTED) {
			std::getline(std::cin, option);

			//just take the 1st char in the string then set the option
			switch (option[0]) {
			case '1':
				dataType = DT_INT;
				break;
			case '2':
				dataType = DT_FLOAT;
				break;
			case '3':
				dataType = DT_DOUBLE;
				break;
			case '4':
				dataType = DT_CHAR;
				break;
			case '5':
				dataType = DT_STRING;
				break;
			default:
				std::cout << "Invalid input. Please enter a valid option." << std::endl;
				dataType = DT_NOTSELECTED;//safety
				break;
			}
			option = ""; //clear string, safety

		}

		//auto generate test case
		if (testCase == TC_AUTOGEN) {

			int numtest = 10;
			for (int i = 0; i < numtest; i++) {
				int stackSize = rand() % 10;
				int numberOfData = rand() % 10;

				switch (dataType) {
				case DT_INT:
					intStack = new StackADT<int>(stackSize);
					std::cout << "Int Stack " + std::to_string(i) + ", Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(numberOfData) << std::endl;
					for (int j = 0; j < numberOfData; j++) {
						int data = rand();
						intStack->push(data);
					}
					intStack->printStack();
					break;
				case DT_FLOAT:
					floatStack = new StackADT<float>(stackSize);
					std::cout << "Float Stack " + std::to_string(i) + ", Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(numberOfData) << std::endl;
					for (int j = 0; j < numberOfData; j++) {
						float data = (float)(rand() % RAND_MAX) / RAND_MAX;
						floatStack->push(data);
					}
					floatStack->printStack();
					break;
				case DT_DOUBLE:
					doubleStack = new StackADT<double>(stackSize);
					std::cout << "Double Stack " + std::to_string(i) + ", Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(numberOfData) << std::endl;
					for (int j = 0; j < numberOfData; j++) {
						double data = (double)(rand() % RAND_MAX) / RAND_MAX;
						doubleStack->push(data);
					}
					doubleStack->printStack();
					break;
				case DT_CHAR:
					charStack = new StackADT<char>(stackSize);
					std::cout << "Char Stack " + std::to_string(i) + ", Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(numberOfData) << std::endl;
					for (int j = 0; j < numberOfData; j++) {
						int number = rand() % 2;
						char data;
						if (number == 0) { //generate upper case
							number = rand() % 26 + 65;
						}
						else { //generate lower case
							number = rand() % 26 + 97;
						}
						data = (char)(number);
						charStack->push(data);
					}
					charStack->printStack();
					break;
				case DT_STRING:
					stringStack = new StackADT<std::string>(stackSize);
					std::cout << "String Stack " + std::to_string(i) + ", Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(numberOfData) << std::endl;
					for (int j = 0; j < numberOfData; j++) {
						int stringLen = rand() % 10;
						std::string data = "";
						//data.append
						for (int k = 0; k < stringLen; k++) {
							int number = rand() % 2;
							char letter;
							if (number == 0) { //generate upper case
								number = rand() % 26 + 65;
							}
							else { //generate lower case
								number = rand() % 26 + 97;
							}
							letter = (char)(number);
							data += letter;
						}
						stringStack->push(data);
					}
					stringStack->printStack();
					break;
				default:
					//reset loop?
					break;
				}
				//formatting
				std::cout << std::endl;

				if (intStack) {
					delete intStack;
					intStack = NULL;
				}
				if (charStack) {
					delete charStack;
					charStack = NULL;
				}
				if (floatStack) {
					delete floatStack;
					floatStack = NULL;
				}
				if (doubleStack) {
					delete doubleStack;
					doubleStack = NULL;
				}
				if (stringStack) {
					delete stringStack;
					stringStack = NULL;
				}

			}
		}
		else if (testCase == TC_MANUAL) {
			std::cout << "Please enter the size you wish the list to be." << std::endl;
			while (true) {
				//check if user input was an int, if yes stack size determined
				if (std::cin >> stackSize) {
					break;
				}
				std::cout << "Invalid input. Please enter an integer." << std::endl;
				//clear stream safety
				std::cin.clear();
				std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
			}
			//user test loop
			std::string input = "";
			bool activeLoop = true;
			bool validInput = false;
			switch (dataType) {
			case DT_INT:
				intStack = new StackADT<int>(stackSize);
				//loop that runs to accept user input
				while (activeLoop) {
					std::cout << "Int Stack Testing, Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(intStack->getSize()) << std::endl;
					std::cout << "Enter $ if you wish to stop the process." << std::endl;
					int data = 0;
					//validation, looping till get a valid input
					validInput = false;
					while (!validInput) {
						std::getline(std::cin, input);
						std::cin.clear();
						if (input.empty()) //skip empty entries
							continue;
						if (input == "$") { //to break out of the loop
							activeLoop = false;
							break;
						}
						try {
							data = std::stoi(input);
							validInput = true;
						}
						catch (const std::invalid_argument& e) {
							std::cout << "Please enter a valid integer" << std::endl;
							validInput = false;
						}
					}
					//to skip the pushing if the user wants to exit
					if (!activeLoop) {
						break;
					}
					intStack->push(data);
					intStack->printStack();
				}
				break;
			case DT_FLOAT:
				floatStack = new StackADT<float>(stackSize);
				//loop that runs to accept user input
				while (activeLoop) {
					std::cout << "Float Stack Testing, Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(floatStack->getSize()) << std::endl;
					std::cout << "Enter $ if you wish to stop the process." << std::endl;
					float data = 0;
					validInput = false;
					//validation, looping till get a valid input
					while (!validInput) {
						std::getline(std::cin, input);
						std::cin.clear();
						if (input.empty()) //skip empty entries
							continue;
						if (input == "$") {
							activeLoop = false;
							break;
						}
						try {
							data = std::stof(input);
							validInput = true;
						}
						catch (const std::invalid_argument& e) {
							std::cout << "Please enter a valid float" << std::endl;
							validInput = false;
						}
					}
					//to skip the pushing if the user wants to exit
					if (!activeLoop) {
						break;
					}
					floatStack->push(data);
					floatStack->printStack();
				}
				break;
			case DT_DOUBLE:
				doubleStack = new StackADT<double>(stackSize);
				//loop that runs to accept user input
				while (activeLoop) {
					std::cout << "Double Stack Testing, Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(doubleStack->getSize()) << std::endl;
					std::cout << "Enter $ if you wish to stop the process." << std::endl;
					double data = 0;
					validInput = false;
					//validation, looping till get a valid input
					while (!validInput) {
						std::getline(std::cin, input);
						std::cin.clear();
						if (input.empty()) //skip empty entries
							continue;
						if (input == "$") {
							activeLoop = false;
							break;
						}
						try {
							data = std::stod(input);
							validInput = true;
						}
						catch (const std::invalid_argument& e) {
							std::cout << "Please enter a valid double" << std::endl;
							validInput = false;
						}
					}
					//to skip the pushing if the user wants to exit
					if (!activeLoop) {
						break;
					}
					doubleStack->push(data);
					doubleStack->printStack();
				}
				break;
			case DT_CHAR:
				charStack = new StackADT<char>(stackSize);
				//loop that runs to accept user input
				while (activeLoop) {
					std::cout << "Char Stack Testing, Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(charStack->getSize()) << std::endl;
					std::cout << "Enter $ if you wish to stop the process." << std::endl;
					char data = ' ';
					validInput = false;
					//validation, looping till get a valid input
					while (!validInput) {
						std::getline(std::cin, input);
						std::cin.clear();
						if (input.empty()) //skip empty entries
							continue;
						if (input == "$") {
							activeLoop = false;
							break;
						}
						try {
							data = input[0];
							validInput = true;
						}
						catch (const std::invalid_argument& e) {
							std::cout << "Please enter a valid char" << std::endl;
							validInput = false;
						}
					}
					//to skip the pushing if the user wants to exit
					if (!activeLoop) {
						break;
					}
					charStack->push(data);
					charStack->printStack();
				}
				break;
			case DT_STRING:
				stringStack = new StackADT<std::string>(stackSize);
				//loop that runs to accept user input
				while (activeLoop) {
					std::cout << "String Stack Testing, Max Stack Size : " + std::to_string(stackSize)
						+ ", Number of data : " + std::to_string(stringStack->getSize()) << std::endl;
					std::cout << "Enter $ if you wish to stop the process." << std::endl;
					std::getline(std::cin, input);
					std::cin.clear();
					if (input.empty()) //skip empty entries
						continue;
					//to skip the pushing if the user wants to exit
					if (input == "$") {
						activeLoop = false;
						break;
					}

					stringStack->push(input);
					stringStack->printStack();
				}
				break;
			default:
				break;
			}

		}

		if (intStack) {
			delete intStack;
			intStack = NULL;
		}
		if (charStack) {
			delete charStack;
			charStack = NULL;
		}
		if (floatStack) {
			delete floatStack;
			floatStack = NULL;
		}
		if (doubleStack) {
			delete doubleStack;
			doubleStack = NULL;
		}
		if (stringStack) {
			delete stringStack;
			stringStack = NULL;
		}
	}
	if (intStack) {
		delete intStack;
		intStack = NULL;
	}
	if (charStack) {
		delete charStack;
		charStack = NULL;
	}
	if (floatStack) {
		delete floatStack;
		floatStack = NULL;
	}
	if (doubleStack) {
		delete doubleStack;
		doubleStack = NULL;
	}
	if (stringStack) {
		delete stringStack;
		stringStack = NULL;
	}

	return 0;
}