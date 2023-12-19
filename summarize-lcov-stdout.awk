#!/usr/bin/awk -f
#
# This AWK script turns the output of lcov into a single row of a CSV file.

BEGIN {
	LINE = "";
	BRANCH = "";
}

/^\s\slines\.\./ {
	LINE = $2;
}

/^\s\sbranches\.\./ {
	BRANCH = $2;
}

END {
	print ENVIRON["PWD"]  "," LINE "," BRANCH
}
