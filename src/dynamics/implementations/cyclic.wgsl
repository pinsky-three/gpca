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
    states: u32,
    threshold: u32,
};

@group(0) @binding(3)
var<uniform> params: Params;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var width: u32 = params.image_width;
    var size: u32 = params.kernel_size;

    var states: u32 = params.states;
    var threshold: u32 = params.threshold;

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

    var current_state: u32 = u32(input.data[index_0]);

    var next_state: f32 = f32((current_state + 1u) % states);

    var n: u32 = u32(input.data[index_1] == next_state) +
            u32(input.data[index_2] == next_state) +
            u32(input.data[index_3] == next_state) +
            u32(input.data[index_4] == next_state) +
            u32(input.data[index_5] == next_state) +
            u32(input.data[index_6] == next_state) +
            u32(input.data[index_7] == next_state) +
            u32(input.data[index_8] == next_state);

    var value: f32 = f32(current_state);

    if (n >= threshold) {
        value = next_state;
    }

    result.data[index_0] = value;
}
