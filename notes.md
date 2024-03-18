# Notes

- Literal checking can be done at compile time
- None literal checking can't be done, can be done at run time



- Need a compile time check for literals and a run time check for everything else


# TODO

-[x] Support condition or list of values
-[x] Fix the literals problem
-[x] Add full list of css properties
-[x] Create output object
-[_] Create dioxus support

-[_] Define any other kinds of checks
-[_] Double Values
-[_] Multiple Values
-[_] Custom Regex




#### Fix

At build time need to scan through code base and find all css objects, at the point they can be passed and converted to css files.
Then at compile time the objects don't need to be checked for validity and only the string names of objects are needed,
The problem with this method is that at build time only literals could be used as the code must be run.

We have to generate the css at run time as we may want to use run time variables in our code.
This suggests that css styles must be defined in functions, however, this is syntactically not very nice. Its is nice to simply define them in the global scope, suggesting that the styles themselves must be functions. When the function is called, it checks if the css style has been inserted int the global css style. If the css style has changed then the css file will be regenerated and the css object in the top level will be recompiled.

## Goals
    - Ability to define a css stylesheet at the top level scope
    - At runtime write all the rules to the css file (potentially add optimisations here for compile time calculable strings)
    - When a rule is changed, update the stylesheet
    - Provide a version for web using stylesheet modifications, or for local file modifications
    - Allow adding of css class names directly as string literals at compile time
    - Allow use of css syntax in rust or just a complete css string




### Build Time
- extract all css definitions from file
- write rules for all css definitions
- give each css definition a randomly generated class name (do this from the hash of the file name and location in file of each object)

### Compile Time
- check that the css rules have been correctly followed
- create constants for all the names of the objects
