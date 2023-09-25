# RobloxImageLoader
## A suite of tools required to convert .png images to a ViewportCanvas format.
### This project relies on a modified version https://github.com/boatbomber/ViewportCanvas for in-game rendering.

## Setup
1. Extract `image_encoder.rar` and install dependencies (PIL, json, os)
2. Drag your images into the `input` folder.
3. Run main.py.
4. Import `Modules.rbxm` into Roblox Studio and ungroup it in ReplicatedStorage.
5. Create a new ModuleScript in ReplicatedStorage.Images
6. Copy the text from the file created in the `output` folder into the ModuleScript so it looks like this -
```lua
  local module = {}
    module.Image = '(text from the output file)'
  return module
```

## Usage example
```lua
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local ImageLoader = require(ReplicatedStorage.ImageLoader)
local TargetImage = ReplicatedStorage.Images.["Image ModuleScript name here"]

local SurfaceGui = game.Players.LocalPlayer.PlayerGui.SurfaceGui

-- Guis have to be descendants of a PlayerGui to load properly.
-- This module should be used on the client.

ImageLoader:LoadImage(TargetImage, SurfaceGui)
```
