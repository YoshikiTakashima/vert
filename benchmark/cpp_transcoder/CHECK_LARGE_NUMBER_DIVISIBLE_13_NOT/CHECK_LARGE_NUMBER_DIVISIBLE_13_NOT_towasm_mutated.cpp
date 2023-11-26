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
int f_gold ( string num ) {
  int length = num . size ( );
  if ( length == 1 && num [ 0 ] == '0' ) return 1;
  if ( length % 3 == 1 ) {
    num += "00";
    length += 2;
  }
  else if ( length % 3 == 2 ) {
    num += "0";
    length += 1;
  }
  int sum = 0, p = 1;
  for ( int i = length - 1;
  i >= 0;
  i -- ) {
    int group = 0;
    group += num [ i -- ] - '0';
    group += ( num [ i -- ] - '0' ) * 10;
    group += ( num [ i ] - '0' ) * 100;
    sum = sum + group * p;
    p *= ( - 1 );
  }
  sum = abs ( sum );
  return ( sum % 13 == 0 );
}


int main(void) {
		f_gold("cd");
}