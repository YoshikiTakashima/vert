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
  int invcount = 0;
  for ( int i = 0;
  i < n - 2;
  i ++ ) {
    for ( int j = i + 1;
    j < n - 1;
    j ++ ) {
      if ( arr [ i ] > arr [ j ] ) {
        for ( int k = j + 1;
        k < n;
        k ++ ) {
          if ( arr [ j ] > arr [ k ] ) invcount ++;
        }
      }
    }
  }
  return invcount;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}