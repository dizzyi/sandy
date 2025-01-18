local util = require("util")

Console.clear()
Console.print("Hello Choom")

---@type Dance
local dance = util.new_dance()

---@type Chrome
local chrome_1 = {
	parts = { util.red_ball() },
	on_tick = function(tick)
		return Transform.from_xyz({
			-- z = tick * 0.01,
		})
	end,
}

dance.chromes = {
	chrome_1,
}

return dance
