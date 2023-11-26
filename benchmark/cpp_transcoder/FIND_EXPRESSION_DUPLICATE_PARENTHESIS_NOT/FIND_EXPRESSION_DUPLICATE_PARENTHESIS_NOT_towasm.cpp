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
int f_gold ( string str ) {
  stack < char > Stack;
  for ( char ch : str ) {
    if ( ch == ')' ) {
      char top = Stack . top ( );
      Stack . pop ( );
      int elementsInside = 0;
      while ( top != '(' ) {
        elementsInside ++;
        top = Stack . top ( );
        Stack . pop ( );
      }
      if ( elementsInside < 1 ) {
        return 1;
      }
    }
    else Stack . push ( ch );
  }
  return 0;
}


int main(void) {
		f_gold("ab");
}