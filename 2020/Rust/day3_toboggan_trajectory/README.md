# Thoughts on the day

This one looked intimidating but was actually quite easy !

## Thoughts on learning/using Rust

* Iterator functions are really helpfull 
* Small introduction to borrowing/ownershipt by passing lines to the count_trees function

# Pseudo code

```rustlang

cursor set to index 0

for each line in the pattern stepping by angle down value
    we increment the cursor by angle step value
    
    if we'd be out of bound we make the cursor loop
    
    if the cursor is on a '#'
        increment counter
    
return counter
```

Idea on optimization :
 - maybe use a buffer instead of reading the whole file as a string
 
 
