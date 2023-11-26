
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int A [ ], int arr_size, int sum ) {
  int l, r;
  for ( int i = 0;
  i < arr_size - 2;
  i ++ ) {
    for ( int j = i + 1;
    j < arr_size - 1;
    j ++ ) {
      for ( int k = j + 1;
      k < arr_size;
      k ++ ) {
        if ( A [ i ] + A [ j ] + A [ k ] == sum ) {
          cout << "Triplet is " << A [ i ] << ", " << A [ j ] << ", " << A [ k ];
          return 1;
        }
      }
    }
  }
  return 0;
}


