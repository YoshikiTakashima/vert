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
int f_gold ( int m, int n ) {
  long long factorCount [ n + 1 ];
  bool prime [ n + 1 ];
  for ( int i = 0;
  i <= n;
  i ++ ) {
    factorCount [ i ] = 0;
    prime [ i ] = 1;
  }
  for ( int i = 2;
  i <= n;
  i ++ ) {
    if ( prime [ i ] == 1 ) {
      factorCount [ i ] = 1;
      for ( int j = i * 2;
      j <= n;
      j += i ) {
        factorCount [ j ] ++;
        prime [ j ] = 0;
      }
    }
  }
  int max = factorCount [ m ];
  int num = m;
  for ( int i = m;
  i <= n;
  i ++ ) {
    if ( factorCount [ i ] > max ) {
      max = factorCount [ i ];
      num = i;
    }
  }
  return num;
}


int main(void) {
		f_gold(1,2);
}