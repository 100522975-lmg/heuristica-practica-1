#model 3

#define pallet params
param L > 0, integer;
param H > 0, integer;

set BOXES;
set PALLET := 1..L;
set FLOOR := 1..H;

param priority {p in BOXES};
param weight   {w in BOXES};
param capacity {c in BOXES};

var x {i in PALLET, j in PALLET, h in FLOOR, k in BOXES}, binary; #1 if box k is located in position [i,j] of floor h

minimize Cost : sum{i in 2..L, j in PALLET, h in FLOOR, k in BOXES} (i-1)*priority[k]*x[i,k,h,k]

#define constraints

s.t. BoxConstraint {k in boxes} : sum{i in PALLET, j in PALLET, h in FLOOR} x[i,j,h,k] == 1; #Each box is only placed once
s.t. PosConstraint {i in PALLET, j in PALLET, h in FLOOR} : sum{k in boxes} x[i,j,k,h] <= 1; #Each position can have at most one box since it may be empty spaces
s.t. PrioConstraint {i in 1..(L-1), j in PALLET, h in FLOOR} : sum{k in BOXES} priority[k]*x[i,j,h,k] >= sum{k in BOXES} priority[k]*x[i+1,j,h,k]; #Priority of a box in a row should be greater in value than the one in the next row
s.t. CargoConstraint {i in PALLET, j in PALLET, h in 1..(H-1)} : sum{b in (h+1)..H, k in BOXES} weight[k]*x[i,j,b,k] <= sum{k in boxes} capacity[k] * x[i,j,h,k]; #The weight of the boxes above one should be less than its capacity
s.t. FlyingBoxConstraint {i in PALLET, j in PALLET, h in 2..H} : sum{k in boxes} x[i,j,h,k] <= sum{k in boxes} x[i,j,h-1,k]; #if there is a box in pos [i,j] of floor h there should be a box in the same position of floor h-1   

