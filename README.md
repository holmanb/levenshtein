LEVENSHTEIN
-----------

This project aims to correctly implement various edit distance algorithms in
the Rust programming language.

Edit distance is the minimum number of editing operations required to transform
one string into another. Multiple different edit distance categories exist.
These edit distances differ in supported operations and algorithmic complexity.

# Operations:

### Substitution
Replace one symbol with another.

Example:
```
Eleghant -> Elephant

Eleghant
|||R||||
Elephant
```
The letter 'g' is substituted with the letter 'p' in a single operation.


### Deletion
Remove one symbol.

Example:
```
Elephhant -> Elephant

Elephhant
||||D||||
Elep hant
```

The second letter 'h' is removed in a single operation.


### Insertion
Add one symbol.

Example:
```
Elehant -> Elephant

Ele hant
|||I||||
Elephant
```

The letter 'p' is added in a single operation.


### Transposition
Swap two adjacent symbols in the string.

Example:
```
Elehpant -> Elephant

Elehpant
|||SS|||
Elephant
```

The letters 'h' and 'p' are swapped in a single operation.


# Edit Distance Types:


### Hamming

Allowed Operations:

- Substitution

Note: strings must be of same length, as substitution does not change length


### Levenshtein

Allowed Operations:

- Substitution
- Deletion
- Insertion


### Damerau-Levenshtein

Allowed Operations:

- Substitution
- Deletion
- Insertion
- Substitution


### Longest common subsequence (not implemented)

Allowed Operations:
- Deletion
- Insertion


### Jaro (not implemented)

Allowed Operations:
- Transposition
