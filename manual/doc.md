# Camera


```lua
table
```


---

# Camera.plane


```lua
function Camera.plane(plane?: { x: number, y: number, z: number })
```


---

# Camera.space


```lua
function Camera.space(space?: { r: number, rx: number, rz: number })
```


---

# CapsulePrism


---

# Chrome

## after_image


```lua
(fun(tick: integer):Material)?
```

## on_tick


```lua
(fun(tick: integer):Transform)?
```

a function that will be call every tick,
expected to return a Transform.

## parts


```lua
ChromePart[]
```

a list of object what will be rendered


---

# ChromePart

## material


```lua
Material?
```

The material of the mesh

## mesh


```lua
Mesh?
```

The mesh that will be render

## offset


```lua
Transform?
```

The offset from the origin of the chrome


---

# Color

## Hsva


```lua
{ hue: number, saturation: number, value: number, alpha: number }?
```

## LinearRgba


```lua
{ red: number, green: number, blue: number, alpha: number }?
```


---

# Color


```lua
table
```


---

# Color.hsva


```lua
function Color.hsva(hsva: boolean|number|{ hue: number, saturation: number, value: number, alpha: number })
  -> Color
```


---

# Color.rgba


```lua
function Color.rgba(rgba: boolean|number|{ red: number, green: number, blue: number, alpha: number })
  -> Color
```


---

# Console


```lua
table
```


---

# Console.clear


```lua
function Console.clear()
```


---

# Console.print


```lua
function Console.print(msg: string)
```


---

# Cuboid


---

# Cylinder


---

# Dance

## chromes


```lua
Chrome[]
```

Define a list of Chrome

## on_start


```lua
(fun():nil)?
```

A function that will be run on runtime startup, rerun every restart

## on_tick


```lua
(fun(tick: integer):nil)?
```

A function that will be run on every tick

## runner


```lua
Runner
```

Configure the runner of the project


---

# Dance


```lua
table
```


---

# Dance.clear


```lua
function Dance.clear(material: any)
```


---

# HSVA


---

# Material

## clearcoat


```lua
number?
```

TODO

## color


```lua
Color?
```

Define the color of a mesh

## emission


```lua
Color?
```

TODO

## metallic


```lua
number?
```

TODO


---

# Mesh

## CapsulePrism


```lua
{ radius: number, length: number, depth: number }?
```

## Cuboid


```lua
{ x: number, y: number, z: number }?
```

## Cylinder


```lua
{ radius: number, height: number }?
```

## Sphere


```lua
{ radius: number }?
```


---

# Mesh


```lua
table
```


---

# Mesh.capsule_prism


```lua
function Mesh.capsule_prism(capsule_prism: number|{ radius: number, length: number, depth: number }|nil)
  -> Mesh
```


---

# Mesh.cuboid


```lua
function Mesh.cuboid(cuboid: number|{ x: number, y: number, z: number }|nil)
  -> Mesh
```


---

# Mesh.cylinder


```lua
function Mesh.cylinder(cylinder: number|{ radius: number, height: number }|nil)
  -> Mesh
```


---

# Mesh.sphere


```lua
function Mesh.sphere(sphere: number|nil)
  -> Mesh
```


---

# Plotter


```lua
table
```


---

# Plotter.aspect


```lua
function Plotter.aspect(aspect: any)
```


---

# Plotter.auto


```lua
function Plotter.auto(auto: any)
```


---

# Plotter.clear


```lua
function Plotter.clear()
```


---

# Plotter.push


```lua
function Plotter.push(name: string, y: number)
```


---

# RGBA


---

# Rotation


---

# Rotation


```lua
table
```


---

# Rotation.from_rx


```lua
function Rotation.from_rx(angle: number)
  -> Rotation
```


---

# Rotation.from_ry


```lua
function Rotation.from_ry(angle: number)
  -> Rotation
```


---

# Rotation.from_rz


```lua
function Rotation.from_rz(angle: number)
  -> Rotation
```


---

# Runner

## max_tick


```lua
integer
```

It define how long your dance will run for,

on "Once" mode, it will halt after max tick is reach,

and on "Repeat" mode, it will restart the run time.

## mode


```lua
"Once"|"Repeat"
```

Either "Once" or "Repeat"

## ms_per_tick


```lua
integer
```

It define how long a tick is suppose to be in milliseconds,
however, the minimum tick time depend on the FPS of your machine

## running


```lua
boolean
```

define if the runner should have a default start


---

# Sphere


---

# Transform

## get_x


```lua
fun(Transform: any):number
```

## get_y


```lua
fun(Transform: any):number
```

## get_z


```lua
fun(Transform: any):number
```


---

# Transform


```lua
table
```


---

# Transform.from_vec_rot


```lua
function Transform.from_vec_rot(xyz: { x: number, y: number, z: number }|nil, rot: Rotation|nil)
  -> Transform
```


---

# Transform.from_xyz


```lua
function Transform.from_xyz(xyz: { x: number, y: number, z: number }|nil)
  -> Transform
```


---

# Vector


---

# Vector


```lua
table
```


---

# Vector.new


```lua
function Vector.new(xyz: { x: number, y: number, z: number })
  -> Vector
```


---

# XYZ