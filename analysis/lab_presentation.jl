### A Pluto.jl notebook ###
# v0.19.14

using Markdown
using InteractiveUtils

# ╔═╡ c514365f-e5bf-4ca6-bc03-87e81bae9ec9
begin
	import Pkg
	Pkg.add(url="https://github.com/Fluid-Dynamics-Group/figure_second", subdir="julia/figure_second")
	using figure_second
end

# ╔═╡ aa609cf6-5f95-11ed-114a-55b3518a0639
using HDF5

# ╔═╡ fdac2300-4ff7-404c-8a82-3a66f355508e
using CairoMakie

# ╔═╡ 986c3d32-e2c9-4cd5-93ca-7afca0ed53b2
using Makie

# ╔═╡ a36bdc3e-a8dc-4e72-8a98-38bed9f0d067
using Reexport

# ╔═╡ 1732fda4-0854-4c6d-99c4-8fa6b276f17d
using JSON

# ╔═╡ 1ca72ff0-4bb0-4955-959f-7d4af0614e48
function ingredients(path::String)
	# this is from the Julia source code (evalfile in base/loading.jl)
	# but with the modification that it returns the module instead of the last object
	name = Symbol(basename(path))
	m = Module(name)
	Core.eval(m,
        Expr(:toplevel,
             :(eval(x) = $(Expr(:core, :eval))($name, x)),
             :(include(x) = $(Expr(:top, :include))($name, x)),
             :(include(mapexpr::Function, x) = $(Expr(:top, :include))(mapexpr, $name, x)),
             :(include($path))))
	m
end

# ╔═╡ 06675671-6be7-47b2-a39c-b777e217f80d
analysis = ingredients("./src/analysis.jl").analysis

# ╔═╡ 3d23c11e-ba4f-49ee-95b0-fd3ad9f58bc0
inkscape = updater("/home/brooks/github/fluids-research/presentations/2022.11.08/slide_2.svg")

# ╔═╡ 28c97d3e-2fe8-4dbc-a839-3103f751fb1b
loader = analysis.DataLoader("../output/distribute_save")

# ╔═╡ 21995ad2-fc68-4bc7-b601-d08056c7a5f0
span_averages = analysis.span_averages(loader, false)

# ╔═╡ c6388238-c982-4105-8d96-1f2e46c1f431
shear_stress = analysis.shear_stress(loader, false)

# ╔═╡ 791ab0ee-4466-450b-85a3-639ac9228fa7
span_times = analysis.spans_times(loader, false)

# ╔═╡ 5bafce88-9bf8-401f-adc8-860bbeca26d6
meta = analysis.metadata(loader)

# ╔═╡ 5662dfb6-bdbb-4d7d-a475-3f95c8f4ff60


# ╔═╡ 8d78ad2a-83b1-4887-a6a3-efcbe72cb6e5
begin
	local table = (
		x = [1,2, 3],
		height = [440.3, 74.52, 26.40]
	)

	local fig = Figure(resolution = (600, 600))
	local ax = Axis(fig[1,1], xticks = (1:3, ["Original speed", "Apply both optimized routines", "parallelize"]), title = "Converting 50 HDF5 flowfields to VTK files", ylabel = "time [s]")


	barplot!(
		ax,
		table.x,
		table.height
	)
		
	fig
end

# ╔═╡ dd7c9514-560c-459b-a96c-4091d1694315
begin
	local table = (
		x = [1,2],
		height = [163.39, 97.87]
	)

	local fig = Figure(resolution = (600, 600))
	local ax = Axis(fig[1,1], xticks = (1:2, ["Writing to temp vector, then file", "Write to buffered file"]), title = "File writing speeds (150 x 150 x 150)", ylabel = "time [ms]")


	barplot!(
		ax,
		table.x,
		table.height
	)
		
	fig
end

# ╔═╡ b2a31f33-4068-494a-a653-6480c761e01a
begin
	table = (
		x = [1,2, 3],
		height = [234.94, 93.89, 62.4]
	)

	fig = Figure(res = (500, 300))
	ax = Axis(fig[1,1], xticks = (1:3, ["original", "native indexing routines", "no bounds checks"]), title = "Array iterator benchmarking (150 x 150 x 150)", ylabel = "time [s]")


	barplot!(
		ax,
		table.x,
		table.height
	)
		
	fig
end

# ╔═╡ Cell order:
# ╠═aa609cf6-5f95-11ed-114a-55b3518a0639
# ╠═fdac2300-4ff7-404c-8a82-3a66f355508e
# ╠═986c3d32-e2c9-4cd5-93ca-7afca0ed53b2
# ╠═a36bdc3e-a8dc-4e72-8a98-38bed9f0d067
# ╠═1732fda4-0854-4c6d-99c4-8fa6b276f17d
# ╠═c514365f-e5bf-4ca6-bc03-87e81bae9ec9
# ╠═1ca72ff0-4bb0-4955-959f-7d4af0614e48
# ╠═06675671-6be7-47b2-a39c-b777e217f80d
# ╠═3d23c11e-ba4f-49ee-95b0-fd3ad9f58bc0
# ╠═28c97d3e-2fe8-4dbc-a839-3103f751fb1b
# ╠═21995ad2-fc68-4bc7-b601-d08056c7a5f0
# ╠═c6388238-c982-4105-8d96-1f2e46c1f431
# ╠═791ab0ee-4466-450b-85a3-639ac9228fa7
# ╠═5bafce88-9bf8-401f-adc8-860bbeca26d6
# ╠═5662dfb6-bdbb-4d7d-a475-3f95c8f4ff60
# ╠═8d78ad2a-83b1-4887-a6a3-efcbe72cb6e5
# ╠═dd7c9514-560c-459b-a96c-4091d1694315
# ╠═b2a31f33-4068-494a-a653-6480c761e01a
