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
int f_gold ( int X [ ], int Y [ ], int m, int n ) {
  int res = 0;
  sort ( X, X + m, greater < int > ( ) );
  sort ( Y, Y + n, greater < int > ( ) );
  int hzntl = 1, vert = 1;
  int i = 0, j = 0;
  while ( i < m && j < n ) {
    if ( X [ i ] > Y [ j ] ) {
      res += X [ i ] * vert;
      hzntl ++;
      i ++;
    }
    else {
      res += Y [ j ] * hzntl;
      vert ++;
      j ++;
    }
  }
  int total = 0;
  while ( i < m ) total += X [ i ++ ];
  res += total * vert;
  total = 0;
  while ( j < n ) total += Y [ j ++ ];
  res += total * hzntl;
  return res;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,4);
}