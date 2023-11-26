
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int temp [ n ];
  for ( int i = 0;
  i < n;
  i ++ ) temp [ i ] = arr [ i ];
  sort ( temp, temp + n );
  int front;
  for ( front = 0;
  front < n;
  front ++ ) if ( temp [ front ] != arr [ front ] ) break;
  int back;
  for ( back = n - 1;
  back >= 0;
  back -- ) if ( temp [ back ] != arr [ back ] ) break;
  if ( front >= back ) return 1;
  do {
    front ++;
    if ( arr [ front - 1 ] < arr [ front ] ) return 0;
  }
  while ( front != back );
  return 1;
}


