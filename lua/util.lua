local util = {}

---Construct a new dance config
---@return Dance
function util.new_dance()
	---@type Dance
	local dance = {
		runner = util.new_runner(),
		chromes = {},
		on_start = function()
			Plotter.clear()
		end,
		on_tick = function() end,
	}
	return dance
end

function util.new_runner()
	---@type Runner
	local runner = {
		mode = "Repeat",
		ms_per_tick = 10,
		max_tick = 1000,
		running = true,
	}
	return runner
end

---make a red ball
---@return ChromePart
function util.red_ball()
	---@type ChromePart
	local part = {
		mesh = Mesh.sphere(1.0),
		material = {
			color = Color.rgba({ red = 1.0, alpha = 1.0 }),
		},
	}
	return part
end
---make a blue cube
---@return ChromePart
function util.blue_cube()
	return {
		mesh = Mesh.cuboid(1.0),
		material = {
			color = Color.rgba({ blue = 1.0, alpha = 1.0 }),
		},
	}
end

function util.make_after_image(max_tick, interval)
  local after_image = function (tick)
    if tick % interval ~= 0 then
      return nil
    end
    ---@type Material
    local material = {
      color = Color.hsva({
        alpha = 0.5,
        hue = tick / max_tick * 360.0,
        value = 0.8,
        saturation = 0.8,
      }),
    }
    return material
  end
  return after_image
end

return util
