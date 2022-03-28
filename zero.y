%{
  #include "fn.h"
  #include <stdio.h>
  #include <stdlib.h>
  #include <string.h>
  #include <iostream>
  #include <map>
  using namespace std;


  // yaccが定義する内部関数のプロトタイプ宣言
  extern int yyerror ( const char* ) ;
  extern int yyparse ( void ) ;
  extern int yylex ( void ) ;
  extern char* yytext ;
  extern FILE* yyin ;
%}
