local ok, api = pcall(require, "api")
local workspace = {}
workspace.path = vim.fn.getcwd(0)

vim.opt.rtp:prepend("./.nvim/")

if ok == true then
	api.snip.add_snip_dir(workspace.path .. "/.nvim/snip/")
else 
	vim.notify("Not have \"api\" package.\nThis .nvim.lua file use lI15SO0's nvim config.\nYou can delete .nvim.lua file.", vim.log.levels.WARN, {title = "LeetCode exrc"})
end

require("ws.core.keymaps")
require("ws.core.options")
