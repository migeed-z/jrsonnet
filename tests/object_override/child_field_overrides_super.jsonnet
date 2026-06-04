// In `A + B`, a field in B (the child / right operand) overrides the same
// field in A (the super / left operand).
std.assertEqual(({ a: 1 } + { a: 2 }).a, 2) &&
std.assertEqual({ a: 1, b: 10 } + { a: 2 }, { a: 2, b: 10 }) &&
true
