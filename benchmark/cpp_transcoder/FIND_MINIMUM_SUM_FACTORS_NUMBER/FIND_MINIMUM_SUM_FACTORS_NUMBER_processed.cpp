
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int num ) {
  int sum = 0;
  for ( int i = 2;
  i * i <= num;
  i ++ ) {
    while ( num % i == 0 ) {
      sum += i;
      num /= i;
    }
  }
  sum += num;
  return sum;
}


