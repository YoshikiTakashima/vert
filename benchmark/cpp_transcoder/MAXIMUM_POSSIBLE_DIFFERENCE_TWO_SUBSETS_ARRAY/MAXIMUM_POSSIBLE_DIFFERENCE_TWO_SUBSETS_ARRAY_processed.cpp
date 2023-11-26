
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int arr [ ], int n ) {
  int SubsetSum_1 = 0, SubsetSum_2 = 0;
  for ( int i = 0;
  i <= n - 1;
  i ++ ) {
    bool isSingleOccurance = 1;
    for ( int j = i + 1;
    j <= n - 1;
    j ++ ) {
      if ( arr [ i ] == arr [ j ] ) {
        isSingleOccurance = 0;
        arr [ i ] = arr [ j ] = 0;
        break;
      }
    }
    if ( isSingleOccurance ) {
      if ( arr [ i ] > 0 ) SubsetSum_1 += arr [ i ];
      else SubsetSum_2 += arr [ i ];
    }
  }
  return abs ( SubsetSum_1 - SubsetSum_2 );
}


