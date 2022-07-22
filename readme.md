
# Today's Goal: To  write Halo 2 based circuit generation for a function:

## xxx First tutorial: Play with Number transformation



## xxx Second tutorial: Achieve the circuit with e = (a + b) + (c + d)

  
### Test  40 =  (x + y) + (k + l)

This circuit requires a single chip:

- For this circuit we only need one add gate. 
- For this gate: two input and one output
- We need two advice column (for witness or intermediate) and one fixed column (instance)
## xxx Third tutorial:  k = (a+b +c) + (d+e+f) + (g+h+j)
- one gate
- three input, one output
- three advice column for each gate/chip
- 5 rows

### Table
- we will do three gate operation so we will have 3 rows for addition
- One row for last check operation
- 
# xxx tutorial: e = (a - b) + (c * d) 

This circuit requires three chips:

- add 
- sub
- mul

Each gate requires two input one output

### Table
- we will do three gate operation so we will have 3 rows (each for different chip)
- One row for last check operation

## xxx tutorial: Lookup Table
Halo 2 uses the following lookup technique, which allows for lookups in arbitrary sets, and is arguably simpler than Plookup.

Here, we are going to prove that set A is subset of set B where 
- A can have duplicates elements
- B can have duplicate elements
- some elements of B may not in the set A

Think about opcodes of a smart contract. A contract can have 30 different opcodes.

- for each run of the contract, a subset of these opcodes will be executed. 
- In this context, it is very useful to use lookup.


lookup lets you prove that the elements of f are also contained in t. For example if `t = [1,2,3,4`] and f = [1,2,3] this will pass a plookup check but
`t = [1,2,3,4]` and `f = [1,2,5]` would not because 5 is not in t.

This can be tought of as a way of doing

- for element in f:
  - assert(element in t)   