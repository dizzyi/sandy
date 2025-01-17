local sandy = {};

---Construct a new dance config
---@return Dance
function sandy.new_dance()
  ---@type Dance
  local dance = {
    runner = sandy.new_runner();
    chromes = {},
    on_start = function()
      Plotter.clear();
    end
  }
  return dance
end

function sandy.new_runner()
  ---@type Runner
  local runner = {
    mode = "Once",
    ms_per_tick = 10,
    max_tick = 1000,
    running = false,
  };
  return runner
end

return sandy;
