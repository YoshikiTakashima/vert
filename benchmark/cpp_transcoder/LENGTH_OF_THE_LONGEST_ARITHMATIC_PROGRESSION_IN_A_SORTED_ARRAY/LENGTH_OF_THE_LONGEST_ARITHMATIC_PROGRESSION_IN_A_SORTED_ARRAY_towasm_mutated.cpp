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
int f_gold ( int set [ ], int n ) {
  if ( n <= 2 ) return n;
  int L [ n ] [ n ];
  int llap = 2;
  for ( int i = 0;
  i < n;
  i ++ ) L [ i ] [ n - 1 ] = 2;
  for ( int j = n - 2;
  j >= 1;
  j -- ) {
    int i = j - 1, k = j + 1;
    while ( i >= 0 && k <= n - 1 ) {
      if ( set [ i ] + set [ k ] < 2 * set [ j ] ) k ++;
      else if ( set [ i ] + set [ k ] > 2 * set [ j ] ) {
        L [ i ] [ j ] = 2, i --;
      }
      else {
        L [ i ] [ j ] = L [ j ] [ k ] + 1;
        llap = max ( llap, L [ i ] [ j ] );
        i --;
        k ++;
      }
    }
    while ( i >= 0 ) {
      L [ i ] [ j ] = 2;
      i --;
    }
  }
  return llap;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}