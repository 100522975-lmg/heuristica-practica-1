#model 3

param M >0 ; #Big M parameter 
#define pallet params
param L > 0, integer;
param H > 0, integer;
param n > 0, integer; #Number of boxes

set BOXES;
set AUGBOXES := BOXES union {"EmptyBox"} ;
set LENGTH := 1..L;
set FLOORS := 1..H;

param priority {p in AUGBOXES}, default M;
param weight   {w in AUGBOXES}, default 0;
param capacity {c in AUGBOXES}, default M;

#Variables

var x {i in LENGTH, j in LENGTH, h in FLOORS, k in AUGBOXES}, binary; #1 if box k is located in position [i,j] of floor h
#Cost variable, If there is a box in position [i,j,h] it takes the value of the added cost by that box, 0 otherwise
var c {i in 1..(L-1), j in LENGTH, h in FLOORS} >= 0;

minimize Cost : (1/n) * sum{i in 1..(L-1), j in LENGTH, h in FLOORS} c[i,j,h];

#define constraints

#Each real box is only placed once (empty boxes must be placed h*l*l - n times) 
s.t. BoxConstraint {k in BOXES} : 
sum{i in LENGTH, j in LENGTH, h in FLOORS} x[i,j,h,k] == 1;

#The total number of empty boxes is h*l*l - n (number of positions - number of boxes) 
s.t. EmptyBoxConstraint :
sum{i in LENGTH, j in LENGTH, h in FLOORS} x[i,j,h,"EmptyBox"] == H*L*L - n; 

#Each position can have at most one box since it may be empty spaces, empty box is represented with special index
s.t. PosConstraint {i in LENGTH, j in LENGTH, h in FLOORS} : 
sum{k in AUGBOXES} x[i,j,h,k] == 1;

#Big-M constraint that constraints the cost of a position if there is a box in there
s.t. CostConstraint {i in 1..(L-1), j in LENGTH, h in FLOORS} :
c[i,j,h] >= sum{r in (i+1)..L,k in BOXES}priority[k]*x[r,j,h,k] - M*x[i,j,h,"EmptyBox"]; 

#Priority of a box in a row should be greater in value than the one in the next row
s.t. PrioConstraint {i in 1..(L-1), j in LENGTH, h in FLOORS} : 
sum{k in BOXES} priority[k]*x[i,j,h,k] + M*x[i,j,h,"EmptyBox"] >= sum{k in BOXES} priority[k]*x[i+1,j,h,k] + M*x[i+1,j,h,"EmptyBox"]; 

#The weight of the boxes above one should be less than its capacity
s.t. CargoConstraint {i in LENGTH, j in LENGTH, h in 1..(H-1)} : 
sum{b in (h+1)..H, k in AUGBOXES} weight[k]*x[i,j,b,k] <= sum{k in AUGBOXES} capacity[k] * x[i,j,h,k];

#if there is a box in pos [i,j] of floor h there should be a box in the same position of floor h-1   
s.t. FlyingBoxConstraint {i in LENGTH, j in LENGTH, h in 2..H} : 
sum{k in BOXES} x[i,j,h,k] <= sum{k in BOXES} x[i,j,h-1,k]; 

solve;

end;
