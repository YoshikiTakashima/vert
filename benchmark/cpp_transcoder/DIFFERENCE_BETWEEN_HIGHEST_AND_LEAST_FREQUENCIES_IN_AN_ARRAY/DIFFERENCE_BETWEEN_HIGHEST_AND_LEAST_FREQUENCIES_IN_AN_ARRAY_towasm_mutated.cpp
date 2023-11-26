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
int f_gold ( int arr [ ], int n ) {
  sort ( arr, arr + n );
  int count = 0, max_count = 0, min_count = n;
  for ( int i = 0;
  i < ( n - 1 );
  i ++ ) {
    if ( arr [ i ] == arr [ i + 1 ] ) {
      count += 1;
      continue;
    }
    else {
      max_count = max ( max_count, count );
      min_count = min ( min_count, count );
      count = 0;
    }
  }
  return ( max_count - min_count );
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}