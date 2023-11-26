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
void f_gold ( int arr [ ], int n, int A, int B, int C ) {
  for ( int i = 0;
  i < n;
  i ++ ) arr [ i ] = A * arr [ i ] * arr [ i ] + B * arr [ i ] + C;
  int index, maximum = INT_MIN;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( maximum < arr [ i ] ) {
      index = i;
      maximum = arr [ i ];
    }
  }
  int i = 0, j = n - 1;
  int new_arr [ n ], k = 0;
  while ( i < index && j > index ) {
    if ( arr [ i ] < arr [ j ] ) new_arr [ k ++ ] = arr [ i ++ ];
    else new_arr [ k ++ ] = arr [ j -- ];
  }
  while ( i < index ) new_arr [ k ++ ] = arr [ i ++ ];
  while ( j > index ) new_arr [ k ++ ] = arr [ j -- ];
  new_arr [ n - 1 ] = maximum;
  for ( int i = 0;
  i < n;
  i ++ ) arr [ i ] = new_arr [ i ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3,4,29);;
}