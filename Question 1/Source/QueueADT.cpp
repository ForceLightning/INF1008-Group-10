#pragma once
#include "QueueADT.h"

template <class T>
QueueADT<T>::QueueADT() {
	maxSize = 0;
	currSize = 0;
	head = tail = NULL;
}

template <class T>
QueueADT<T>::QueueADT(int size) {
	maxSize = size;
	currSize = 0;
	head = tail = NULL;
}

//delete, could do the same thing as empty? 
template <class T>
QueueADT<T>::~QueueADT() {
	Node<T>* iter = head;
	//while iter exists, delete it and move next
	while (iter) {
		head = iter;
		iter = iter->next;
		delete head;
		head = NULL;
	}
	//safety
	head = tail = iter = NULL;
}
template <class T>
void QueueADT<T>::setMaxSize(int size) {
	maxSize = size;
}

template <class T>
int QueueADT<T>::getSize() {
	return currSize;
}

template <class T>
int QueueADT<T>::getMaxSize() {
	return maxSize;
}

//insert element at the end of the Queuee
template <class T>
void QueueADT<T>::push(T data) {

	// if list is full return out
	if (isFull()) {
		std::cout << "Container full, failed to push "<< data << std::endl;
		return;
	}

	Node<T>* newNode = new Node<T>(data);

	//check if list is empty, if yes assign head and tail to same node 
	if (isEmpty()) {
		head = tail = newNode;	
		++currSize;
		return;
	}

	//getting last object and linking newNode to list
	Node<T>* iter = tail;
	iter->next = newNode;
	newNode->prev = iter;
	tail = newNode;
	++currSize; //add to counter
}

//remove and return 1st element
template <class T>
Node<T>* QueueADT<T>::pop() {
	if (isEmpty()) {
		return NULL;
	}
	Node<T>* iter = head;
	if (head->next != NULL) {
		head = head->next; //move head pointer back
	}
	head->prev = NULL; //break prev link to old head node
	iter->next = NULL; //break next link from old head node
	--currSize; //minus counter
	return iter;
}

//checks if container is empty
template <class T>
bool QueueADT<T>::isEmpty() {
	//returns if head and tail are both NULL
	return ((head == NULL || tail == NULL) && currSize == 0);
}

//returns if the Queuee is full / at capacity
template <class T>
bool QueueADT<T>::isFull() {
	return currSize >= maxSize;
}

template <class T>
void QueueADT<T>::printQueue() {
	//if head or tail is not init, something wrong return out
	if (!head || !tail)
		return;

	std::cout << "Data stored : ";
	//printing data in list
	Node<T>* iter = head;
	while (iter != NULL) {
		//printf("%.6f", iter->data); //printf here to acheieve format
		std::cout << iter->data;
		//formatting
		if (iter->next != NULL)
			std::cout << ", ";
		iter = iter->next;
	}
	std::cout << std::endl;
}

template<class T>
Node<T>* QueueADT<T>::front()
{
	//if head exists return head else NULL
	if (head)
		return head;

	return NULL;
}

template<class T>
Node<T>* QueueADT<T>::back()
{
	//if tail exists return head else NULL
	if (tail)
		return tail;

	return NULL;
}

