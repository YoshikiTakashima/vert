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
int f_gold ( char str [ ], int n ) {
  int ans = ( n * ( n + 1 ) ) / 2;
  int a_index = 0;
  int b_index = 0;
  int c_index = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( str [ i ] == 'a' ) {
      a_index = i + 1;
      ans -= min ( b_index, c_index );
    }
    else if ( str [ i ] == 'b' ) {
      b_index = i + 1;
      ans -= min ( a_index, c_index );
    }
    else {
      c_index = i + 1;
      ans -= min ( a_index, b_index );
    }
  }
  return ans;
}


int main(void) {
	char xv[] = {'c','d'};
	f_gold(xv,29);
}