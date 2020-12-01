# Thoughts on the day

First one is always easy, I went with a simple brute-force approach.

Optimization : 
* Added a check if the initial value is already bigger than 2020
* Added another check in part II if the 2 first number are bigger than 2020

# Pseudo code

```rustlang
For each value in the report

  For each other_value in the report starting a index+1

     if value + other_value equals 2020

       return value + other_value
```

Part II is simply an added nested loop.
