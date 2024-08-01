################################################################################
#
#       Makefile for CCP Plants, should work for gcc and clang
#
################################################################################

#edit finder, locate commonly eddited places >>edit<<

############################################################################
# Macros
############################################################################

#name of exe
EXE = cplants
#directory to compile to. if needed to be changed. 
DIR = ./$(CC)/
#object files. (files between compile and link)	                            edit
OBJ =$(DIR)$(EXE).o 
#Compiler name (default should be gcc)
CC =gcc

############################################################################
# Overide flags
############################################################################ 
#   allows for adding flags instead of overwriting them.

#compiler flags, 
override CFLAGS+= -g -O -Wall -Wextra -Werror -pedantic #-pthread 
#Linker flags, 
override LFLAGS+= #-lpthread 

#used for diff, should be changed.
#input for test against 
TIN = output.txt
#output file for test.
TOUT = my$(TIN)

############################################################################
# Helpers
###########################################################################

#output silencer, will only print errors, like the old ways.
ifndef VERBOSE
.SILENT:
endif

# If the first argument is "run"...
ifeq (run,$(firstword $(MAKECMDGOALS)))
	# use the rest as arguments for "run"
	RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
	# ...and turn them into do-nothing targets
	$(eval $(RUN_ARGS):;@:)
endif

# If the first argument is "valg"...
ifeq (valg,$(firstword $(MAKECMDGOALS)))
	# use the rest as arguments for "valg"
	RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
	# ...and turn them into do-nothing targets
	$(eval $(RUN_ARGS):;@:)
endif

#NOTE: if run args are getopt you must type '--' before.

############################################################################
# Main Make.
############################################################################

# links the program, Default recipy.
$(DIR)$(EXE) : $(OBJ) 
	$(CC) $(OBJ) -o $(DIR)$(EXE) $(LFLAGS)
	-$(MAKE) tablen

# compiles a .o file for the "main" file.
$(DIR)$(EXE).o : $(EXE).c 
	$(CC) $(CFLAGS) -c $(EXE).c  -o $(DIR)$(EXE).o 

#...	                                                                    edit

# compiles a .o file for any template                                       edit
#$(DIR)TEMPLATE.o : TEMPLATE.c TEMPLATE.h $(SELF)
#	$(CC) $(CFLAGS) -c TEMPLATE.c -o $(DIR)TEMPLATE.o 

############################################################################
# Commands recipys.
############################################################################

# Cleans the directory.
clean :
	rm $(DIR)$(EXE) $(OBJ)

# throws out old code and forces a recompile and link.
new : 
	-$(MAKE) clean
	-$(MAKE)

# runs the exe from your desired directory.
.PHONY: run
run :
	$(DIR)$(EXE) $(RUN_ARGS)

#forces recompile and link then runs.
newRun :
	-$(MAKE) new
	-$(MAKE) run


# creats output from run and compares it to test.
diff :
	-$(MAKE) run > $(DIR)$(TOUT)
	diff $(DIR)$(TOUT) $(TIN) --strip-trailing-cr

# runs doctor memory on code. NOTE: this may take a while. Obvi not in systems that dont have Dr Memory. 
drmem:
	-$(MAKE) clean
	-$(MAKE) CFLAGS+=-g
	drmemory -batch -no_summary -- $(DIR)$(EXE)

# runs valgrind on code to check for memory leaks.
.PHONY: run
valg:
	-$(MAKE) clean
	-$(MAKE) CFLAGS+=-g
	valgrind -q --tool=memcheck --leak-check=full --show-reachable=yes --trace-children=yes $(DIR)$(EXE) $(RUN_ARGS)
