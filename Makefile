zero: y.tab.o lex.yy.o fn.o
	g++ -o zero y.tab.o lex.yy.o myFunction.o

y.tab.o: zero.y
	g++ -c y.tab.c

lex.yy.o: zero.l
	flex -l zero.l
	g++ -c lex.yy.c

fn.o: fn.c
	g++ -c fn.c

clean:
	rm mycalc y.tab.c y.tab.h lex.yy.c *.o
