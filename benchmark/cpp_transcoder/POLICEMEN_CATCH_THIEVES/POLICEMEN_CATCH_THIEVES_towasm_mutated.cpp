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
int f_gold ( char arr [ ], int n, int k ) {
  int res = 0;
  vector < int > thi;
  vector < int > pol;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( arr [ i ] == 'P' ) pol . push_back ( i );
    else if ( arr [ i ] == 'T' ) thi . push_back ( i );
  }
  int l = 0, r = 0;
  while ( l < thi . size ( ) && r < pol . size ( ) ) {
    if ( abs ( thi [ l ] - pol [ r ] ) <= k ) {
      res ++;
      l ++;
      r ++;
    }
    else if ( thi [ l ] < pol [ r ] ) l ++;
    else r ++;
  }
  return res;
}


int main(void) {
	char xv[] = {'c','d'};
	f_gold(xv,2,29);
}