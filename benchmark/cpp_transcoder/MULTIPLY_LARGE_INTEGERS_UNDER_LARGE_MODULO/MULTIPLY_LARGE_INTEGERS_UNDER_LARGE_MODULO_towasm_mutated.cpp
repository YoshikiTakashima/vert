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
long long f_gold ( long long a, long long b, long long mod ) {
  long long res = 0;
  a %= mod;
  while ( b ) {
    if ( b & 1 ) res = ( res + a ) % mod;
    a = ( 2 * a ) % mod;
    b >>= 1;
  }
  return res;
}


int main(void) {
		f_gold(1,2,29);
}