// std.length(object) is the count of visible fields.
std.assertEqual(std.length({ a: 1, b: 2, c: 3 }), 3) &&
std.assertEqual(std.length({ single: 1 }), 1) &&
true
