local ok, dap = pcall(require, 'dap')

local adapters = {
	-- filetype = {...}
}
local configurations = {
	-- filetype = {...}
}

if ok == true then
	for i, j in pairs(adapters) do
		dap.adapters[i] = j
	end
	
	for i, j in pairs(configurations) do
		dap.configurations[i] = j
	end

else 

	vim.notify("Cannot found dap plugins.", vim.log.levels.WARN, {title = "Workspace DAP Configurer." })
end
