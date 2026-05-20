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
$struct User {
	name: str,
	email: str,
}

let user: $User = $User!{ name: "John", email: "john@example.com" }
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

In
```
$struct Pair [any, any]

let pair: $Pair = $Pair![42, "Hello"]
<: Pair[0]
<: Pair[1]
```
Out
```
let pair = [42, "Hello"]
<: pair[0]
<: pair[1]
```

---

### Enum
#### Examples

In
```
$enum Shape {
	Rectangle {
		width: num,
		height: num,
	},
	Text {
		text: str,
	},
}

@show(shape: Shape): void {
	if shape $is $Shape:Rectangle {
		<: shape.width
		<: shape.height
	} else {
		<: shape.text
	}
}

let rectangle: Shape = $Shape:Rectangle!{ width: 10, height: 20 }
show(rectangle)
let text: Shape = $Shape:Text!{ text: "Hello" }
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

In
```
$enum Option {
	None [],
	Some [any],
}

let none = $Option:None![]
let some = $Option:Some!["Hello"]
```

Out
```
let none = [0]
let some = [1, "Hello"]
```

---

In
```
$enum Fruit {
	Apple,
	Banana,
}

let fruit = $Fruit:Apple
<: fruit $is $Fruit:Apple
```

Out
```
let fruit = 0
<: fruit == 0
```

---
