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
int f_gold ( int n ) {
  int count = 0;
  for ( int curr = 19;
  ;
  curr += 9 ) {
    int sum = 0;
    for ( int x = curr;
    x > 0;
    x = x / 10 ) sum = sum + x % 10;
    if ( sum == 10 ) count ++;
    if ( count == n ) return curr;
  }
  return - 1;
}


int main(void) {
		f_gold(1);
}