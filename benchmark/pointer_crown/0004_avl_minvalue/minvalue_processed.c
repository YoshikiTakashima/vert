#include <stdio.h>
#include <stdlib.h>

struct node {
	int key;
	struct node *left, *right;
};

struct node* minValueNode(struct node* node)
{
	struct node* current = node;

	while (current && current->left != NULL)
		current = current->left;

	return current;
}
