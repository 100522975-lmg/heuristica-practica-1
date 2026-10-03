# dq we
param L > 0, integer;
 
set BOXES;
set PALLET := 1..L; 

param priority {b in BOXES}; #Box k has priority k 

var x {i in PALLET, j in PALLET, k in BOXES} , binary; #True if box k is in position [i,j]

minimize Cost: sum{i in 2..L,j in PALLET, k in BOXES} (i-1)*priority[k]*x[i,j,k]; 

#box in row i is counted i-1 times for cost (once for  each box below)   

s.t. BoxConstraint {k in BOXES} : sum{i in PALLET, j in PALLET} x[i,j,k] == 1; #Each box must be placed only once
s.t. PosConstraint {i in PALLET, j in PALLET} : sum{k in BOXES} x[i,j,k] == 1; #Each position must contain only one box
s.t. PrioConstraint {i in 1..(L-1), j in PALLET}: sum{k in BOXES} priority[k]*x[i,j,k] >= sum{k in BOXES} priority[k]*x[i+1,j,k]; 
