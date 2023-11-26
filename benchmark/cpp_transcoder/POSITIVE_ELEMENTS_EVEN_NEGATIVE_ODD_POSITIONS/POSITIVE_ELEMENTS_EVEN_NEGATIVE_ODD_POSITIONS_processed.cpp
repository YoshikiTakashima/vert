
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
void f_gold ( int a [ ], int size ) {
  int positive = 0, negative = 1;
  while ( 1 ) {
    while ( positive < size && a [ positive ] >= 0 ) positive += 2;
    while ( negative < size && a [ negative ] <= 0 ) negative += 2;
    if ( positive < size && negative < size ) swap ( a [ positive ], a [ negative ] );
    else break;
  }
}


