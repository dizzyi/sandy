local sandy = require "lib/sandy";

Console.print("Hello Choom!");

Camera.space();

---@type Dance
local dance = sandy.new_dance();


---@type Chrome
local chrome_1 = {
  on_tick = function (tick)
    return Transform.from_xyz({
      z = tick * 0.001,
      y = 10.0 * math.sin(tick * 0.05),
      x = 10.0 * math.cos(tick * 0.05),
    })
  end,
  parts = {
    {
      mesh = Mesh.sphere(1.0),
      material = {
        color = Color.rgba({green=0.8, blue=0.1}),
      },
      offset = Transform.from_xyz({z=1});
    }
  }
}

dance.chromes = {
  chrome_1,
};

dance.runner.running = true;
dance.runner.ms_per_tick = 10;
dance.runner.max_tick = 10000;

return dance;
