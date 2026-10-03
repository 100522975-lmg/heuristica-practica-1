#model 2 

#define pallet params
param L > 0 , integer;
param H > 0 , integer;

set BOXES;
set PALLET := 1..L;
set FLOOR  := 1..H;

param priority {p in BOXES}; #Box k has priority p[k]
param weight   {w in BOXES}; #Box k has weight   w[k]
param capacity {c in BOXES}; #Box k has capacity c[k]

var x{i in PALLET,j in PALLET,h in FLOOR, k in BOXES}, binary; #True if box k is located in position [i,j] and floor h 

minimize Cost : sum{i in 2..L, j in PALLET, h in FLOOR, k in BOXES} (i-1)*priority[k]*x[i,j,h,k];

#Same as in model 1 but iterating over all floors (reasoning is the same) 

s.t. BoxConstraint {k in BOXES} : sum{i in PALLET, j in PALLET, h in FLOOR} x[i,j,h,k] == 1; #Each box must be placed only once
s.t. PosConstraint {i in PALLET, j in PALLET, h in FLOOR} : sum{k in BOXES} x[i,j,h,k] == 1; # Each position must contain only one box
s.t. PrioConstraint {i in 1..(L-1),j in PALLET, h in FLOOR} : sum{k in BOXES} priority[k]*x[i,j,h,k] >= sum{k in BOXES} priority[k]*x[i+1,j,h,k];
s.t. CargoConstraint{i in PALLET, j in PALLET,h in 1..(H-1)} : sum{b in (h+1)..H, k in BOXES} weight[k]*x[i,j,b,k] <= sum {k in BOXES} capacity[k] * x[i,j,h,k];    
