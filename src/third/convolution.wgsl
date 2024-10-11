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

    var value: f32 = 0.0;
    var i: u32 = 0u;

    loop {
        if (i >= size) {
            break;
        }

        var j: u32 = 0u;

        loop {
            if (j >= size) {
                break;
            }

            var k: f32 = kernel.data[j * size + i];
            var x: u32 = global_id.x + i;
            var y: u32 = global_id.y + j;
            value = value + input.data[y * width + x] * k;

            continuing {
                j = j + 1u;
            }
        }

        continuing {
            i = i + 1u;
        }
    }

    var crop: u32 = size - 1u;
    var index: u32 = global_id.y * (width - crop) + global_id.x;

    var val: f32 = 0.0;

    if (value == 2.0) {
        var x: u32 = global_id.x;
        var y: u32 = global_id.y;

        val = input.data[y * width + x];
    } else if (value == 3.0) {
        val = 1.0;
    } else {
        val = 0.0;
    }

    result.data[index] = val;
}
