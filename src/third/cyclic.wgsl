struct Image {
    data: array<f32>,
};

@group(0) @binding(0)
var<storage, read_write> input: Image;

@group(0) @binding(1)
var<storage, read_write> result: Image;

@group(0) @binding(2)
var<storage, read_write> kernel: Image;

struct Params {
    image_width: u32,
    kernel_size: u32,
};

@group(0) @binding(3)
var<uniform> params: Params;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var width: u32 = params.image_width;
    var size: u32 = params.kernel_size;

    var states: u32 = 6;
    var threshold: u32 = 1;

    var x: u32 = global_id.x;
    var y: u32 = global_id.y;

    var index_0: u32 = y * width + x;

    var index_1: u32 = (y-1u) * width + x;
    var index_2: u32 = (y+1u) * width + x;
    var index_3: u32 = y * width + (x-1u);
    var index_4: u32 = y * width + (x+1u);
    var index_5: u32 = (y-1u) * width + (x-1u);
    var index_6: u32 = (y-1u) * width + (x+1u);
    var index_7: u32 = (y+1u) * width + (x-1u);
    var index_8: u32 = (y+1u) * width + (x+1u);

    var i: u32 = u32(input.data[index_0]);

    var i_1: f32 = f32((i + 1u) % states);

    var n: u32 = u32(input.data[index_1] == i_1) +
            u32(input.data[index_2] == i_1) +
            u32(input.data[index_3] == i_1) +
            u32(input.data[index_4] == i_1) +
            u32(input.data[index_5] == i_1) +
            u32(input.data[index_6] == i_1) +
            u32(input.data[index_7] == i_1) +
            u32(input.data[index_8] == i_1);

    var value: f32 = f32(i);

    if (n > threshold) {
        value = i_1;
    }

    result.data[index_0] = value;
}
