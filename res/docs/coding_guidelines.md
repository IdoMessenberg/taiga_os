# coding guidelines
## safety
## [the power of ten](https://en.wikipedia.org/wiki/The_Power_of_10:_Rules_for_Developing_Safety-Critical_Code)
the power of ten are rules for coding for reliable software
1. Avoid complex flow constructs, such as goto and recursion.
2. All loops must have fixed bounds. This prevents runaway code.
3. Avoid heap memory allocation (only the heap if you must).
4. Restrict functions to a single printed page (60 - 100 lines).
5. Use a minimum of two runtime assertions per function.
6. Restrict the scope of data to the smallest possible (i don't understand that).
7. Check the return value of all non-void functions, or cast to void to indicate the return value is useless.
8. Use the preprocessor sparingly.
9. Limit pointer use to a single dereference, and do not use function pointers.
10. Compile with all possible warnings active; all warnings should then be addressed before release of the software.

