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
int f_gold ( char str [ ] ) {
  int len = strlen ( str );
  if ( str [ 0 ] < 'A' || str [ 0 ] > 'Z' ) return 0;
  if ( str [ len - 1 ] != '.' ) return 0;
  int prev_state = 0, curr_state = 0;
  int index = 1;
  while ( str [ index ] ) {
    if ( str [ index ] >= 'A' && str [ index ] <= 'Z' ) curr_state = 0;
    else if ( str [ index ] == ' ' ) curr_state = 1;
    else if ( str [ index ] >= 'a' && str [ index ] <= 'z' ) curr_state = 2;
    else if ( str [ index ] == '.' ) curr_state = 3;
    if ( prev_state == curr_state && curr_state != 2 ) return 0;
    if ( prev_state == 2 && curr_state == 0 ) return 0;
    if ( curr_state == 3 && prev_state != 1 ) return ( str [ index + 1 ] == '\0' );
    index ++;
    prev_state = curr_state;
  }
  return 0;
}


int main(void) {
	char xv[] = {'c','d'};
	f_gold(xv);
}