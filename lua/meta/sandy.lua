---@meta

---API for controlling camera position
Camera = {}

---Set the camera to planar mode top view of the world
---in this mode, the camera always point straight down
---@param plane? {x:number, y:number, z:number}
function Camera.plane(plane) end
---Set the camera to spatial mode default 3D view of the world
---in this mode, the camera always point to the origin
---@param space? {r:number, rx:number, rz:number}
function Camera.space(space) end

---API for logging at the console
Console = {}
---Print a log message to the console
---@param msg string
function Console.print(msg) end

---API for plotting
Plotter = {}

---Push a data point on to a plot specify by name
---@param name string
---@param y number | integer
function Plotter.push(name, y) end
---Clear all the recorded point.
function Plotter.clear() end
---Set the plot to be auto bounded
function Plotter.auto(auto) end
---Set the aspect ration of the plotting window
function Plotter.aspect(aspect) end

---The table that the project script should return
---it define how the project need to be runs
---@class (exact) Dance
---Configure the runner of the project
---@field runner Runner
---Define a list of Chrome
---@field chromes Chrome[]
---A function that will be run on runtime startup, rerun every restart
---@field on_start? fun():nil
---A function that will be run on every tick
---@field on_tick? fun(tick:integer):nil

---API for adding after image in the engine
Dance = {}
---When call, it will spawn after image with given material for all chromes
---multiple call within in one tick will only result in one after image
---@param material Material
function Dance.after_image(material) end

---It define controllable object that will be render
---
---it is compose of multiple different ChromePart,
---which will define what to render.
---
---and on each tick, the `on_tick` function will be call,
---and a Transfrom will be expected and it will define
---where the object will be rendered.
---@class Chrome
---a list of object what will be rendered
---@field parts ChromePart[]
---a function that will be call every tick,
---expected to return a Transform.
---@field on_tick? fun(tick: integer):Transform

---It defines a part of the chrome that will be rendered
---
---it is composed by a mesh, a material and the offset
---@class ChromePart
---The mesh that will be render
---@field mesh? Mesh
---The material of the mesh
---@field material? Material
---The offset from the origin of the chrome
---@field offset? Transform

---It defines the runtime of the engine.
---@class (exact) Runner
---Either "Once" or "Repeat"
---@field mode "Once" | "Repeat"
---It define how long a tick is suppose to be in milliseconds,
---however, the minimum tick time depend on the FPS of your machine
---@field ms_per_tick integer
---It define how long your dance will run for,
---
---on "Once" mode, it will halt after max tick is reach,
---
---and on "Repeat" mode, it will restart the run time.
---@field max_tick integer
---define if the runner should have a default start
---@field running boolean

---Define the color of a mesh
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

---Define the material
---@class (exact) Material
---@field color? Color
---TODO
---@field emission? Color
---TODO
---@field metallic? number
---TODO
---@field clearcoat? number

---Define the mesh
---
---you should not construct this class directly,
---the prefer way to make a define a new mesh is to call helper function
---@class (exact) Mesh
---@field Sphere? Sphere
---@field Cuboid? Cuboid
---@field Cylinder? Cylinder
---@field CapsulePrism? CapsulePrism

---API for help constructing mesh definitions.
Mesh = {}

---@alias Sphere {radius: number}
---construct a new sphere mesh with radius
---@param sphere nil | number
---@return Mesh
function Mesh.sphere(sphere) end

---@alias Cuboid {x:number, y:number, z:number}
---construct a new cube like mesh with it dimension
---@param cuboid nil | number | Cuboid
---@return Mesh
function Mesh.cuboid(cuboid) end

---@alias Cylinder {radius:number, height:number}
---construct a new cylinder with radius and height
---@param cylinder nil | number | Cylinder
---@return Mesh
function Mesh.cylinder(cylinder) end

---@alias CapsulePrism {radius:number, length:number, depth:number}
---construct a pill shape prism with radius, lenght between center and depth
---@param capsule_prism nil | number | CapsulePrism
---@return Mesh
function Mesh.capsule_prism(capsule_prism) end

---@alias XYZ {x:number,y:number,z:number}

---It define a spatial transform,
---composed from a translation and rotation.
---
---it is not a lua table, 
---you need to construct it with sandy buildin function
---@class Transform
---@operator mul(Transform) : Transform

---API for constructing Transform
Transform = {}

---construct a new transform from xyz
---@param xyz XYZ | nil
---@return Transform
function Transform.from_xyz(xyz) end
---construct a new transform from xyz and rotation
---@param xyz XYZ | nil
---@param rot Rotation | nil
---@return Transform
function Transform.from_vec_rot(xyz,rot) end

---It define a spatial translation,
---
---it is not a lua table, 
---you need to construct it with sandy buildin function
---@class Vector

---API for construction Vector
Vector = {}

---construct a new vector from xyz
---@param xyz XYZ
---@return Vector
function Vector.new(xyz) end

---It define a spatial rotation,
---
---it is not a lua table, 
---you need to construct it with sandy buildin function
---@class Rotation

---API for construction Rotation
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
