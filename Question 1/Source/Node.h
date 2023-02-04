#pragma once
#include <stdio.h>
#include <iostream>
template <class T>
class Node{
public:
	T data;
	Node<T>* next;
	Node<T>* prev;
	Node();
	Node(T data);
	~Node();
};

