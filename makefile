NAME=cassette

SH       = sh
RM       = rm -f
CC       = clang
ASTYLE   = astyle
DEFINES  =
INCLUDES = -I.
WARNINGS = -Wextra -Wall -Werror -Wno-unused-parameter -Wno-unused-function
CFLAGS   = -Os -g $(DEFINES) $(WARNINGS) $(INCLUDES)
LDLIBS   =
LDDIRS   =
LDFLAGS  = $(LDDIRS) $(LDLIBS)


SRCS =

HDRS =

OBJS     = $SRCS:%.c=%.o)
EOBJ     = main.o
TEST_OBJ = test.o

TARGETS = $(NAME)

.PHONY: all format clean package

all: $(TARGETS)

format::
	@echo "FMT  $(SRCS) $(HDRS)"
	@$(ASTYLE) --options=astylerc main.c $(SRCS) $(HDRS) --suffix=none --quiet

clean::
	$(RM) $(TARGETS) $(OBJS) $(EOBJ)

package:: $(TARGETS)
	rm -rf bin
	mkdir bin
	cp $(TARGETS) bin/
	tar cf $(NAME).tar bin/
	gzip $(NAME).tar
	rm -rf bin

test: $(TEST_OBJ) $(OBJS) $(HDRS)
	@echo " LD  $@"
	@$(CC) -o $@ $(TEST_OBJ) $(OBJS) $(LDFLAGS) $(LDLIBS)

%.o: %.c
	@echo " CC  $<"
	$(CC) -c $< -o $@ $(CFLAGS)

$(NAME): $(EOBJ) $(OBJS) $(HDRS)
	@echo " LD  $@"
	$(CC) -o $@ $(EOBJ) $(OBJS) $(LDFLAGS)
