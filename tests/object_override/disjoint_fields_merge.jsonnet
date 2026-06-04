// Disjoint fields merge regardless of operand order (pass_to_pass guard:
// stays green even if override precedence is broken).
std.assertEqual({ a: 1 } + { b: 2 }, { a: 1, b: 2 }) &&
std.assertEqual({ b: 2 } + { a: 1 }, { a: 1, b: 2 }) &&
true
