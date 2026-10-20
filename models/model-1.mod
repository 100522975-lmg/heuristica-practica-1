# dq we
param L > 0, integer;
 
set BOXES;
set LENGTH := 1..L; 

param priority {b in BOXES}; #Box k has priority k 

var x {i in LENGTH, j in LENGTH, k in BOXES} , binary; #True if box k is in position [i,j]

minimize Cost:
(1/(L*L)) * sum{i in 2..L,j in LENGTH, k in BOXES} (i-1)*priority[k]*x[i,j,k]; 

#box in row i is counted i-1 times for cost (once for  each box below)   

#Each box should be placed only once
s.t. BoxConstraint {k in BOXES} :
sum{i in LENGTH, j in LENGTH} x[i,j,k] == 1;

#Each position must contain only one box
s.t. PosConstraint {i in LENGTH, j in LENGTH}:
sum{k in BOXES} x[i,j,k] == 1;

#The priority of a box one row behind should be bigger in value than the one in front
s.t. PrioConstraint {i in 1..(L-1), j in LENGTH}:
sum{k in BOXES} priority[k]*x[i,j,k] >= sum{k in BOXES} priority[k]*x[i+1,j,k];

solve; 
