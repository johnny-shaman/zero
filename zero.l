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
" "   return SP ;
"\n"  return CR ;

"<="  return ELS ;
"=<"  return ELS ;
">="  return EMR ;
"=>"  return EMR ;

"<<"  return SHL ;
">>"  return SHR ;

"->"  return RTA ;
"<-"  return LTA ;

".."  return SCT ;

"++"  return INC ;
"--"  return DEC ;

"<"   return LES ;
">"   return MOR ;

"*"	  return PRD ;
"/"	  return PER ;
"%"   return MOD ;
"+"	  return SUM ;
"-"	  return SUB ;

"&"   return AND ;
"!"   return NOT ;


"="   return EQ ;
":"   return CLN ;
";"   return SCN ;
","   return CNM ;
"."   return DOT ;

"|"   return PIP ;

"("   return LSP ;
")"   return RSP ;
"["   return LLP ;
"]"   return RLP ;

"_"   return DRP ;


[-]*[0-9]+ {

}

[-]*[0-9]+\.[0-9]+ {

}

[\$_a-zA-Z]*[\$_a-zA-Z0-9]+ {

}
