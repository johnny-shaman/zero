%{
  #include <stdio.h>
  #include <stdlib.h>
  #include <string.h>
  
  // yacc header
  #include "y.tab.h"

  // myfunction header
  #include "fn.h"

  map<string, double> varStack;

  extern "C" int yywrap( void ){
    return 1 ;
  }
%}

%%
" "   {}
"\n"  return CR ;

"<="  return ELS ;
">="  return EMR ;
"->"  return RIT ;
"<-"  return LFT ;

"++"  return INC ;
"--"  return DEC ;
".."  return SPR ;

"<"   return LES ;
">"   return MOR ;

"*"	  return PRD ;
"/"	  return PER ;
"%"   return MOD ;
"+"	  return SUM ;
"-"	  return SUB ;

"="   return EQ ;
":"   return IS  ;
","   return CNM ;
"."   return DOT ;

"&"   return AND ;
"|"   return OR  ;
"^"   return XOR ;
"!"   return NOT ;

"("   return LSP ;
")"   return RSP ;
"{"   return LMP ;
"}"   return RMP ;
"["   return LLP ;
"]"   return RLP ;


[-]*[0-9]+ {

}

[-]*[0-9]+\.[0-9]+ {

}

[$_a-zA-Z]*[$_a-zA-Z0-9]+ {

}
