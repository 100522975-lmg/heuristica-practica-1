# model 4

param L > 0, integer;
param p_cost > 0, integer;

set BOXES;
set LENGTH := 1..L; 
set PALLETS;

param priority {b in BOXES}; #Box k has priority k 

var x {i in LENGTH, j in LENGTH, k in BOXES, p in PALLETS} , binary; #True if box k is in position [i,j]
var pallet {p in PALLETS}, binary;

minimize Cost:
(1/(L*L)) * sum{i in 2..L,j in LENGTH, k in BOXES, p in PALLETS} ((i-1)*priority[k]*x[i,j,k,p]) + sum{p in PALLETS} (p_cost * pallet[p]);

#box in row i is counted i-1 times for cost (once for  each box below)   

#Each box should be placed only once
s.t. BoxConstraint {k in BOXES}:
sum{i in LENGTH, j in LENGTH, p in PALLETS} x[i,j,k,p] == 1;

#Each position must contain only one box
s.t. PosConstraint {i in LENGTH, j in LENGTH, p in PALLET}:
sum{k in BOXES} x[i,j,k,p] == 1;

# Num. pallets must be at most 1 per box
# s.t PalletConstraint {p in PALLETS} :
# sum pallet[p] <= L*L;

#The priority of a box one row behind should be bigger in value than the one in front
s.t. PrioConstraint {i in 1..(L-1), j in LENGTH, p in PALLET}:
sum{k in BOXES} priority[k]*x[i,j,k,p] >= sum{k in BOXES} priority[k]*x[i+1,j,k,p];

# Unused pallets have no boxes
s.t PalletConstraint {p in PALLETS}:
sum{i in LENGTH, j in LENGTH, k in BOXES} x[i,j,k,p] <= H*L*L*pallet[p];

solve; 
end;
