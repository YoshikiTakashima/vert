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
int f_gold ( int n ) {
  if ( n <= 0 ) return 0;
  int fibo [ n + 1 ];
  fibo [ 0 ] = 0, fibo [ 1 ] = 1;
  int sum = fibo [ 0 ] + fibo [ 1 ];
  for ( int i = 2;
  i <= n;
  i ++ ) {
    fibo [ i ] = fibo [ i - 1 ] + fibo [ i - 2 ];
    sum += fibo [ i ];
  }
  return sum;
}


int main(void) {
		f_gold(29);
}