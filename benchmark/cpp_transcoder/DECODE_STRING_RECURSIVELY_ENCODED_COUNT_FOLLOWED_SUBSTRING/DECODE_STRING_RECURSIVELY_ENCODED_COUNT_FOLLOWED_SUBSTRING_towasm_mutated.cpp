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
string f_gold ( string str ) {
  stack < int > integerstack;
  stack < char > stringstack;
  string temp = "", result = "";
  for ( int i = 0;
  i < str . length ( );
  i ++ ) {
    int count = 0;
    if ( str [ i ] >= '0' && str [ i ] <= '9' ) {
      while ( str [ i ] >= '0' && str [ i ] <= '9' ) {
        count = count * 10 + str [ i ] - '0';
        i ++;
      }
      i --;
      integerstack . push ( count );
    }
    else if ( str [ i ] == ']' ) {
      temp = "";
      count = 0;
      if ( ! integerstack . empty ( ) ) {
        count = integerstack . top ( );
        integerstack . pop ( );
      }
      while ( ! stringstack . empty ( ) && stringstack . top ( ) != '[' ) {
        temp = stringstack . top ( ) + temp;
        stringstack . pop ( );
      }
      if ( ! stringstack . empty ( ) && stringstack . top ( ) == '[' ) stringstack . pop ( );
      for ( int j = 0;
      j < count;
      j ++ ) result = result + temp;
      for ( int j = 0;
      j < result . length ( );
      j ++ ) stringstack . push ( result [ j ] );
      result = "";
    }
    else if ( str [ i ] == '[' ) {
      if ( str [ i - 1 ] >= '0' && str [ i - 1 ] <= '9' ) stringstack . push ( str [ i ] );
      else {
        stringstack . push ( str [ i ] );
        integerstack . push ( 1 );
      }
    }
    else stringstack . push ( str [ i ] );
  }
  while ( ! stringstack . empty ( ) ) {
    result = stringstack . top ( ) + result;
    stringstack . pop ( );
  }
  return result;
}


int main(void) {
		f_gold("cd");
}