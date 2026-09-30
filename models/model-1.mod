# dq we

set BOXES;
set l; 

param priority {b in boxes};

var x {i in l, j in l, k in boxes} , binary;
