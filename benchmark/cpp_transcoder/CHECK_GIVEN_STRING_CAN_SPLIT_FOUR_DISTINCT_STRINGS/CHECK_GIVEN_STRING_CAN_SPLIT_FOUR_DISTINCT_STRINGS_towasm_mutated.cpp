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
int f_gold ( string s ) {
  if ( s . size ( ) >= 10 ) return 1;
  for ( int i = 1;
  i < s . size ( );
  i ++ ) {
    for ( int j = i + 1;
    j < s . size ( );
    j ++ ) {
      for ( int k = j + 1;
      k < s . size ( );
      k ++ ) {
        string s1 = s . substr ( 0, i );
        string s2 = s . substr ( i, j - i );
        string s3 = s . substr ( j, k - j );
        string s4 = s . substr ( k, s . size ( ) - k );
        if ( s1 != s2 && s1 != s3 && s1 != s4 && s2 != s3 && s2 != s4 && s3 != s4 ) return 1;
      }
    }
  }
  return 0;
}


int main(void) {
		f_gold("cd");
}