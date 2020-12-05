# Thoughts on the day

Super easy one with some binary knowledge

## Thoughts on learning/using Rust

* Getting even more familiar with Rust
* I tried to use a more functionnal syntax

## Pseudo code

### To Decode row and column 

```rustlang

    take 7 first character and replace F by 0 and B by 1
        interpret binary number as decimal

    take 3 last character and replace L by 0 and R by 1
        interpret binary number as decimal
 
```

### To find my seat

```rustlang

    for each boarding pass
        decode the seat id
    
   sort the seat_id (O(n) Log(n))

   for each seat in sorted seats
    if next_seat != (current_seat + 1)
        return current_seat + 1
 
```

 
 
