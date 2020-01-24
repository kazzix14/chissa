# Chissa
Chissa is a tiny programming language inspired by brainfxxk.

# Operations
| ID    | Character | Function                                                       |
| ----- | --------- | -------------------------------------------------------------- |
| 0     | >         | increment data pointer                                         |
| 1     | <         | decrement data pointer                                         |
| 2     | [         | marker                                                         |
| 3     | ]         | go back to the first '['                                       |
| 4     | j         | if value is 0 then skip next operation                         |
| 5     | i         | input value                                                    |
| 6     | o         | output value                                                   |
| 7     | +         | increment value                                                |
| 8     | ^         | increment value by 10                                          |
| 9     | -         | decrement value                                                |
| 10    | v         | decrement value by 10                                          |
| 11-19 | 1-9       | repeat last operation n times                                  |
| 20    | r         | replace character at data pointer by ID=(value % NUM_COMMANDS) |
| 21    | 0         | set value to 0                                                 |
| 22    | c         | copy value                                                     |
| 23    | p         | paste value                                                    |
| 24    | (         | begin comment                                                  |
| 25    | )         | end comment                                                    |
