#pragma once
#include "StackADT.h"

template<class T>
StackADT<T>::StackADT(){
	queue = new QueueADT<T>(0);
}

template<class T>
StackADT<T>::StackADT(int size){
	queue = new QueueADT<T>(size);
}

template<class T>
StackADT<T>::~StackADT(){
	//if queue was init, delete
	if (queue) {
		delete queue;
		queue = NULL;
	}
}

template<class T>
void StackADT<T>::setSize(int size){
	queue->setSize(size);
}

template<class T>
int StackADT<T>::getSize(){
	return queue->getSize();
}

template<class T>
int StackADT<T>::getMaxSize() {
	return queue->getMaxSize();
}

template<class T>
void StackADT<T>::push(T data){
	// if list is full return out
	if (isFull()) {
		std::cout << "List is full, unable to push in " << data << std::endl;
		return;
	}
	
	//push the new data on, 
	queue->push(data);
	//iterate through pop and re-push everything in to 
	//push the new data to the top
	if (queue->getSize() > 1) {
		for (int i = 0; i < queue->getSize()-1; ++i) {
			T oldData = queue->pop()->data;
			queue->push(oldData);
		}
	}
}

template<class T>
Node<T>* StackADT<T>::pop()
{
	//get front of queue / top of stack
	//Node<T>* iter = queue->pop();
	return queue->pop();
}

template<class T>
bool StackADT<T>::isEmpty()
{
	return queue->isEmpty();
}

template<class T>
bool StackADT<T>::isFull()
{
	return queue->isFull();;
}

template<class T>
void StackADT<T>::printStack()
{
	queue->printQueue();
}
