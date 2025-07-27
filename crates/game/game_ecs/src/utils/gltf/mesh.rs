use crate::GameResources;
use crate::components::MeshComponent;
use crate::utils::gltf::process_material;
use gltf::buffer::Data;
use gltf::mesh::iter::Primitives;
use sedona_renderer::types::StaticVertex;

pub fn process_meshes(
    g_meshes: Primitives,
    g_buffers: &[Data],
    g_images: &[gltf::image::Data],
    node: usize,
    resources: &mut GameResources,
) -> Vec<MeshComponent> {
    let mut meshes = Vec::new();

    for g_mesh in g_meshes {
        let reader = g_mesh.reader(|buffer| Some(&g_buffers[buffer.index()]));

        let positions: Vec<[f32; 4]> = match reader.read_positions() {
            Some(iter) => iter.map(|[x, y, z]| [x, y, z, 1.0]).collect(),
            None => continue,
        };

        let len = positions.len();

        let normals = reader
            .read_normals()
            .map(|iter| iter.map(|[x, y, z]| [x, y, z, 1.0]).collect())
            .unwrap_or_else(|| vec![[0.0; 4]; len]);

        let colors = reader
            .read_colors(0)
            .map(|c| c.into_rgba_f32().collect())
            .unwrap_or_else(|| vec![[1.0; 4]; len]);

        let tex_coords = reader
            .read_tex_coords(0)
            .map(|tc| tc.into_f32().collect())
            .unwrap_or_else(|| vec![[0.0, 0.0]; len]);

        let indices: Vec<u32> = reader
            .read_indices()
            .map(|i| i.into_u32().collect())
            .unwrap_or_else(|| (0..len as u32).collect());

        let tangents = reader
            .read_tangents()
            .map(|iter| iter.map(|[x, y, z, w]| [x, y, z, w]).collect())
            .unwrap_or_else(|| vec![[0.0; 4]; len]);

        let mut vertices = Vec::with_capacity(len);
        for ((((position, normal), tangent), color), tex_coord) in positions
            .into_iter()
            .zip(normals)
            .zip(tangents)
            .zip(colors)
            .zip(tex_coords)
        {
            vertices.push(StaticVertex {
                position,
                normal,
                tangent,
                color,
                tex_coord,
            });
        }

        let material = process_material(&g_mesh, g_images, resources);
        let mesh_handle = resources
            .renderer
            .create_render_object(&vertices, &indices, material, node)
            .unwrap();

        meshes.push(MeshComponent {
            handle: mesh_handle,
        });
    }

    meshes
}
