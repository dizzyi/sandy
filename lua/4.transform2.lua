local util = require("util")

Console.print("Hello Choom!")
Console.print("This is a message!")

Camera.space()
Plotter.clear()

---@type Dance
local dance = util.new_dance()

dance.runner = {
	running = true,
	ms_per_tick = 10,
	max_tick = 1000,
	mode = "Once",
}

dance.on_start = function()
	Plotter.clear()
	Plotter.auto(true)
end

dance.on_tick = function(tick)
	-- Camera.space({ r = 75, rz = tick / 10 });
	-- if tick % 50 == 0 then
	-- 	local m = { color = Color.hsva({ hue = tick / 1000 * 360, saturation = 1, value = 1, alpha = 0.4 }) }
	-- 	Dance.after_image(m)
	-- end
end

local after_image_fn = util.make_after_image(dance.runner.max_tick, 10)

---@type Chrome
local chrome_1 = {
	on_tick = function(tick)
		Console.print("tick : " .. tick)
		local A = Transform.from_xyz({ z = 2 })
		local B = Transform.from_vec_rot({ z = 5 }, Rotation.from_rz(tick * 1))
		local C = Transform.from_vec_rot({}, Rotation.from_rx(tick * 0.1 - 45))
		local D = C * B * A
    Plotter.push("y coord", D:get_y())
    Plotter.push("z coord", D:get_z())
		return D
	end,
	parts = {
		{
			mesh = Mesh.cuboid(2.0),
			material = {
				color = Color.rgba({ green = 0.8, blue = 0.1, alpha = 1 }),
			},
		},
	},
  after_image = after_image_fn
}

dance.chromes = {
	chrome_1,
}

return dance
