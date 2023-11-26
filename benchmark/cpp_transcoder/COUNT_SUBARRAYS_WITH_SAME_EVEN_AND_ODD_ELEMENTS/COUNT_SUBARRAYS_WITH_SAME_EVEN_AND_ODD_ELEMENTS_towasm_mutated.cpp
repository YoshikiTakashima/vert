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
  int difference = 0;
  int ans = 0;
  int hash_positive [ n + 1 ], hash_negative [ n + 1 ];
  fill_n ( hash_positive, n + 1, 0 );
  fill_n ( hash_negative, n + 1, 0 );
  hash_positive [ 0 ] = 1;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( arr [ i ] & 1 == 1 ) difference ++;
    else difference --;
    if ( difference < 0 ) {
      ans += hash_negative [ - difference ];
      hash_negative [ - difference ] ++;
    }
    else {
      ans += hash_positive [ difference ];
      hash_positive [ difference ] ++;
    }
  }
  return ans;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}