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
string f_gold ( string number, int divisor ) {
  string ans;
  int idx = 0;
  int temp = number [ idx ] - '0';
  while ( temp < divisor ) temp = temp * 10 + ( number [ ++ idx ] - '0' );
  while ( number . size ( ) > idx ) {
    ans += ( temp / divisor ) + '0';
    temp = ( temp % divisor ) * 10 + number [ ++ idx ] - '0';
  }
  if ( ans . length ( ) == 0 ) return "0";
  return ans;
}


int main(void) {
		f_gold("ab",2);
}