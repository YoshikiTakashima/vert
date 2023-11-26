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
int f_gold ( char str [ ], int len ) {
  int count = 1;
  if ( len == 1 ) return count;
  if ( str [ 0 ] == str [ 1 ] ) count *= 1;
  else count *= 2;
  for ( int j = 1;
  j < len - 1;
  j ++ ) {
    if ( str [ j ] == str [ j - 1 ] && str [ j ] == str [ j + 1 ] ) count *= 1;
    else if ( str [ j ] == str [ j - 1 ] || str [ j ] == str [ j + 1 ] || str [ j - 1 ] == str [ j + 1 ] ) count *= 2;
    else count *= 3;
  }
  if ( str [ len - 1 ] == str [ len - 2 ] ) count *= 1;
  else count *= 2;
  return count;
}


int main(void) {
	char xv[] = {'c','d'};
	f_gold(xv,29);
}