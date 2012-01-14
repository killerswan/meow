echo "With bench.rs at --opt-level 0:"
   rustc --opt-level 0 -L . --lib str2.rs \
&& rustc --opt-level 0 -L . --lib bench.rs \
&& rustc --opt-level 0 -L . main.rs -o main \
&& ./main

echo ""
echo "With bench.rs at --opt-level 1:"
   rustc --opt-level 0 -L . --lib str2.rs \
&& rustc --opt-level 1 -L . --lib bench.rs \
&& rustc --opt-level 0 -L . main.rs -o main \
&& ./main
echo ""

