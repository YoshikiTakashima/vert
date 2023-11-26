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
  int * profit = new int [ n ];
  for ( int i = 0;
  i < n;
  i ++ ) profit [ i ] = 0;
  int max_price = price [ n - 1 ];
  for ( int i = n - 2;
  i >= 0;
  i -- ) {
    if ( price [ i ] > max_price ) max_price = price [ i ];
    profit [ i ] = max ( profit [ i + 1 ], max_price - price [ i ] );
  }
  int min_price = price [ 0 ];
  for ( int i = 1;
  i < n;
  i ++ ) {
    if ( price [ i ] < min_price ) min_price = price [ i ];
    profit [ i ] = max ( profit [ i - 1 ], profit [ i ] + ( price [ i ] - min_price ) );
  }
  int result = profit [ n - 1 ];
  delete [ ] profit;
  return result;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}