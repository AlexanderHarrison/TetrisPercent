# Hold Optimization

## Overview

Using hold makes more bags work for a given field than without hold. I 
figured that because of this, I shouldn't have to test evey single bag with hold
logic to determine what works and what doesn't. Instead, I can see if a bag 
works *without* hold, and if it does, mark all the bags that are equivalent to the bag
*using* hold to work.

There is currently no optimization if the bag does not work. I can see a 
possibility and I may try to implement it if I need the speedup. 

This is as less of a write-up for your interest as it is notes for me.

In this I assume you understand the Hold system in Tetris, permutations, 
and factorials.

## Syntax

I have two representations of all possible bags. One describes the bag itself
and the other describes the relationship between bags.

Also note when I write *bag* it is usually equivalent to *permutation*.

### Representation 1

I directly represent bags using capital letters from A-Z.<br>
All the permutations of a bag of size 3 are:<br>
ABC ACB BAC BCA CAB CBA (6 possibilites)

All the permutations of a bag of size 4 are:<br>
ABCD ABDC ADBC DABC ACBD ACDB ADCB DACB CABD CADB CDAB DCAB
BACD BADC BDAC DBAC BCAD BCDA BDCA DBCA CBAD CBDA CDBA DCBA

For a bag of size n, there are n! possibilities. 3! = 3 * 2 = 6. 4! = 4 * 3 * 2 = 24

If you want to translate this into a Tetris bag, just replace a letter with a 
piece. ABC => TOJ<br>
ABC ACB BAC BCA CAB CBA<br>
TOJ TJO OTJ OJT JTO JOT<br>
I think of the pieces coming from left to right, so TOJ => T, O, J.

### Representation 2

The representation describing relationships is more complicated.

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

For a bag of size 6, the right digit has 6 possibilites: 0-5. The next to the left 
has 5. The next has 4. Then 3, and 2.<br>
So for a bag of size n, this representation covers n * (n-1) * (n-2) ... * 2
possibilites, or n! possibilites.

**This represents relationships, as each digit string does not describe a
specific bag, but a transformation of some other bag.**

So I can take ABDC and apply 113 to it.
lign them right justified.
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

Note this representation cannot be added with itself easily.

If you apply any one of these representations to ABCD you get a unique bag, so
effectively you can represent all possible permutations of ABCD by this 
representation system.

## Optimization
````
ABC -> ABC ACB BAC CAB
````
If ABC works without hold, then ABC ACB BAC CAB works with hold. I say ABC 
*implies* ABC ACB BAC CAB.

Let us see what the transformations to ABC are of these. <br>
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

But there is more optimization to be had. Each bag has a twin that implies the
exact same bags. Flip the last two letters to get the twin. implies(ABCD) = 
implies(ABDC). This means half of tests are irrelevant, splitting the neccessary
bags to test in half. 1 307 674 368 000 bags becomes 39 907 055 bags.

## Being Implied
It has been shown how to find all the bags implied from a certain bag. What if you wanted to find the opposite? How would you find all the bags which imply a certain bag? Or in other words, what is the set of bags **without hold** equivalent to this bag **with hold**? 

ABC *is implied by* ABC BAC BCA ACB

````
ABC <- ABC BAC BCA ACB
 00 <-  00  10  02  01

ABCD <- ABCD BACD BADC BCAD BCDA ACBD ACDB ABDC
 000 <-  000  100  101  020  003  010  002  001
````

Do you see the pattern?
````
 00000 <- 00000 
          00001 00101 10101 00201 00301 00002 01002 02002 00003 10003
          00010 01010 02010 00020 10020 00030
          00100 10100 00200 00300
          01000 02000
          10000
````

If you don't I don't blame you. It can be hard to pin down.
Essentially, it is the set of transformations where these two rules hold.
1. Each digit n has at least n zeroes to the left of it
2. There is one not visible pseudozero at the very left that counts as a zero for rule 1

This method is complicated so I don't use it.
There is simpler method for finding what implies a bag, and it is surprising.

Notice how to initial bags are reverses of each other
````
rev(ABC) -> CBA

ABC <- ABC BAC BCA ACB
 00 <-  00  10  02  01

CBA -> CBA CAB ACB BCA
 00 ->  00  01  11  10
````

Since CBA is the reverse of ABC, what happens if we reverse the implies of CBA?

````
CBA -> CBA CAB ACB BCA
             rev
       ABC BAC BCA ACB

ABC <- ABC BAC BCA ACB
````

**They are the same as what implies ABC!**

Succinctly:
````
rev(
  this_implies(
    rev(ABC)
  )
) == this_implied_by(ABC)
````

I don't have a mathematical proof for this but it seems to work. And I can sort of reason that it works in my head.

This is needed because I never manually find what a bag implies, even though there is a simple method for finding it. I always find what implies a bag instead, which now has a simple method for converting to an implies function with some simple reversing.

## Memory Usage
Without optimization there is no memory problem. Simply visit each bag and keep a tally of which bags work and which do not.


Just testing of each bag leads to a memory problem. There needs to be some way to store what bags have been solved so I don't re-test them, but I can't store 1.3 trillion values in memory, unless you have a terabyte of ram.

I use two passes over the set of bag possibilities. In the first pass, I find all the bags that work without hold. In the next pass, for each bag I find, if any of the set of bags that imply the testing bag are in the bags that work, then this bag works. This will still take large amounts of memory if there are many bags that work without hold, but much less than storing all bags.

pseudocode for people who can't understand my bad explanation.
````
for bag in bag_possibilities
  if works_without_hold(bag, field)
    works_without_hold_bags += bag

for bag in bag_possibilities
  let bags_which_imply_this = this_implied_by(bag)

  if bags_which_imply_this.any() in works_without_hold_bags
    works += 1

print(works / bag_possibilites.len())
````

# Alternative
Second representation only

```
ABC <- ABC BAC BCA ACB
 00 <-  00  10  11  01

BAC <- BAC ABC ACB BCA
 00 <-  00  10  11  01

CAB <- CAB ACB ABC CBA
 00 <-  00  10  11  01

ABC ACB CAB BAC BCA CBA
 00  01  02  10  11  12
```

first pass:
  get bags that work without hold

second pass:
  for each bag:
    compute what implies this bag
    if any of those are in the bags that work without hold, then add the works total

the works total is how many bags with hold work

