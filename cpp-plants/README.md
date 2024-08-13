## CPP plants 

First things first. The makefile. 

This is my old template makefile. its been a while sense I really put thought into it so, it could use some improvements.
That being said. Currently. if you have an object you want to add to be compiled with make. you copy the example called TEMPLATE and replace the word TEMPLATE and then add the name to the list of OBJ.

That line looks like this:

```
# compiles a .o file for any template                                       edit
#$(DIR)TEMPLATE.o : TEMPLATE.c TEMPLATE.h $(SELF)
#   $(CC) $(CFLAGS) -c TEMPLATE.c -o $(DIR)TEMPLATE.o 
```

Then on line #18, you need to add the name of the object file, to this list:
```
#object files. (files between compile and link)                             edit
OBJ =$(DIR)$(EXE).o 
```

for the example of `mymodule`

```
# compiles a .o file for mymodule
#$(DIR)mymodule.o : mymodule.c mymodule.h $(SELF)
#   $(CC) $(CFLAGS) -c mymodule.c -o $(DIR)mymodule.o 
```
(note that edit is removed as you no longer need to edit that place.)
and
```
#object files. (files between compile and link)                             edit
OBJ =$(DIR)$(EXE).o mymodule.o
```

Things will be compiled into the folder with the name of the compiler. If you wanna change compiler, Just know it will need to make that folder. 
currently, I suspect 
