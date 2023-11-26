
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


