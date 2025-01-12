# Camera

## plane


```lua
function Camera.plane(plane?: { x: number, y: number, z: number })
```

Set the camera to planar mode top view of the world

## space


```lua
function Camera.space(space?: { r: number, rx: number, rz: number })
```

Set the camera to spatial mode default 3D view of the world


---

# Camera


```lua
Camera
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

## on_tick


```lua
(fun(tick: integer):Transform)?
```

## parts


```lua
ChromePart[]
```


---

# ChromePart

## material


```lua
Material?
```

## mesh


```lua
Mesh?
```

## offset


```lua
Transform?
```


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

## print


```lua
function Console.print(msg: string)
```

Print a log message to the console


---

# Console


```lua
Console
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


```lua
table
```


---

# Dance

## chromes


```lua
Chrome[]
```

## on_start


```lua
(fun():nil)?
```

## on_tick


```lua
(fun(tick: integer):nil)?
```

## runner


```lua
Runner
```


---

# Dance.after_image


```lua
function Dance.after_image(material: Material)
```


---

# HSVA


---

# Material

## clearcoat


```lua
number?
```

## color


```lua
Color?
```

## emission


```lua
Color?
```

## metallic


```lua
number?
```


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

## aspect


```lua
function Plotter.aspect(aspect: any)
```

## auto


```lua
function Plotter.auto(auto: any)
```

## clear


```lua
function Plotter.clear()
```

## push


```lua
function Plotter.push(name: string, y: number)
```

Push a data point on to a plot specify by name


---

# Plotter


```lua
Plotter
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


```lua
table
```


---

# Rotation


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

## mode


```lua
"Once"|"Repeat"
```

## ms_per_tick


```lua
integer
```

## running


```lua
boolean
```


---

# Sphere


---

# Transform


---

# Transform


```lua
table
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