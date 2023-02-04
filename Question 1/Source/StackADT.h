#pragma once
#include "QueueADT.h"
#include "QueueADT.cpp"

template <class T>
class StackADT
{
private:
	QueueADT<T>* queue;

public:
	StackADT();
	StackADT(int size);
	~StackADT();

	void setSize(int size);
	int getSize();
	int getMaxSize();
	void push(T data);
	Node<T>* pop();
	bool isEmpty();
	bool isFull();
	void printStack();
	//Node<T>* front();
	//Node<T>* back();
};

