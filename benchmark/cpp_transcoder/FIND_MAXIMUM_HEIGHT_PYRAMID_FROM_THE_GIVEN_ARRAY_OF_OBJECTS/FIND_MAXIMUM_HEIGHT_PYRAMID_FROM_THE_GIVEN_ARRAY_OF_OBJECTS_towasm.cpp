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
int f_gold ( int boxes [ ], int n ) {
  sort ( boxes, boxes + n );
  int ans = 1;
  int prev_width = boxes [ 0 ];
  int prev_count = 1;
  int curr_count = 0;
  int curr_width = 0;
  for ( int i = 1;
  i < n;
  i ++ ) {
    curr_width += boxes [ i ];
    curr_count += 1;
    if ( curr_width > prev_width && curr_count > prev_count ) {
      prev_width = curr_width;
      prev_count = curr_count;
      curr_count = 0;
      curr_width = 0;
      ans ++;
    }
  }
  return ans;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}