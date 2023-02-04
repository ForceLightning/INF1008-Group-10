#pragma once
#include "Node.h"
#include "Node.cpp"

template <class T>
class QueueADT
{
private:
	int maxSize;
	int currSize;
	Node<T>* head;
	Node<T>* tail;

public:
	QueueADT();
	QueueADT(int size);
	~QueueADT();

	void setSize(int size);
	int getSize();
	int getMaxSize();
	void push(T data);
	Node<T>* pop();
	bool isEmpty();
	bool isFull();
	void printQueue();
	Node<T>* front();
	Node<T>* back();

};

