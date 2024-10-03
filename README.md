# ε-NFA Simulator

This Rust program implements a simulator for a non-deterministic finite automaton (NFA) with epsilon transitions (ε-NFA). The program reads a definition of the automaton from input, processes a set of input strings, and outputs the sets of states the automaton occupies for each character of the input strings.

## Task Description

You are required to implement a simulator for an ε-NFA as part of a laboratory exercise. The program will simulate the automaton's transitions based on a textual definition and an input string. The output will show the sets of states the automaton is in for each input symbol, maintaining lexicographic ordering of states in each set. The input consists of several components:

1. **First line**: Input strings separated by `|`. Symbols in each string are separated by commas.
2. **Second line**: Lexicographically ordered set of states separated by commas.
3. **Third line**: Lexicographically ordered set of alphabet symbols separated by commas.
4. **Fourth line**: Lexicographically ordered set of accepting states separated by commas.
5. **Fifth line**: The initial state of the automaton.
6. **Sixth line and onwards**: Transition functions in the format `currentState,inputSymbol->nextStates`. Epsilon transitions use `$` instead of an alphabet symbol. If there are no defined transitions for a state-input pair, the transition is to an empty set (`#`).

### Constraints:
- The input string length will not exceed 1500 characters.
- The automaton will have at most 100 states and 100 alphabet symbols.
- States and alphabet symbols are represented as sequences of lowercase English letters and decimal digits, with a length between 1 and 20 characters.
- Lexicographic order is defined by the ASCII values of characters. The symbol `$` is lexicographically smaller than all digits and letters.

### Example Input

Below is an example input that defines an ε-NFA:

```plaintext
a,pnp,a|pnp,lab2|pnp,a|pnp,lab2,utr,utr
p5,s3,s4,st6,stanje1,stanje2
a,lab2,pnp,utr
p5
stanje1
s3,a->stanje2
s3,lab2->p5,s4
s4,$->st6
s4,utr->p5,s3
stanje1,a->stanje2
stanje1,pnp->s3
stanje2,$->st6
stanje2,a->#
```

### Example Output

The output for the above input will look like this:

```plaintext
stanje1|stanje2,st6|#|#
stanje1|s3|p5,s4,st6
stanje1|s3|stanje2,st6
stanje1|s3|p5,s4,st6|p5,s3|#
```


## Running the Program

To run the program using Cargo, follow these steps:

1. Ensure that Rust and Cargo are installed on your system.
2. In the terminal, navigate to the directory containing your project.
3. Use the following command to run the program with `cargo`:

```bash
cargo run -- < input.txt
```
