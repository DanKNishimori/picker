# Picker
A simple program to draw an item from a given list.

## Modes
### File Mode (wip)
You can draw an item from a file using by typing `picker list_file.txt`

### REPL Mode (wip)
Another option is to just type `picker -r` or `picker --repl` and type directly your entries.

```
<< REPL Mode ; list your entries then type "!go" or "!draw" >>

>>> a
>>> b
>>> c
>>>
>>> !go

```

## Flags (wip)
You can add some flags to the entries and change the probability of somthing being drawn. The flags indicate that if it isn't an entry or it should ignore the entry. 

### Values
Some flags can use values to change the probality of an entry coming out. These values can be:
 - [x] integer numbers
 - [ ] negative numbers
 - [ ] decimal numbers
 - [ ] porcentages

### Priority flag
You can define a higher priority level by adding the flag `#priority <LEVEL>` in the end of the entry.

The other entries will recieve automatically a lower level when that is made, and these levels are in descending order, which means that `#priority 1` is greater than `#priority 2`.

The way it works is similar to repeating the entry one more time, like `a #priority 1` = `a, a` in this sample:
```
a #priority 1
b 
c
```
> result: a (50% of the times), b (25% of the times), c (25% of the times)

Note: `#priority` does not accepts porcentages due to how was chosen to it work. 

### Weight flag
You also can set a weight level, by adding the flag `#weight <LEVEL>`.

The weight is set to 1 by defaut, and the order is in ascending order (`#weight 1` is lower than `#weight 2`).
```
a #weight 1.5
b 
c
```
> result: a (50% of the times), b (33.3% of the times), c (33.3% of the times)

### Ignore flags
These flags sets to 0% the probality of coming a out, and the difference between them is just the name, like:
 - `#title`
 - `#label`
 - `#ignore`
 - `#comment`
 - `#...`

They are used to just add text and context to the file, without making it work wrongly.

One sample of usage of these tags is:

```
Raffle List of December for Youtube viewers #title
Prize: special minecraft skin               #comment

- Subscribers group -                       #label
Mildewed
Assaultive
ThunderHawk
#...

- Regulars group -                          #label
Morello
Roanokay
Friezer (last winner)                       #ignore
TommyGun
Hedonist
#...
```