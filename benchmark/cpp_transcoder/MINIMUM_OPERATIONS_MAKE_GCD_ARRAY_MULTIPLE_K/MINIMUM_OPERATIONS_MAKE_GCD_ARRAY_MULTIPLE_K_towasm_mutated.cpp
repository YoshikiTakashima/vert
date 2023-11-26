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
int f_gold ( int a [ ], int n, int k ) {
  int result = 0;
  for ( int i = 0;
  i < n;
  ++ i ) {
    if ( a [ i ] != 1 && a [ i ] > k ) {
      result = result + min ( a [ i ] % k, k - a [ i ] % k );
    }
    else {
      result = result + k - a [ i ];
    }
  }
  return result;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}