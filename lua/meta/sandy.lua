---@meta

---@class Camera
Camera = {}

---Set the camera to planar mode top view of the world
function Camera.plane() end
---Set the camera to spatial mode default 3D view of the world
function Camera.space() end

---@class Console
Console = {}
---Print a log message to the console
---@param msg string
function Console.print(msg) end

---@class Plotter
Plotter = {}

---Push a data point on to a plot specify by name
---@param name string
---@param y number | integer
function Plotter.push(name, y) end
function Plotter.clear() end

---@class (exact) Dance
---@field runner Runner
---@field chromes Chrome[]
---@field on_start? fun():nil

---@class Chrome
---@field parts ChromePart[]
---@field on_tick? fun(tick: integer):Transform

---@class ChromePart
---@field mesh? Mesh
---@field material? Material
---@field offset? Transform


---@class (exact) Runner
---@field mode "Once" | "Repeat"
---@field ms_per_tick integer
---@field max_tick integer
---@field running boolean


---@class Color
---@field LinearRgba? RGBA
---@field Hsva? HSVA

Color = {}
---@alias RGBA {red:number, green:number, blue:number, alpha:number}
---construct a new color with rgba value
---@param rgba boolean | number | RGBA
---@return Color
function Color.rgba(rgba) end
---@alias HSVA {hue:number, saturation:number, value:number, alpha:number}
---construct a new color with hsva value
---@param hsva boolean | number | HSVA
---@return Color
function Color.hsva(hsva) end

---@class (exact) Material 
---@field color? Color
---@field emission? Color
---@field metallic? number
---@field clearcoat? number

---@class (exact) Mesh
---@field Sphere? Sphere
---@field Cuboid? Cuboid
---@field Cylinder? Cylinder
---@field CapsulePrism? CapsulePrism

Mesh = {}
---@alias Sphere {radius: number}
---construct a new sphere mesh with radius
---@param sphere nil | number
---@return Mesh
function Mesh.sphere(sphere) end
---
---@alias Cuboid {x:number, y:number, z:number}
---construct a new cuboid mesh with radius
---@param cuboid nil | number | Cuboid
---@return Mesh
function Mesh.cuboid(cuboid) end
---
---@alias Cylinder {radius:number, height:number}
---construct a new cuboid mesh with radius
---@param cylinder nil | number | Cylinder
---@return Mesh
function Mesh.cylinder(cylinder) end
---
---@alias CapsulePrism {radius:number, length:number, depth:number}
---construct a new cuboid mesh with radius
---@param capsule_prism nil | number | CapsulePrism
---@return Mesh
function Mesh.capsule_prism(capsule_prism) end


---@alias XYZ {x:number,y:number,z:number}

---@class Transform

Transform = {}

---construct a new transform from xyz
---@param xyz XYZ | nil
---@return Transform
function Transform.from_xyz(xyz) end

---@class Vector

Vector = {}

---construct a new vector from xyz
---@param xyz XYZ
---@return Vector
function Vector.new(xyz) end


---@class Rotation

Rotation = {}

---construct a new rotation from rx
---@param angle number
---@return Rotation
function Rotation.from_rx(angle) end
---construct a new rotation from ry
---@param angle number
---@return Rotation
function Rotation.from_ry(angle) end
---construct a new rotation from rz
---@param angle number
---@return Rotation
function Rotation.from_rz(angle) end

