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
string f_gold ( string seq ) {
  int n = seq . length ( );
  if ( n >= 9 ) return "-1";
  string result ( n + 1, ' ' );
  int count = 1;
  for ( int i = 0;
  i <= n;
  i ++ ) {
    if ( i == n || seq [ i ] == 'I' ) {
      for ( int j = i - 1;
      j >= - 1;
      j -- ) {
        result [ j + 1 ] = '0' + count ++;
        if ( j >= 0 && seq [ j ] == 'I' ) break;
      }
    }
  }
  return result;
}


int main(void) {
		f_gold("ab");
}