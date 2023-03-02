# Grammar
## Human Forms
- `1` -> 1
- `d6` -> Roll 1d6
- `1d8` -> Roll 1d6
- `2d6 + 4` -> Roll 2d6, add 4
- `1d6 + 2d4` -> Roll 1d6, add 2d4
- `1d6 + 1d4 + 2` -> Roll 1d6, add 1d4, add 2
- `d20 - 2d4` -> Roll 1d20, subtract 1d4
- `1d20 - 1d4 + 2` -> Roll 1d20, subtract 1d4, add 2
- `2d20rl + 2` -> Roll 1d20 and remove 1 lowest, add 2
- `5d6r2l` -> Roll 5d6 and remove 2 lowest
- `2d20rh + 1` -> Roll 2d20 and remove 1 highest, add one

## Context Free Grammar
### Terminals
| Symbol | Meaning                            | Lexeme Examples |
|--------|------------------------------------|-----------------|
| INT    | Any non-negative, non-zero integer | 1, 23, 4        |
| DIE    | Dice indicator                     | d               |
| REM    | Removal indicator                  | r               |
| DIR    | Removal direction, low or high     | l, h            |
| ADD    | Addition operator                  | +               |
| SUB    | Subtraction operator               | -               |

### Language Rules
```txt
expr -> expr ADD term | expr SUB term | term
term -> dice REM INT DIR | dice REM DIR | dice | INT
dice -> INT DIE INT | DIE INT
```
### Translations of Human Form Examples
| Human Form       | Reduced CFG Form                      |
|------------------|---------------------------------------|
| `1`              | `INT`                                 |
| `d6`             | `DIE INT`                             |
| `1d6`            | `INT DIE INT`                         |
| `2d6 + 4`        | `INT DIE INT ADD INT`                 |
| `1d6 + 1d4 + 2`  | `INT DIE INT ADD INT DIE INT ADD INT` |
| `d20 - 2d4`      | `DIE INT SUB INT DIE INT`             |
| `1d20 - 1d4 + 2` | `INT DIE INT SUB INT DIE INT ADD INT` |
| `2d20rl + 2`     | `INT DIE INT REM DIR ADD INT`         |
| `5d6r2l`         | `INT DIE INT REM INT DIR`             |
| `2d20rh + 1`     | `INT DIE INT REM DIR ADD INT`         |

### Parse Tree Examples

#### 2d6 + 4
##### CFG Form
```mermaid
flowchart TD
    A(expr)-->B(expr)
    B-->C(term)
    C-->D(roll)
    D-->E(dice)
    E-->F[2]
    E-->G[d]
    E-->H[6]
    A-->I[+]
    A-->J(term)
    J-->K[4]
```
##### Code Form
```mermaid
flowchart TD
    A(+) --> B(d)
    A    --> C(4)
    B    --> D(2)
    B    --> E(6)
```

#### 3d20r2l - 2d4rh + d2 + 7 - 3d7
```mermaid
flowchart TD
    1-1(expr) --> 2-1(expr)
    2-1       --> 3-1(expr)
    3-1       --> 4-1(expr)
    4-1       --> 5-1(expr)
    5-1       --> 6-1(term)
    6-1       --> 7-1(roll)
    7-1       --> 8-1(dice)
    8-1       --> 9-1[3]
    8-1       --> 9-2[d]
    8-1       --> 9-3[20]
    7-1       --> 8-2(remv)
    8-2       --> 9-4[r]
    8-2       --> 9-5[2]
    8-2       --> 9-6[l]
    4-1       --> 5-2["-"]
    4-1       --> 5-3(term)
    5-3       --> 6-2(roll)
    6-2       --> 7-2(dice)
    7-2       --> 8-3[2]
    7-2       --> 8-4[d]
    7-2       --> 8-5[4]
    6-2       --> 7-3(remv)
    7-3       --> 8-6[r]
    7-3       --> 8-7[h]
    3-1       --> 4-2[+]
    3-1       --> 4-3(term)
    4-3       --> 5-4(roll)
    5-4       --> 6-3(dice)
    6-3       --> 7-4[d]
    6-3       --> 7-5[2]
    2-1       --> 3-2[+]
    2-1       --> 3-3(term)
    3-3       --> 4-4[7]
    1-1       --> 2-2["-"]
    1-1       --> 2-3(term)
    2-3       --> 3-4(roll)
    3-4       --> 4-5(dice)
    4-5       --> 5-5[3]
    4-5       --> 5-6[d]
    4-5       --> 5-7[7]
```
Code Form
3d20r2l - 2d4rh + d2 + 7 - 3d7
```mermaid
flowchart TD
    A("-") --> B(+)
    A      --> C(d)
    C      --> D[3]
    C      --> E[7]
    B      --> F(+)
    B      --> G[7]
    F      --> H("-")
    F      --> I(d)
    I      --> J[2]
    H      --> K(r)
    H      --> L(r)
    L      --> M(d)
    M      --> O[2]
    M      --> P[4]
    L      --> Q[h]
    K      --> R(d)
    R      --> S[3]
    R      --> T[20]
    K      --> U[2]
    K      --> V[l]
```