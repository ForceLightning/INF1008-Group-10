#pragma once
#include "Node.h"

template <class T>
Node<T>::Node() {
	next = prev = NULL;
}

template <class T>
Node<T>::Node(T data) {
	this->data = data;
	next = prev = NULL;
}

template <class T>
Node<T>::~Node() {
	/*if (data) {
		delete data;
		data = NULL;
	}*/
}