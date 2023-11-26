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
  int halfSum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) halfSum = halfSum + arr [ i ];
  halfSum = halfSum / 2;
  sort ( arr, arr + n, greater < int > ( ) );
  int res = 0, curr_sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    curr_sum += arr [ i ];
    res ++;
    if ( curr_sum > halfSum ) return res;
  }
  return res;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}