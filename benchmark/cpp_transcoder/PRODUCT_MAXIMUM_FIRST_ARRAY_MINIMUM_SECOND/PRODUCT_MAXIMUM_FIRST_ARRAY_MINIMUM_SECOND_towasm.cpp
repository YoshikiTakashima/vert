#include <stdio.h>
        #include <math.h>
        #include <stdlib.h>
        #include <limits.h>
        #include <stdbool.h>
int min(int x, int y) { return (x < y)? x: y; }
        int max(int x, int y) { return (x > y)? x: y; }
        int cmpfunc (const void * a, const void * b) {return ( *(int*)a - *(int*)b );}
        int len (int arr [ ]) {return ((int) (sizeof (arr) / sizeof (arr)[0]));}
        void sort (int arr [ ], int n) {qsort (arr, n, sizeof(int), cmpfunc);}

#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr1 [ ], int arr2 [ ], int n1, int n2 ) {
  sort ( arr1, arr1 + n1 );
  sort ( arr2, arr2 + n2 );
  return arr1 [ n1 - 1 ] * arr2 [ 0 ];
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,4);
}