local util = require("util")

Console.clear()

---@type Dance
local dance = util.new_dance()

dance.on_tick = function(tick) end

local after_image_fn = util.make_after_image(dance.runner.max_tick, 10);

---@type Chrome
local chrome_1 = {
	parts = {
		{
			mesh = Mesh.sphere(1.5),
			material = {
				color = Color.rgba({ red = 0.8, blue = 0.3, alpha = 1.0 }),
			},
			offset = Transform.from_xyz(),
		},
		{
			mesh = Mesh.cylinder({ radius = 1, height = 1.5 }),
			material = {
				color = Color.rgba({ green = 0.8, blue = 0.3, alpha = 1.0 }),
			},
			offset = Transform.from_xyz({ z = 0.5 }),
		},
	},
	on_tick = function(tick)
		local x = math.sin(tick / 200.0) * 10.0
		local y = math.cos(tick / 200.0) * 10.0 local z = 0
		return Transform.from_xyz({
			x = x,
			y = y,
			z = z,
		})
	end,
  after_image = after_image_fn
}
---@type Chrome
local chrome_2 = {
	parts = { util.blue_cube() },
	on_tick = function(tick)
		local x = math.cos(tick / 200.0) * 10.0
		local y = math.sin(tick / 200.0) * 10.0
		local z = math.sin(tick / 50.0) * 2 + 5
		return Transform.from_xyz({
			x = x,
			y = y,
			z = z,
		})
	end,
  -- after_image = after_image_fn
}

dance.chromes = {
	chrome_1,
	chrome_2,
}

return dance
