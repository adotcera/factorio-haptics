local IGNORE = {}
IGNORE["space-science-pack"] = true
local TARGET = 150

script.on_init(function ()
    global.science_packs = {}

    local prototypes = game.get_filtered_item_prototypes{{filter="subgroup", subgroup="science-pack"}, {filter="flag", flag="hidden", invert=true, mode="and"}}
    
    for _, pack in pairs(prototypes) do
        if not IGNORE[pack.name] then
            table.insert(global.science_packs, pack.name)
        end
    end
end)

local function clamp(x, min, max)
    if x > max then return max end
    if x < min then return min end
    return x
end

script.on_nth_tick(60 * 5, function ()
    local buffer = {}

    for _, packname in ipairs(global.science_packs) do
        -- get current flow
        local stats = game.forces.player.item_production_statistics
        local consumed = stats.get_flow_count{name=packname, input=false, precision_index=defines.flow_precision_index.one_minute, count=true}
        -- game.print(packname .. ": " .. consumed)

        -- set intensity
        local intensity = clamp(consumed / TARGET, 0, 1)
        table.insert(buffer, tostring(math.ceil(intensity * 100)))
    end

    local contents = table.concat(buffer, ",")
    game.write_file("haptics-current", contents)
    -- game.print("Wrote " .. contents)
end)
