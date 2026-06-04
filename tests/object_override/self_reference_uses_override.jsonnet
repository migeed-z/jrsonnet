// `self` inside an inherited field resolves to the final (overridden) value,
// so the child's override is visible to the super's computed field.
std.assertEqual(
  {
    name: 'Alice',
    welcome: 'Hello ' + self.name + '!',
  } + {
    name: 'Bob',
  },
  { name: 'Bob', welcome: 'Hello Bob!' },
) &&
true
