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
long f_gold ( int n, int k ) {
  long total = k;
  int mod = 1000000007;
  int same = 0, diff = k;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    same = diff;
    diff = total * ( k - 1 );
    diff = diff % mod;
    total = ( same + diff ) % mod;
  }
  return total;
}


int main(void) {
		f_gold(1,29);
}