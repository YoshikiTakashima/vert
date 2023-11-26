
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
string f_gold ( string num ) {
  int n = num . size ( );
  int rightMin [ n ], right;
  rightMin [ n - 1 ] = - 1;
  right = n - 1;
  for ( int i = n - 2;
  i >= 1;
  i -- ) {
    if ( num [ i ] >= num [ right ] ) rightMin [ i ] = right;
    else {
      if ( num [ i ] == num [ i + 1 ] ) {
        rightMin [ i ] = right;
      }
      else {
        rightMin [ i ] = - 1;
        right = i;
      }
    }
  }
  int small = - 1;
  for ( int i = 1;
  i < n;
  i ++ ) if ( num [ i ] != '0' ) {
    if ( small == - 1 ) {
      if ( num [ i ] < num [ 0 ] ) small = i;
    }
    else if ( num [ i ] <= num [ small ] ) small = i;
  }
  if ( small != - 1 ) swap ( num [ 0 ], num [ small ] );
  else {
    for ( int i = 1;
    i < n;
    i ++ ) {
      if ( rightMin [ i ] != - 1 && num [ i ] != num [ rightMin [ i ] ] ) {
        swap ( num [ i ], num [ rightMin [ i ] ] );
        break;
      }
    }
  }
  return num;
}


