struct Image {
    data: array<f32>,
};

struct Neighbourhood {
    data: array<f32>,
};

struct Params {
    image_width: u32,
    kernel_size: u32,
    neighbour_dimension: u32,
};

@group(0) @binding(0)
var<storage, read_write> input: Image;

@group(0) @binding(1)
var<storage, read_write> result: Image;

@group(0) @binding(2)
var<storage, read_write> kernel: Neighbourhood;

@group(0) @binding(3)
var<uniform> params: Params;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var width: u32 = params.image_width;
    var height: u32 = arrayLength(&input.data) / width;

    var x: u32 = global_id.x;
    var y: u32 = global_id.y;

    var index_0: u32 = y * width + x;

    var total_neighbours: u32 = arrayLength(&kernel.data) / params.neighbour_dimension;
    var neighbours_data : array<f32, 128>;

    for (var i: u32 = 0; i < total_neighbours; i = i + 1) {
        var dx: i32 = i32(kernel.data[i*params.neighbour_dimension]);
        var dy: i32 = i32(kernel.data[(i+1)*params.neighbour_dimension]);

        var xx: i32 = i32(x);
        var yy: i32 = i32(y);

        var index_i = u32(yy+dy) * width + u32(xx+dx);
        
        neighbours_data[i] = input.data[index_i];

        // var index_i = (y+dy)* width + (x+dx);
        // var n_data = input.data[index_i];
    }

    

    // var index_0: u32 = y * width + x;

    // var index_1: u32 = (y-1) * width + x;
    // var index_2: u32 = (y+1) * width + x;
    // var index_3: u32 = y * width + (x-1);
    // var index_4: u32 = y * width + (x+1);
    // var index_5: u32 = (y-1) * width + (x-1);
    // var index_6: u32 = (y-1) * width + (x+1);
    // var index_7: u32 = (y+1) * width + (x-1);
    // var index_8: u32 = (y+1) * width + (x+1);

    var sum_neighbours: f32 = 0.0;

    for (var i: u32 = 0; i < total_neighbours; i = i + 1) {
        sum_neighbours = sum_neighbours + neighbours_data[i];
    }

    
    var value: f32 = 0.0;

    // let n = input.data[index_1] + 
    //         input.data[index_2] + 
    //         input.data[index_3] + 
    //         input.data[index_4] + 
    //         input.data[index_5] + 
    //         input.data[index_6] + 
    //         input.data[index_7] + 
    //         input.data[index_8];

    if (sum_neighbours == 3.0) {
        value = 1.0;
    } else if (sum_neighbours == 2.0) {
        value = input.data[index_0];
    } else {
        value = 0.0;
    }

    result.data[index_0] = value;
}
