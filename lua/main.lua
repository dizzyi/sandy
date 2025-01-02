local sandy = require "lib/sandy";

Console.print("Hello Choom!");
Console.print("This is a message!");

Camera.space();
-- Camera.plane();
Plotter.clear();

---@type Dance
local dance = sandy.new_dance();


---@type Chrome
local chrome_1 = {
  on_tick = function (tick)
    -- Console.print("ticking . . ." .. tick);
    local y = 10 * math.sin(tick * 0.05);
    local x = 10 * math.cos(tick * 0.05);
    local z = tick * 0.01;
    Plotter.push("Chrome 1 y coord", y);
    Plotter.push("Chrome 1 x coord", x);
    Plotter.push("Chrome 1 z coord", z);
    -- Plotter.push("Chrome 1 r coord", math.sqrt(z * z + x * x + y * y));
    return Transform.from_xyz({
      z = z,
      x = x,
      y = y,
    })
  end,
  parts = {
    {
      mesh = Mesh.sphere(1.0),
      material = {
        color = Color.rgba({green=0.8, blue=0.1}),
      },
      offset = Transform.from_xyz({z=1});
    },
    {
      mesh = Mesh.sphere(1.0),
      material = {
        color = Color.rgba({red=0.8, blue=0.1}),
      },
      offset = Transform.from_xyz({z=2});
    },
    {
      mesh = Mesh.sphere(1.0),
      material = {
        color = Color.rgba({blue=0.8, red=0.1}),
      },
      offset = Transform.from_xyz({z=3});
    }
  }
}

dance.chromes = {
  chrome_1,
};

dance.runner.running = true;
dance.runner.ms_per_tick = 10;
dance.runner.max_tick = 1000;
dance.runner.mode = "Repeat";

return dance;
