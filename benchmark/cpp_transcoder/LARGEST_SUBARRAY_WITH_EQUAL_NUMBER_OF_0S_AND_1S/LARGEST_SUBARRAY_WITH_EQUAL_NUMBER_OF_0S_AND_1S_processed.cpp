
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int sum = 0;
  int maxsize = - 1, startindex;
  for ( int i = 0;
  i < n - 1;
  i ++ ) {
    sum = ( arr [ i ] == 0 ) ? - 1 : 1;
    for ( int j = i + 1;
    j < n;
    j ++ ) {
      ( arr [ j ] == 0 ) ? ( sum += - 1 ) : ( sum += 1 );
      if ( sum == 0 && maxsize < j - i + 1 ) {
        maxsize = j - i + 1;
        startindex = i;
      }
    }
  }
  if ( maxsize == - 1 ) cout << "No such subarray";
  else cout << startindex << " to " << startindex + maxsize - 1;
  return maxsize;
}


