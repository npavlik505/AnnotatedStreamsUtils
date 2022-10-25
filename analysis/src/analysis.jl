module analysis
    include("./h5_helpers.jl")

    using Reexport
    @reexport using .H5Helpers: load_hdf5_vector_field, load_hdf5_scalar_field, load_hdf5_scalar_series, load_hdf5_2d_series

    export test

    function test()
        println("test results!")
    end

end # module analysis
