local util = require("util")

Console.clear()

---@type Dance
local dance = util.new_dance()

dance.runner.mode = "Once"

dance.on_tick = function(tick)
	Camera.space({ r = 50, rx = 45, rz = tick * 0.1 })
end

---@type Chrome
local chrome_1 = {
	parts = { util.red_ball() },
	on_tick = function(tick)
		local x = math.sin(tick / 50.0) * (8.0 * (tick / dance.runner.max_tick) ^ 2 + 2)
		local y = math.cos(tick / 50.0) * 10.0
		local z = tick * 0.01
		Plotter.push("x coord", x)
		Plotter.push("y coord", y)
		Plotter.push("z coord", z)
		return Transform.from_xyz({
			x = x,
			y = y,
			z = z,
		})
	end,
}

dance.chromes = {
	chrome_1,
}

return dance
