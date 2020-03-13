# Hold Optimization

## Overview

Using hold makes more bags work for a given field than without hold. I 
figured that because of this, I shouldn't have to test evey single bag with hold
logic to determine what works and what doesn't. Instead, I can see if a bag 
works *without* hold, and if it does, mark all the bags that are equivalent
*using* hold to work.

There is currently no optimization if the bag does not work. I can see a 
possibility and I may try to implement it if I need the speedup. 

This is as much a write-up for your interest as it is notes for me.

In this I assume you understand the Hold system in Tetris, permutations, 
and factorials.

## Syntax

I have two representations of all possible bagss. One describes the bag itself
and the other describes the relationship between bags.

Also note when I write *bag* it is usually equivalent to *permutation*.

### Representation 1

I directly represent bags using capital letters from A-Z.<br>
All the permutations of a bag of size 3 are:<br>
ABC ACB BAC BCA CAB CBA (6 possibilites)

For a bag of size n, there are n! possibilities. 3! = 3 * 2 = 6.

If you want to translate this into a Tetris bag, just replace a letter with a 
piece. ABC => TOJ<br>
TOJ TJO OTJ OJT JTO JOT<br>
I think of the pieces coming from left to right, so TOJ => T, O, J.

### Representation 2

The representation describing relationships is more complicated.<br>

All the permutations of a bag of size 3 are:<br>
00 01 02 10 11 12

All the permutations of a bag of size 4 are:<br>
000 001 002 003 010 011 012 013 020 021 022 023
100 101 102 103 110 111 112 113 120 121 122 123

For a bag with n items:

* There are n-1 digits.
* The digit to the far left is always either 0 or 1.
* The digit to the far right ranges between 0 and n-1.
* A digit ranges one less than the digit to the right of it.

For a bag of size 6, the right digit has 6 possibilites. The next to the left 
has 5. The next has 4. Then 3, and 2.<br>
So for a bag of size n, this representation covers n * (n-1) * (n-2) ... * 2
possibilites, or n! possibilites.

**This represents relationships, as each digit string does not describe a
specific bag, but a transformation of some other bag.**

So I can take ABDC and apply 113 to it.
Align them right justified.
````
  A B D C
+   1 1 3
````
1. Get the leftmost non-zero digit
2. Shift the letter above that digit that many times to the left
3. Set that digit to 0
4. Repeat until all digits are 0
````
  A B D C
+  (1)1 3

  B A D C
+   0(1)3

  B D A C
+   0 0(3)

  C B D A
+   0 0 0

ABDC + 113 = CBDA
````

If you apply any one of these representations to ABCD you get a unique bag, so
effectively you can represent all possible permutations of ABCD by this 
representation system.

## Optimization
````
ABC -> ABC ACB BAC CAB
````
If ABC works without hold, then ABC ACB BAC CAB works with hold. I say ABC 
*implies* ABC ACB BAC CAB.

Let us see what the transformations to ABC are of 
these. <br>
(**IMPORTANT:** this is not the transformation **from** ABC, but the
transformation that is required to **get** to ABC):
````
ABC -> ABC ACB BAC CAB
 00 ->  00  01  10  11
````
How about with 4 items?
````
BDCA -> BDCA BDAC BCDA BADC DBCA DBAC DCBA DABC
 000 ->  000  001  010  011  100  101  110  111
````
Where are the 2s and 3s?

This is the key to the optimization and the reason the transformation
representation is complicated: **THE ONLY TRANSFORMATIONS THAT WORK WITH HOLD ARE THE TRANSFORMATIONS WITH ONLY 0 and 1**

So that means out of n! total bags, 2^(n-1) permutations are implied 
each time if one bag works. This is still O(n!), but in practice it is much
faster than without.

Take the example of 15 pieces. Bags are limited to 7 pieces, so in reality this 
would never happen, but bear with me.

There would be 15! bags. 15! = 1 307 674 368 000. Without hold optimization I 
would have to test each of the 1.3 trillion possibilites. With hold optimization
this is reduced to a minimum of 79 814 110 bags to check. In reality the 
difference is less, but you get the point.

But this leads to a memory problem. There needs to be some way to store what 
bags have been solved so I don't re-test them, but I can't store 1.3 trillion
values in memory, unless you have a terabyte of ram, so some other method is 
used. Unfortunately I haven't discovered this method yet.