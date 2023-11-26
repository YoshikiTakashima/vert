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
int f_gold ( int arr [ ], int n, int x, int y ) {
  int i, j;
  int min_dist = INT_MAX;
  for ( i = 0;
  i < n;
  i ++ ) {
    for ( j = i + 1;
    j < n;
    j ++ ) {
      if ( ( x == arr [ i ] && y == arr [ j ] || y == arr [ i ] && x == arr [ j ] ) && min_dist > abs ( i - j ) ) {
        min_dist = abs ( i - j );
      }
    }
  }
  return min_dist;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3,4);
}