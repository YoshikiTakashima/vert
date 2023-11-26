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
int f_gold ( int a [ ], int size ) {
  int max_so_far = INT_MIN, max_ending_here = 0, start = 0, end = 0, s = 0;
  for ( int i = 0;
  i < size;
  i ++ ) {
    max_ending_here += a [ i ];
    if ( max_so_far < max_ending_here ) {
      max_so_far = max_ending_here;
      start = s;
      end = i;
    }
    if ( max_ending_here < 0 ) {
      max_ending_here = 0;
      s = i + 1;
    }
  }
  return ( end - start + 1 );
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}