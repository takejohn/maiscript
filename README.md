# mAiScript: A programming language to generate effectively minified AiScript code

**!! WORK IN PROGRESS !!**

## Features
- A superset of AiScript
- Nominal typing
- Static binding

## Syntax

### Struct
#### Examples

In
```
struct User {
	name: str,
	email: str,
}

let user: User = User { name: "John", email: "john@example.com" }
<: user.name
<: user.email
```
Out
```
let user = { a: "John", b: "john@example.com" }
<: user.a
<: user.b
```

---

### Enum
#### Examples

In
```
enum Fruit {
	Apple,
	Banana,
}

let fruit = Fruit:Apple
<: fruit == Fruit:Apple
```

Out
```
let fruit = 0
<: fruit == 0
```

---

### Tagged Union
#### Examples

In
```
enum Tag {
	Num,
	Str,
}

union Tagged(tag: Tag) {
	Num {
		value: num,
	},
	Str {
		value: str,
	},
}

let num: Tagged = Tagged:Num { value: 42 }
<: num.tag == Tag:Num
```

Out
```
let tagged = { a: 0, b: 42 }
```
---

In
```
union Shape(tag) {
	Rectangle {
		width: num,
		height: num,
	},
	Text {
		text: str,
	},
}

@show(shape: Shape): void {
	if shape.tag == Shape:Rectangle {
		<: shape.width
		<: shape.height
	} else {
		<: shape.text
	}
}

let rectangle: Shape = Shape:Rectangle { width: 10, height: 20 }
show(rectangle)
let text: Shape = Shape:Text { text: "Hello" }
show(text)
```

Out
```
@show(a) {
	if a.a == 0 {
		<: a.b
		<: a.c
	} else {
		<: a.b
	}
}

let rectangle = { a: 0, b: 10, c: 20 }
show(rectangle)
let text = { a: 1, b: "Hello" }
show(text)
```

---

### Visibility Modifiers

#### Examples

In
```
:: Ns {
	private let hidden = 42

	@func() {
		<: hidden
	}
}

// ns:hidden // Error
ns:func()
```

Out
```
:: Ns {
	// Private variables can be renamed for minification.
	let a = 42

	@func() {
		<: a
	}
}

Ns:func()
```

---
