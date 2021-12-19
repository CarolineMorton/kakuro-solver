## Kakuro Solver

This is a simple command line tool that creates unique permutations to solve [Kakuro](https://en.wikipedia.org/wiki/Kakuro). 

Each permutation can only be made up out of 1 - 9, and must add up to the value indicated as the start of the column or row. 

### How to use

`cargo run <target_value> <length> <existing_numbers>`

- `<target_value>` - This is the sum that you are aiming for
- `<length>`- This is the length of the permutation that you want
- `<existing_numbers>` - These are numbers that you have already put in your puzzle for this permutation. 
Put as `0` if you have no numbers. 

##### Example:

![img_2.png](img_2.png)

We need 4 numbers to make up 21. 

`cargo run 21 4 0` 

We get:
```python
[1, 3, 8, 9]
[1, 4, 7, 9]
[1, 5, 6, 9]
[1, 5, 7, 8]
[2, 3, 7, 9]
[2, 4, 6, 9]
[2, 4, 7, 8]
[2, 5, 6, 8]
[3, 4, 5, 9]
```

Imagine, we already had 7 written in this row and therefore  we are only interested in permutations that include 7. 

`cargo run 21 4 7`

We get:
```python
[1, 4, 7, 9]
[1, 5, 7, 8]
[2, 3, 7, 9]
[2, 4, 7, 8]
[3, 5, 6, 7]
```

Imagine now we have 8 as well as 7. 

`cargo run 21 4 78`

We get:
```python
[1, 5, 7, 8]
[2, 4, 7, 8]
```