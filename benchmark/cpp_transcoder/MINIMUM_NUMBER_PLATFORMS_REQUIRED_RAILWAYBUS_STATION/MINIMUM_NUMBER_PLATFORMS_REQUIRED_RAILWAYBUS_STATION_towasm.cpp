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
int f_gold ( int arr [ ], int dep [ ], int n ) {
  sort ( arr, arr + n );
  sort ( dep, dep + n );
  int plat_needed = 1, result = 1;
  int i = 1, j = 0;
  while ( i < n && j < n ) {
    if ( arr [ i ] <= dep [ j ] ) {
      plat_needed ++;
      i ++;
      if ( plat_needed > result ) result = plat_needed;
    }
    else {
      plat_needed --;
      j ++;
    }
  }
  return result;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3);
}