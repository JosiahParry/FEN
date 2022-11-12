/*
Comments:
  - rest of line: indicated by a semicolon the rest of the line is a comment
  - braces {} (curly brackets) 
  
Escapes:
 - lines that start with `%` are ignored 

Tokens: 
 - pgn is made up of tokens which are sequences of characters
   - delimited by whitespace
 
*/

/* Parsing guide */
/* 

2 parts:

Header:
    - tag pair at header
        - 7 standard tags 
        - can only contain digits, letters, underscores
        - tags used for storage have uppercase first character
        - multiple values can be provided in an element deliited by a colon
            - e.g. [Players "Name:Other:Person"]
            - colonrs shouldn't appear elsewhere because of this

    - standard tags:
        1) Event (the name of the tournament or match event)
        2) Site (the location of the event)
        3) Date (the starting date of the game)
        4) Round (the playing round ordinal of the game)
        5) White (the player of the white pieces)
        6) Black (the player of the black pieces)
        7) Result (the result of the game)

Move Parts

*/

/* 
with notation like Ne7, Bb7 etc. we  don't have origination information
we have to determine from the existing pieces,  which can make that move
for a move like Ne7 we have to determine if there are 2 nights on the board 
    (can be parsed from the FEN easily) and if only 1, that piece moves
    
*/