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
  int res = INT_MIN;
  for ( int i = 0;
  i < n;
  i ++ ) {
    int prefix_sum = arr [ i ];
    for ( int j = 0;
    j < i;
    j ++ ) prefix_sum += arr [ j ];
    int suffix_sum = arr [ i ];
    for ( int j = n - 1;
    j > i;
    j -- ) suffix_sum += arr [ j ];
    if ( prefix_sum == suffix_sum ) res = max ( res, prefix_sum );
  }
  return res;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}