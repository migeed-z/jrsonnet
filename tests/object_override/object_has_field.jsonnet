// std.objectHas reports whether a visible field is present.
std.assertEqual(std.objectHas({ a: 1, b: 2 }, 'a'), true) &&
std.assertEqual(std.objectHas({ a: 1 }, 'missing'), false) &&
true
