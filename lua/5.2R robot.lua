local util = require("util")
Console.clear()
local dance = util.new_dance()

local q1 = 0
local q2 = 0
local t1 = Transform.from_xyz({})
local t2 = Transform.from_xyz({})

dance.on_tick = function(tick)
	q1 = (tick / dance.runner.max_tick) ^ 2  * 360
	q2 = math.sin(tick / 50) * 90

	t1 = Transform.from_vec_rot({}, Rotation.from_rz(q1))
	t2 = t1 * Transform.from_vec_rot({ y = 6 }, Rotation.from_rz(q2))
end

---@type Material
local material1 = {
	color = Color.rgba({ alpha = 1.0, blue = 0.5 }),
}

---@type Chrome
local chrome1 = {
	parts = {
		{
			mesh = Mesh.capsule_prism({ radius = 1, depth = 0.5, length = 6 }),
			material = material1,
			offset = Transform.from_xyz({ z = 0.25, y = 3 }),
		},
		{
			mesh = Mesh.cuboid({ x = 2, y = 4, z = 0.5 }),
			material = material1,
			offset = Transform.from_xyz({ z = 0.75, y = 3 }),
		},
		{
			mesh = Mesh.capsule_prism({ radius = 1, depth = 0.5, length = 6 }),
			material = material1,
			offset = Transform.from_xyz({ z = 1.25, y = 3 }),
		},
	},
	on_tick = function(tick)
		return t1
	end,
  after_image = util.make_after_image(dance.runner.max_tick, 25)
}

---@type Chrome
local chrome2 = {
	parts = {
		{
			mesh = Mesh.capsule_prism({ radius = 1, depth = 0.25, length = 6 }),
			material = material1,
			offset = Transform.from_xyz({ z = 0.75, y = 3 }),
		},
	},
	on_tick = function(tick)
		return t2
	end,
}

dance.chromes = {
	chrome1,
	chrome2,
}

return dance
