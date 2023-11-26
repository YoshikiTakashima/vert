
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int n ) {
  long int multiTerms = n * ( n + 1 ) / 2;
  long int sum = multiTerms;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    multiTerms = multiTerms - ( i - 1 );
    sum = sum + multiTerms * i;
  }
  return sum;
}


