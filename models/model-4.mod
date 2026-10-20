#model 3

param M >0 ; #Big M parameter 
#define pallet params

param L > 0, integer;
param H > 0, integer;
param n > 0, integer; #Number of boxes
param p_cost >0, integer; # Cost of adding a pallet

set BOXES;
set PALLETS;
set AUGBOXES := BOXES union {"EmptyBox"} ;
set LENGTH := 1..L;
set FLOORS := 1..H;

param priority {p in AUGBOXES}, default M;
param weight   {w in AUGBOXES}, default 0;
param capacity {c in AUGBOXES}, default M;

#Variables

var x {i in LENGTH, j in LENGTH, h in FLOORS, p in PALLETS, k in AUGBOXES}, binary; #1 if box k is located in position [i,j] of floor h
#Cost variable, If there is a box in position [i,j,h] it takes the value of the added cost by that box, 0 otherwise
var c {i in 2..L, j in LENGTH, h in FLOORS, p in PALLETS} >= 0;
var pallet {p in PALLETS}, binary;

minimize Cost : sum{i in 2..L, j in LENGTH, h in FLOORS, p in PALLETS} c[i,j,h,p] + sum{p in PALLETS} p_cost * pallet[p];

#define constraints

#Each real box is only placed once (empty boxes must be placed h*l*l - n times) 
s.t. BoxConstraint {k in BOXES} : 
sum{i in LENGTH, j in LENGTH, h in FLOORS, p in PALLETS} x[i,j,h,p,k] == 1;

#The total number of empty boxes is h*l*l - n (number of positions - number of boxes) 
s.t. EmptyBoxConstraint:
sum{i in LENGTH, j in LENGTH, h in FLOORS, p in PALLETS} x[i,j,h,p,"EmptyBox"] == (sum{p in PALLETS} pallet[p]*H*L*L) - n; 

#Each position can have at most one box since it may be empty spaces, empty box is represented with special index
s.t. PosConstraint {i in LENGTH, j in LENGTH, h in FLOORS, p in PALLETS} : 
sum{k in AUGBOXES} x[i,j,h,p,k] == 1;

#Big-M constraint that constraints the cost of a position if there is a box in there
s.t. CostConstraint {i in 2..L, j in LENGTH, h in FLOORS, k in BOXES, p in PALLETS} :
c[i,j,h,p] >= priority[k] * (sum{r in 1..(i-1), l in BOXES} x[r,j,h,p,l] ) - M*(1-x[i,j,h,p,k]); 

#Priority of a box in a row should be greater in value than the one in the next row
s.t. PrioConstraint {i in 1..(L-1), j in LENGTH, h in FLOORS, p in PALLETS} : 
sum{k in BOXES} priority[k]*x[i,j,h,p,k] + M*x[i,j,h,p,"EmptyBox"] >= sum{k in BOXES} priority[k]*x[i+1,j,h,p,k] + M*x[i+1,j,h,p,"EmptyBox"]; 

#The weight of the boxes above one should be less than its capacity
s.t. CargoConstraint {i in LENGTH, j in LENGTH, h in 1..(H-1), p in PALLETS} : 
sum{b in (h+1)..H, k in AUGBOXES} weight[k]*x[i,j,b,p,k] <= sum{k in AUGBOXES} capacity[k] * x[i,j,h,p,k];

#if there is a box in pos [i,j] of floor h there should be a box in the same position of floor h-1   
s.t. FlyingBoxConstraint {i in LENGTH, j in LENGTH, h in 2..H, p in PALLETS} : 
sum{k in BOXES} x[i,j,h,p,k] <= sum{k in BOXES} x[i,j,h-1,p,k]; 

# Unused pallets have no boxes
s.t. PalletConstraint {p in PALLETS}:
sum{i in LENGTH, j in LENGTH, h in FLOORS, k in BOXES} x[i,j,h,p,k] <= n * pallet[p];

solve;
end;
