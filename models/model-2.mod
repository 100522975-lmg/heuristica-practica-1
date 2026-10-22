#model 2 

#define pallet params
param L > 0 , integer;
param H > 0 , integer;

set BOXES;
set LENGTH := 1..L;
set FLOORS  := 1..H;

param priority {p in BOXES}; #Box k has priority p[k]
param weight   {w in BOXES}; #Box k has weight   w[k]
param capacity {c in BOXES}; #Box k has capacity c[k]

var x{i in LENGTH,j in LENGTH,h in FLOORS, k in BOXES}, binary; #True if box k is located in position [i,j] and floor h 

minimize Cost : sum{i in 2..L, j in LENGTH, h in FLOORS, k in BOXES} (i-1)*priority[k]*x[i,j,h,k];

#Same as in model 1 but iterating over all floors (reasoning is the same) 

#Each box must be placed only once
s.t. BoxConstraint {k in BOXES} : 
sum{i in LENGTH, j in LENGTH, h in FLOORS} x[i,j,h,k] == 1; 

# Each position must contain only one box
s.t. PosConstraint {i in LENGTH, j in LENGTH, h in FLOORS} : 
sum{k in BOXES} x[i,j,h,k] == 1; 

#For each floor and column, the priority of a box should be greater in value than the one situated in the next row
s.t. PrioConstraint {i in 1..(L-1),j in LENGTH, h in FLOORS} : 
sum{k in BOXES} priority[k]*x[i,j,h,k] >= sum{k in BOXES} priority[k]*x[i+1,j,h,k];

#The weight of the boxes in the above floors should not exceed the capacity of the box that holds them
s.t. CargoConstraint{i in LENGTH, j in LENGTH,h in 1..(H-1)} : 
sum{b in (h+1)..H, k in BOXES} weight[k]*x[i,j,b,k] <= sum {k in BOXES} capacity[k] * x[i,j,h,k];

solve;    
