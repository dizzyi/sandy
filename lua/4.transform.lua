local util = require("util")
Console.clear()

---@type Dance
local dance = util.new_dance()

local after_image_fn = util.make_after_image(dance.runner.max_tick, 10)

local q1 = 0
local q2 = 0
local t1 = Transform.from_xyz({})
local t2 = Transform.from_xyz({})

dance.on_tick = function(tick)
	q1 = math.sin(tick / 100) * 60
	q2 = math.sin(tick / 50) * 30

	t1 = Transform.from_vec_rot({}, Rotation.from_rz(q1))
	t2 = t1 * Transform.from_vec_rot({ y = 5 }, Rotation.from_rz(q2))
end

---@type Chrome
local chrome_1 = {
	parts = {
		{
			mesh = Mesh.cylinder({ radius = 0.5, height = 5 }),
			material = {
				color = Color.rgba({ red = 0.5, alpha = 1 }),
			},
			offset = Transform.from_vec_rot({ y = 2.5 }, {}),
		},
	},
	on_tick = function(tick)
		return t1
	end,
  after_image = after_image_fn
}
---@type Chrome
local chrome_2 = {
	parts = {
		{
			mesh = Mesh.cylinder({ radius = 0.25, height = 5 }),
			material = {
				color = Color.rgba({ blue = 0.5, alpha = 1 }),
			},
			offset = Transform.from_vec_rot({ y = 2.5 }, {}),
		},
	},
	on_tick = function(tick)
		return t2
	end,
  after_image = after_image_fn
}

dance.chromes = {
	chrome_1,
	chrome_2,
}

return dance
