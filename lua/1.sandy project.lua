local util = require("util")

Console.clear()
Console.print("This only run when the project is loaded/reloaded")

---@type Dance
local dance = util.new_dance()

dance.on_start = function()
	Console.print("This run when the runner is started/restarted")
end

dance.on_tick = function(tick)
	-- this run every tick
	if tick % 10 == 0 then
		Console.print("Tick : " .. tick)
	end
end

dance.runner = {
	mode = "Repeat",
	max_tick = 1000,
	ms_per_tick = 100,
	running = true,
}

---@type Chrome
local chrome_1 = {
	parts = { util.red_ball() },
	on_tick = function(tick)
		return Transform.from_xyz({
			z = tick * 0.01,
		})
	end,
}

dance.chromes = {
	chrome_1,
}

return dance
