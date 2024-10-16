struct Image {
    data: array<f32>,
};

struct Neighbourhood {
    data: array<vec2<i32>>,  // Coordenadas relativas de los vecinos
};

struct Params {
    image_width: u32,
    image_height: u32,
    neighbour_count: u32,  // 8 vecinos en el Game of Life
};

@group(0) @binding(0)
var<storage, read_write> input: Image;

@group(0) @binding(1)
var<storage, read_write> result: Image;

@group(0) @binding(2)
var<storage, read_write> neighbors: Neighbourhood;

@group(0) @binding(3)
var<uniform> params: Params;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var width: u32 = params.image_width;
    var height: u32 = params.image_height;
    
    var x: u32 = global_id.x;
    var y: u32 = global_id.y;

    var index_0: u32 = y * width + x;
    var sum_neighbours: f32 = 0.0;

    for (var i: u32 = 0; i < params.neighbour_count; i = i + 1) {
        let dx: i32 = neighbors.data[i].x;
        let dy: i32 = neighbors.data[i].y;

        // Calcular la nueva posición del vecino
        let nx: i32 = i32(x) + dx;
        let ny: i32 = i32(y) + dy;

        // Verificación de límites correcta en ambas dimensiones
        if (nx >= 0 && ny >= 0 && u32(nx) < width && u32(ny) < height) {  
            let neighbor_index: u32 = u32(ny) * width + u32(nx);
            sum_neighbours = sum_neighbours + input.data[neighbor_index];
        }
    }

    var value: f32 = 0.0;
    if (sum_neighbours == 3.0) {
        value = 1.0;  // Nace una nueva celda
    } else if (sum_neighbours == 2.0) {
        value = input.data[index_0];  // Sobrevive
    } else {
        value = 0.0;  // Muere
    }

    result.data[index_0] = value;
}