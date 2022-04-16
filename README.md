
# zero language

## ex hello world
```
main _ -> 
  say 'Hello World'

  say <- ('Hello' ' ' 'World')

  // to = map
  say
    to upper 'Hello World'

  say <- ('Hello' ' ' 'World')
```

## ex Functional 1
```
mul : * //Type of mul is *
dbl : mul 2
say dbl 3
```

## ex Functional 2
```
myFn a b -> a ** 2 + a * b // take 2 arges
myFn2 : myFn 2             // carrying

say myFn2 3

say 4 myFn 7               // Can write Other side
```

## ex Functional 3
```
//Can use ',' or ';'

R f x -> f x , x

// Write Otherwise (Looks like a Pascal)

R f x ->
  f x
  x

say R x -> ('Left' x) 'Right' // Output only 'Right'

```

## Tuple
```
myT : (1 'test' 2 3 4 5)

// function can take a Tuple and it spread automatic

first : id

first myT
myT first

second _ x -> x

second myT
myT second

//Higher
myT filter int | map * 2 | fold +
```

## Dictionaly (Structure)
```
myStationary : 
  pencil : 3
  eraser :
    type   : 'soft'
    number : 4
  pen    : 1
```

## calculate
```
x : 1 // let x = 1

y : x + 3
y : x - 3
y : x * 2
y : x / 2
y : x ** 2
y : x ** 1/2


//calculate redefinition

x++     //increment
x--     //declement

x :+ 3  // Add
x :- 3  // Sub

x :* 2    // Multi
x :** 2   // Power
x :** 1/2 // SqR
x :/ 2    // Divide

x = 20      // Equal
x < 20      // Less
x > 20      // More

z :
  a : 1
  b : 2
  a + b

z x ->
  a : 3
  b : 4
  a + b - x
```

## structure Abstruction
```
ProfileA : o ->
  //declare public (Public Optional = Undeclare)
  first
  father

  //Can declare private (It's a option but declared Type must to take!)
  ! hobby

  //declare private optional
  ? job

  //declare Product Type(Can private)
  ! address | tel | email \.*[@]+.*\
  twitter \[@]+.*\ | facebook

  // declare Method
  name : (first ' ' last)

// deregation
ProfileB : ProfileA ->
  hobby
  sayAge x : ("I'm " (age - x) "years.")

luke k : ProfileA
  first  : 'Luke'
  last   : 'Skywalker'
  hobby  : 'Electronics'
  age    : 18
  father k : ProfileB
    first  : 'Anakin'
    last // auto delegation from luke
    hobby  : 'Electronics'
    age    : 100

say luke name // Luke Skywalker

say x -> luke x 'first'

say luke age //none

say luke father age //infinity

```
## case
```
case x
  'Luke' : 'first'
  'Skywalker' : 'last'
  18 : 'age'
  'Anakin' : 'father'
```

// lift flat (Pointer)

```
x : 5
y :> x // lift
z <: y // flat

a <: 'abc' // (char, char, char) <: String
to equal a ('a' 'b' 'c') // true


```

// export
// @ : Reference global

```
@ :> x -> x
@ :> x y -> (x y)
```


// import
```
IO <: @ stdio  // in memory
Vector2D <: @ ./vector2D // file
```
