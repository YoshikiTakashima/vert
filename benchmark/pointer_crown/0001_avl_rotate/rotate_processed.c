#include<stdio.h>
#include<stdlib.h>


struct Node
{
    int key;
    struct Node *left;
    struct Node *right;
    int height;
};




int max(int a, int b);
 
int height(struct Node *N)
{
    if (N == NULL)
        return 0;
    return N->height;
}

struct Node *rightRotate(struct Node *y)
{
    struct Node *x = y->left;
    struct Node *T2 = x->right;
 
    y->left = T2;
    y->height = max(height(y->left), height(y->right))+1;
    x->right = y;
    x->height = max(height(x->left), height(x->right))+1;
 
    return x;
}
 
int getBalance(struct Node *N)
{
    if (N == NULL)
        return 0;
    return height(N->left) - height(N->right);
}

struct Node *leftRotate(struct Node *x)
{
    struct Node *y = x->right;
    struct Node *T2 = y->left;
 
    x->right = T2;
    x->height = max(height(x->left), height(x->right))+1;
    y->left = x;
    y->height = max(height(y->left), height(y->right))+1;
 
    return y;
}

struct Node* insert(struct Node* node, int key)
{
    if (node == NULL)
        return(newNode(key));
 
    if (key < node->key)
        node->left  = insert(node->left, key);
    else if (key > node->key)
        node->right = insert(node->right, key);
    else // Equal keys not allowed
        return node;
 
    node->height = 1 + max(height(node->left),
                           height(node->right));
 
    int balance = getBalance(node);
 
 
    if (balance > 1 && key < node->left->key)
        return rightRotate(node);
 
    if (balance < -1 && key > node->right->key)
        return leftRotate(node);
 
    if (balance > 1 && key > node->left->key)
    {
        node->left =  leftRotate(node->left);
        return rightRotate(node);
    }
 
    if (balance < -1 && key < node->right->key)
    {
        node->right = rightRotate(node->right);
        return leftRotate(node);
    }
 
    return node;
}
