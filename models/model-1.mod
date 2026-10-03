# dq we
set BOXES;
set l; 

param priority {b in BOXES}; #Box k has priority k 

var x {i in l, j in l, k in BOXES} , binary; #True if box k is in position [i,j]

minimize Cost: sum{i in l, j in l, k in BOXES} x[i,j,k] * (sum{b in BOXES, r > i , r < l,} priority[b]*x[r,j,b]) 

s.t. BoxConstraint {k in BOXES} : sum{i in l, j in l} x[i,j,k] == 1; #Each box must be placed only once
s.t. PosConstraint {i in l, j in l} : sum{k in BOXES} x[i,j,k] == 1; #Each position must contain only one box
s.t. PrioConstraint {i in l, j in l}: sum{k in BOXES} priority[k]*x[i,j,k] > sum{k in BOXES} priority[k]*x[i+1,j,k]    
