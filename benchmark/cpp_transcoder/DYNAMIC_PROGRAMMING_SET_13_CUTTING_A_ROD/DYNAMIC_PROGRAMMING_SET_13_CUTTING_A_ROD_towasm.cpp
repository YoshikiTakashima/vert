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
int f_gold ( int price [ ], int n ) {
  int val [ n + 1 ];
  val [ 0 ] = 0;
  int i, j;
  for ( i = 1;
  i <= n;
  i ++ ) {
    int max_val = INT_MIN;
    for ( j = 0;
    j < i;
    j ++ ) max_val = max ( max_val, price [ j ] + val [ i - j - 1 ] );
    val [ i ] = max_val;
  }
  return val [ n ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}