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
  int max = arr1 [ 0 ];
  int min = arr2 [ 0 ];
  int i;
  for ( i = 1;
  i < n1 && i < n2;
  ++ i ) {
    if ( arr1 [ i ] > max ) max = arr1 [ i ];
    if ( arr2 [ i ] < min ) min = arr2 [ i ];
  }
  while ( i < n1 ) {
    if ( arr1 [ i ] > max ) max = arr1 [ i ];
    i ++;
  }
  while ( i < n2 ) {
    if ( arr2 [ i ] < min ) min = arr2 [ i ];
    i ++;
  }
  return max * min;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,4);
}