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
int f_gold ( int arr1 [ ], int arr2 [ ], int m, int n ) {
  int i = 0, j = 0;
  if ( m < n ) return 0;
  sort ( arr1, arr1 + m );
  sort ( arr2, arr2 + n );
  while ( i < n && j < m ) {
    if ( arr1 [ j ] < arr2 [ i ] ) j ++;
    else if ( arr1 [ j ] == arr2 [ i ] ) {
      j ++;
      i ++;
    }
    else if ( arr1 [ j ] > arr2 [ i ] ) return 0;
  }
  return ( i < n ) ? 0 : 1;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,29);
}