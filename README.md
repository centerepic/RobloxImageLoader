# RobloxImageLoader
## A suite of tools required to convert .png images to a ViewportCanvas format.
### This project relies on a modified version https://github.com/boatbomber/ViewportCanvas for in-game rendering.

## Setup
1. Extract `image_encoder.rar`
2. Drag your image into the `input` folder. (Must be .png)
3. Run `png_to_rbxmx.exe`.
4. Import `Modules.rbxm` into Roblox Studio and ungroup it in ReplicatedStorage.
5. Drag the generated `.rbxmx` file in the output folder into your game and parent it to `ReplicatedStorage.Images`.

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
