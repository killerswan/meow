echo "With all at --opt-level 0:"
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


# In order for bench.rs not to slow way down:
RUST_MIN_STACK=1000000
export RUST_MIN_STACK

echo ""
echo "With bench.rs at --opt-level 1, RUST_MIN_STACK set higher:"
   rustc --opt-level 0 -L . --lib str2.rs \
&& rustc --opt-level 1 -L . --lib bench.rs \
&& rustc --opt-level 0 -L . main.rs -o main \
&& ./main

echo ""
echo "With all at --opt-level 1, RUST_MIN_STACK set higher:"
   rustc --opt-level 1 -L . --lib str2.rs \
&& rustc --opt-level 1 -L . --lib bench.rs \
&& rustc --opt-level 1 -L . main.rs -o main \
&& ./main

echo ""

