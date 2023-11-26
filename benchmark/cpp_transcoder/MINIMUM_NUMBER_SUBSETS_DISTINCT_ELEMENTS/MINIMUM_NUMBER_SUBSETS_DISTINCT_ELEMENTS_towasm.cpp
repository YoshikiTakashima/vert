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
int f_gold ( int ar [ ], int n ) {
  int res = 0;
  sort ( ar, ar + n );
  for ( int i = 0;
  i < n;
  i ++ ) {
    int count = 1;
    for (;
    i < n - 1;
    i ++ ) {
      if ( ar [ i ] == ar [ i + 1 ] ) count ++;
      else break;
    }
    res = max ( res, count );
  }
  return res;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}