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
int f_gold ( int coins [ ], int m, int V ) {
  int table [ V + 1 ];
  table [ 0 ] = 0;
  for ( int i = 1;
  i <= V;
  i ++ ) table [ i ] = INT_MAX;
  for ( int i = 1;
  i <= V;
  i ++ ) {
    for ( int j = 0;
    j < m;
    j ++ ) if ( coins [ j ] <= i ) {
      int sub_res = table [ i - coins [ j ] ];
      if ( sub_res != INT_MAX && sub_res + 1 < table [ i ] ) table [ i ] = sub_res + 1;
    }
  }
  return table [ V ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}