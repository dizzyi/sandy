local sandy = require("lib/sandy")

Console.print("Hello Choom!")
Console.print("This is a message!")

Camera.space()
-- Camera.plane();
Plotter.clear()

---@type Dance
local dance = sandy.new_dance()

dance.runner = {
  running = true,
  ms_per_tick = 10,
  max_tick = 1000,
  mode = "Repeat"
}

dance.on_start = function()
	Plotter.clear()
	Plotter.auto(true)
end

dance.on_tick = function (tick)
	-- Camera.space({ r = 75, rz = tick / 10 });
  if tick % 50 == 0 then
    local m = { color = Color.hsva({hue=tick/1000*360, saturation=1, value=1, alpha=0.4}) }
    Dance.after_image(m);
  end
end

---@type Chrome
local chrome_1 = {
	on_tick = function(tick)
		-- local y = ( 2 + (tick / 1000) ^ 2 * 8) * math.sin(tick * 0.05)
		-- local x = 10 * math.cos(tick * 0.05)
		-- local z = tick * 0.01
		-- Plotter.push("Chrome 1 y coord", y)
		-- Plotter.push("Chrome 1 x coord", x)
		-- Plotter.push("Chrome 1 z coord", z)
		-- Plotter.push("Chrome 1 r coord", math.sqrt(z * z + x * x + y * y));
		--   local vec = Vector.new({x=x,y=y,z=z});
		--   local rot = Rotation.from_rx(tick * 1);
		-- return Transform.from_vec_rot(vec, rot);
    local A = Transform.from_xyz({z = 2})
    local B = Transform.from_vec_rot(
      {z=5},
      Rotation.from_rz(tick * 1)
    );
    local C = Transform.from_vec_rot(
      {},
      Rotation.from_rx(tick * 0.1 - 45)
    );
    local D = C * B * A;
    return D
  end,
	parts = {
		{
			mesh = Mesh.cuboid(2.0),
			material = {
				color = Color.rgba({ green = 0.8, blue = 0.1, alpha = 1 }),
        emission = Color.rgba({red=1.0})
			},
			-- offset = Transform.from_xyz({ z = 1 }),
		},
		-- {
		--   mesh = Mesh.sphere(1.0),
		--   material = {
		--     color = Color.rgba({red=0.8, blue=0.1}),
		--   },
		--   offset = Transform.from_xyz({z=2});
		-- },
		-- {
		--   mesh = Mesh.sphere(1.0),
		--   material = {
		--     color = Color.rgba({blue=0.8, red=0.1}),
		--   },
		--   offset = Transform.from_xyz({z=3});
		-- }
	},
}

dance.chromes = {
	chrome_1,
}

return dance
