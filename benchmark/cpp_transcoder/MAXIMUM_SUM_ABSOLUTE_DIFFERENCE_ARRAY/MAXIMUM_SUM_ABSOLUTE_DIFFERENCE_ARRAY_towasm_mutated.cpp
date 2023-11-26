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
int f_gold ( int a [ ], int n ) {
  vector < int > finalSequence;
  sort ( a, a + n );
  for ( int i = 0;
  i < n / 2;
  ++ i ) {
    finalSequence . push_back ( a [ i ] );
    finalSequence . push_back ( a [ n - i - 1 ] );
  }
  int MaximumSum = 0;
  for ( int i = 0;
  i < n - 1;
  ++ i ) {
    MaximumSum = MaximumSum + abs ( finalSequence [ i ] - finalSequence [ i + 1 ] );
  }
  MaximumSum = MaximumSum + abs ( finalSequence [ n - 1 ] - finalSequence [ 0 ] );
  return MaximumSum;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}