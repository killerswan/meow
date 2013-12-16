Kevin Cantu
January 2012

When the two functions in bench.rs are compiled
with any optimization, that DOUBLES the time to
map a hash function over a vector of strings.

For example:
   $ ./run.sh 
   With all at --opt-level 0:
   Loading the lolcat bible...
   Benching djb...      0.128 sec

   With bench.rs at --opt-level 1:
   Loading the lolcat bible...
   Benching djb...      0.309 sec

   With bench.rs at --opt-level 1, RUST_MIN_STACK set higher:
   Loading the lolcat bible...
   Benching djb...      0.137 sec

   With all at --opt-level 1, RUST_MIN_STACK set higher:
   Loading the lolcat bible...
   Benching djb...      0.062 sec


