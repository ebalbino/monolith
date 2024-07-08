use crate::math::*;
use crate::arena::{Arena, ArenaView};

#[derive(Debug, Clone, PartialEq)]
pub struct VertexData {
    positions: ArenaView<Vec3>,
    normals: ArenaView<Vec3>,
    texcoords: ArenaView<Vec2>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Point,
    Line,
    Triangle,
    Quad,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Point(ArenaView<u32>),
    Line(ArenaView<Vec2u>),
    Triangle(ArenaView<Vec3u>),
    Quad(ArenaView<Vec4u>),
}

pub struct Mesh {
    elements: Element,
    vertices: VertexData,
}

impl VertexData {
    pub fn positions(&self) -> &[Vec3] {
        self.positions.as_ref()
    }

    pub fn normals(&self) -> &[Vec3] {
        self.normals.as_ref()
    }

    pub fn texcoords(&self) -> &[Vec2] {
        self.texcoords.as_ref()
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_valid(&self) -> bool {
        let len = self.len();

        if self.normals.len() != len {
            return false;
        }

        if self.texcoords.len() != len {
            return false;
        }

        true
    }
}

impl Mesh {
    pub fn new(vertices: VertexData, elements: Element) -> Mesh {
        Mesh {
            vertices,
            elements,
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        self.vertices.is_valid()
    }

    pub fn vertices(&self) -> &VertexData {
        &self.vertices
    }

    pub fn positions(&self) -> &[Vec3] {
        self.vertices.positions()
    }

    pub fn normals(&self) -> &[Vec3] {
        self.vertices.normals()
    }

    pub fn texcoords(&self) -> &[Vec2] {
        self.vertices.texcoords()
    }

    pub fn elements(&self) -> &Element {
        &self.elements
    }
}

pub fn merge_meshes(arena: &Arena, meshes: &[Mesh], element_type: ElementType) -> Option<Mesh> {
    if meshes.is_empty() {
        return None;
    }

    if meshes.iter().any(|m| match element_type {
        ElementType::Point => match m.elements() {
            Element::Point(_) => false,
            _ => true,
        },
        ElementType::Line => match m.elements() {
            Element::Line(_) => false,
            _ => true,
        },
        ElementType::Triangle => match m.elements() {
            Element::Triangle(_) => false,
            _ => true,
        },
        ElementType::Quad => match m.elements() {
            Element::Quad(_) => false,
            _ => true,
        },
    }) {
        return None;
    }

    let element_count = meshes.iter().map(|m| {
        match element_type {
            ElementType::Point => match m.elements() {
                Element::Point(indices) => indices.len(),
                _ => 0,
            },
            ElementType::Line => match m.elements() {
                Element::Line(indices) => indices.len(),
                _ => 0,
            },
            ElementType::Triangle => match m.elements() {
                Element::Triangle(indices) => indices.len(),
                _ => 0,
            },
            ElementType::Quad => match m.elements() {
                Element::Quad(indices) => indices.len(),
                _ => 0,
            },
        }
    }).sum();

    let vertex_count = meshes.iter().map(|m| m.len()).sum();

    if element_count == 0 || vertex_count == 0 {
        return None;
    }

    let mut elements = match element_type {
        ElementType::Point => Element::Point(arena.allocate::<u32>(element_count)?),
        ElementType::Line => Element::Line(arena.allocate::<Vec2u>(element_count)?),
        ElementType::Triangle => Element::Triangle(arena.allocate::<Vec3u>(element_count)?),
        ElementType::Quad => Element::Quad(arena.allocate::<Vec4u>(element_count)?),
    };

    let mut positions = arena.allocate::<Vec3>(vertex_count)?;
    let mut normals = arena.allocate::<Vec3>(vertex_count)?;
    let mut texcoords = arena.allocate::<Vec2>(vertex_count)?;

    let mut index_offset = 0;
    let mut vertex_offset = 0;

    for mesh in meshes {
        match element_type {
            ElementType::Point => match mesh.elements() {
                Element::Point(mesh_indices) => {
                    let element_indices = match &mut elements {
                        Element::Point(indices) => indices,
                        _ => return None,
                    };

                    element_indices[index_offset..(index_offset + mesh_indices.len())].copy_from_slice(&mesh_indices[..]);

                    if index_offset > 0 {
                        for index in element_indices.iter_mut() {
                            *index += index_offset as u32;
                        }
                    }

                    index_offset += mesh_indices.len();
                },
                _ => return None,
            },
            ElementType::Line => match mesh.elements() {
                Element::Line(mesh_indices) => {
                    let element_indices = match &mut elements {
                        Element::Line(indices) => indices,
                        _ => return None,
                    };

                    for i in 0..mesh_indices.len() {
                        element_indices[index_offset + i] = Vec2u::new(
                            mesh_indices[i].x + vertex_offset as u32,
                            mesh_indices[i].y + vertex_offset as u32,
                        );
                    }

                    if index_offset > 0 {
                        for index in element_indices.iter_mut() {
                            index.x += vertex_offset as u32;
                            index.y += vertex_offset as u32;
                        }
                    }

                    index_offset += mesh_indices.len();
                },
                _ => return None,
            },
            ElementType::Triangle => match mesh.elements() {
                Element::Triangle(mesh_indices) => {
                    let element_indices = match &mut elements {
                        Element::Triangle(indices) => indices,
                        _ => return None,
                    };

                    for i in 0..mesh_indices.len() {
                        element_indices[index_offset + i] = Vec3u::new(
                            mesh_indices[i].x + vertex_offset as u32,
                            mesh_indices[i].y + vertex_offset as u32,
                            mesh_indices[i].z + vertex_offset as u32,
                        );
                    }

                    if index_offset > 0 {
                        for index in element_indices.iter_mut() {
                            index.x += vertex_offset as u32;
                            index.y += vertex_offset as u32;
                            index.z += vertex_offset as u32;
                        }
                    }

                    index_offset += mesh_indices.len();
                },
                _ => return None,
            },
            ElementType::Quad => match mesh.elements() {
                Element::Quad(mesh_indices) => {
                    let element_indices = match &mut elements {
                        Element::Quad(indices) => indices,
                        _ => return None,
                    };

                    for i in 0..mesh_indices.len() {
                        element_indices[index_offset + i] = Vec4u::new(
                            mesh_indices[i].x + vertex_offset as u32,
                            mesh_indices[i].y + vertex_offset as u32,
                            mesh_indices[i].z + vertex_offset as u32,
                            mesh_indices[i].w + vertex_offset as u32,
                        );
                    }

                    if index_offset > 0 {
                        for index in element_indices.iter_mut() {
                            index.x += vertex_offset as u32;
                            index.y += vertex_offset as u32;
                            index.z += vertex_offset as u32;
                            index.w += vertex_offset as u32;
                        }
                    }

                    index_offset += mesh_indices.len();
                },
                _ => return None,
            },
        }

        positions[vertex_offset..(vertex_offset + mesh.len())].copy_from_slice(&mesh.positions()[..]);
        normals[vertex_offset..(vertex_offset + mesh.len())].copy_from_slice(&mesh.normals()[..]);
        texcoords[vertex_offset..(vertex_offset + mesh.len())].copy_from_slice(&mesh.texcoords()[..]);

        vertex_offset += mesh.len();
    }

    Some(Mesh {
        elements,
        vertices: VertexData {
            positions,
            normals,
            texcoords,
        },
    })
}

pub fn make_quads(arena: &Arena, steps: Vec2u, scale: Vec2, uvscale: Vec2) -> Option<Mesh> {
    let mut positions = arena.allocate::<Vec3>(((steps.x + 1) * (steps.y + 1)) as usize)?;
    let mut normals = arena.allocate::<Vec3>(((steps.x + 1) * (steps.y + 1)) as usize)?;
    let mut texcoords = arena.allocate::<Vec2>(((steps.x + 1) * (steps.y + 1)) as usize)?;
    let mut quads = arena.allocate::<Vec4u>((steps.x * steps.y) as usize)?;

    for y in 0..(steps.y + 1) {
        for x in 0..(steps.x + 1) {
            let uv = vec2(x as f32 / steps.x as f32, y as f32 / steps.y as f32);
            let index = (y * (steps.x + 1) + x) as usize;

            positions[index] = vec3(
                ((2.0 * uv.x) - 1.0) * scale.x,
                ((2.0 * uv.y) - 1.0) * scale.y,
                0.0,
            );
            normals[index] = vec3(0.0, 0.0, 1.0);
            texcoords[index] = vec2(uv.x, 1.0 - uv.y) * uvscale;
        }
    }

    for y in 0..steps.y {
        for x in 0..steps.x {
            let index = (y * steps.x + x) as usize;
            quads[index] = Vec4u::new(
                y * (steps.x + 1) + x,
                y * (steps.x + 1) + (x + 1),
                (y + 1) * (steps.x + 1) + (x + 1),
                (y + 1) * (steps.x + 1) + x
            );
        }
    }

    return Some(Mesh {
        vertices: VertexData {
            positions,
            normals,
            texcoords,
        },
        elements: Element::Quad(quads),
    });
}

pub fn make_rect(arena: &Arena, steps: Vec2u, scale: Vec2, uvscale: Vec2) -> Option<Mesh> {
    return make_quads(arena, steps, scale, uvscale);
}

pub fn make_bulged_rect(arena: &Arena, steps: Vec2u, scale: Vec2, uvscale: Vec2, height: f32) ->Option<Mesh> {
    let mut rect = make_rect(arena, steps, scale, uvscale)?;

    if height != 0.0 {
        let height = height.min(scale.min_element());
        let radius = (1.0 + height * height) / (2.0 * height);
        let center = Vec3::new(0.0, 0.0, -radius + height);
        for (position, normal) in rect.vertices.positions.iter_mut().zip(rect.vertices.normals.iter_mut()) {
            let pn = (*position - center).normalize();
            *position = center + pn * radius;
            *normal = pn;
        }
    }

    return Some(rect);
}

pub fn make_recty(arena: &Arena, steps: Vec2u, scale: Vec2, uvscale: Vec2) -> Option<Mesh> {
    let mut rect = make_rect(arena, steps, scale, uvscale)?;

    for position in rect.vertices.positions.iter_mut() {
        *position = Vec3::new(position.x, position.z, -position.y);
    }

    for normal in rect.vertices.normals.iter_mut() {
        *normal = Vec3::new(normal.x, normal.z, normal.y);
    }

    return Some(rect);
}

pub fn make_bulged_recty(arena: &Arena, steps: Vec2u, scale: Vec2, uvscale: Vec2, height: f32) -> Option<Mesh> {
    let mut rect = make_bulged_rect(arena, steps, scale, uvscale, height)?;

    for position in rect.vertices.positions.iter_mut() {
        *position = Vec3::new(position.x, position.z, -position.y);
    }

    for normal in rect.vertices.normals.iter_mut() {
        *normal = Vec3::new(normal.x, normal.z, normal.y);
    }

    return Some(rect);
}

pub fn make_box(arena: &Arena, steps: Vec3u, scale: Vec3, uvscale: Vec3) -> Option<Mesh> {
    let mut z_plus = make_rect(arena, Vec2u::new(steps.x, steps.y), Vec2::new(scale.x, scale.y), Vec2::new(uvscale.x, uvscale.y))?;
    let mut z_minus = make_rect(arena, Vec2u::new(steps.x, steps.y), Vec2::new(scale.x, scale.y), Vec2::new(uvscale.x, uvscale.y))?;
    let mut x_plus = make_recty(arena, Vec2u::new(steps.z, steps.y), Vec2::new(scale.z, scale.y), Vec2::new(uvscale.z, uvscale.y))?;
    let mut x_minus = make_recty(arena, Vec2u::new(steps.z, steps.y), Vec2::new(scale.z, scale.y), Vec2::new(uvscale.z, uvscale.y))?;
    let mut y_plus = make_rect(arena, Vec2u::new(steps.x, steps.z), Vec2::new(scale.x, scale.z), Vec2::new(uvscale.x, uvscale.z))?;
    let mut y_minus = make_rect(arena, Vec2u::new(steps.x, steps.z), Vec2::new(scale.x, scale.z), Vec2::new(uvscale.x, uvscale.z))?;

    for (position, normal) in z_plus.vertices.positions.iter_mut().zip(z_plus.vertices.normals.iter_mut()) {
        *position = Vec3::new(position.x, position.y, scale.z);
        *normal = Vec3::new(0.0, 0.0, 1.0);
    }

    for (position, normal) in z_minus.vertices.positions.iter_mut().zip(z_minus.vertices.normals.iter_mut()) {
        *position = Vec3::new(position.x, position.y, -scale.z);
        *normal = Vec3::new(0.0, 0.0, -1.0);
    }

    for (position, normal) in x_plus.vertices.positions.iter_mut().zip(x_plus.vertices.normals.iter_mut()) {
        *position = Vec3::new(scale.x, position.y, position.z);
        *normal = Vec3::new(1.0, 0.0, 0.0);
    }

    for (position, normal) in x_minus.vertices.positions.iter_mut().zip(x_minus.vertices.normals.iter_mut()) {
        *position = Vec3::new(-scale.x, position.y, position.z);
        *normal = Vec3::new(-1.0, 0.0, 0.0);
    }

    for (position, normal) in y_plus.vertices.positions.iter_mut().zip(y_plus.vertices.normals.iter_mut()) {
        *position = Vec3::new(position.x, scale.y, position.z);
        *normal = Vec3::new(0.0, 1.0, 0.0);
    }

    for (position, normal) in y_minus.vertices.positions.iter_mut().zip(y_minus.vertices.normals.iter_mut()) {
        *position = Vec3::new(position.x, -scale.y, position.z);
        *normal = Vec3::new(0.0, -1.0, 0.0);
    }

    let faces = [z_plus, z_minus, x_plus, x_minus, y_plus, y_minus];
    return merge_meshes(arena, &faces, ElementType::Quad);
}

pub fn make_rounded_box(arena: &Arena, steps: Vec3u, scale: Vec3, uvscale: Vec3, radius: f32) -> Option<Mesh> {
    let mut box_mesh = make_box(arena, steps, scale, uvscale)?;

    if radius != 0.0 {
        let radius = radius.min(scale.min_element());
        let c = scale - radius;

        for (position, normal) in box_mesh.vertices.positions.iter_mut().zip(box_mesh.vertices.normals.iter_mut()) {
            let pc = Vec3::new(position.x.abs(), position.y.abs(), position.z.abs());
            let ps = Vec3::new(position.x.signum(), position.y.signum(), position.z.signum());

            if pc.x >= c.x && pc.y >= c.y && pc.z >= c.z {
                let pn = (pc - c).normalize();
                *position = c + radius * pn;
                *normal = pn;
            } else if pc.x >= c.x && pc.y >= c.y {
                let pn = ((pc - c) * Vec3::new(1.0, 1.0, 0.0)).normalize();
                *position = Vec3::new(c.x + radius * pn.x, c.y + radius * pn.y, pc.z);
                *normal = pn;
            } else if pc.x >= c.x && pc.z >= c.z {
                let pn = ((pc - c) * Vec3::new(1.0, 0.0, 1.0)).normalize();
                *position = Vec3::new(c.x + radius * pn.x, pc.y, c.z + radius * pn.z);
                *normal = pn;
            } else if pc.y >= c.y && pc.z >= c.z {
                let pn = ((pc - c) * Vec3::new(0.0, 1.0, 1.0)).normalize();
                *position = Vec3::new(pc.x, c.y + radius * pn.y, c.z + radius * pn.z);
                *normal = pn;
            } else {
                continue;
            }

            *position *= ps;
            *normal *= ps;
        }
    }

    return Some(box_mesh);
}

pub fn make_rect_stack(arena: &Arena, steps: Vec3u, scale: Vec3, uvscale: Vec2) -> Option<Mesh> {
    let mut meshes = arena.allocate::<Mesh>((steps.z + 1) as usize)?;

    for i in 0..(steps.z + 1) {
        let mut mesh = make_rect(arena, Vec2u::new(steps.x, steps.y), Vec2::new(scale.x, scale.y), uvscale)?;

        for position in mesh.vertices.positions.iter_mut() {
            position.z = (-1.0 + 2.0 * i as f32 / steps.z as f32) * scale.z;
        }

        meshes[i as usize] = mesh;
    }

    return merge_meshes(arena, &meshes, ElementType::Quad);
}

pub fn make_floor(arena: &Arena, steps: Vec2u, scale: Vec2, uvscale: Vec2) -> Option<Mesh> {
    let mut mesh = make_rect(arena, steps, scale, uvscale)?;

    for position in mesh.vertices.positions.iter_mut() {
        *position = Vec3::new(position.x, position.z, -position.y);
    }

    for normal in mesh.vertices.normals.iter_mut() {
        *normal = Vec3::new(normal.x, normal.z, normal.y);
    }

    return Some(mesh);
}

pub fn make_sphere(arena: &Arena, steps: u32, scale: f32, uvscale: f32) -> Option<Mesh> {
    let mut mesh = make_box(arena, Vec3u::new(steps, steps, steps), Vec3::new(scale, scale, scale), Vec3::new(uvscale, uvscale, uvscale))?;

    for (position, normal) in mesh.vertices.positions.iter_mut().zip(mesh.vertices.normals.iter_mut()) {
        let pn = position.normalize();
        *position = scale * pn;
        *normal = pn;
    }

    return Some(mesh);
}

pub fn make_uv_sphere(arena: &Arena, steps: Vec2u, scale: f32, uvscale: Vec2) -> Option<Mesh> {
    let mut mesh = make_rect(arena, steps, Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0))?;

    for i in 0..mesh.vertices.positions.len() {
        let uv = mesh.vertices.texcoords[i];
        let a = Vec2::new(2.0 * core::f32::consts::PI * uv.x, core::f32::consts::PI * (1.0 - uv.y));

        mesh.vertices.positions[i] = Vec3::new(
            a.x.cos() * a.y.sin(),
            a.x.sin() * a.y.sin(),
            a.y.cos(),
        ) * scale;

        mesh.vertices.normals[i] = mesh.vertices.positions[i].normalize();
        mesh.vertices.texcoords[i] = uv * uvscale;
    }

    return Some(mesh);
}

pub fn make_uv_sphere_y(arena: &Arena, steps: Vec2u, scale: f32, uvscale: Vec2) -> Option<Mesh> {
    let mut mesh = make_uv_sphere(arena, steps, scale, uvscale)?;

    for position in mesh.vertices.positions.iter_mut() {
        *position = Vec3::new(position.x, position.z, position.y);
    }

    for normal in mesh.vertices.normals.iter_mut() {
        *normal = Vec3::new(normal.x, normal.z, normal.y);
    }

    for texcoords in mesh.vertices.texcoords.iter_mut() {
        *texcoords = Vec2::new(texcoords.x, 1.0 - texcoords.y);
    }

    let elements = match mesh.elements {
        Element::Quad(ref mut quads) => quads,
        _ => return None,
    };

    for quad in elements.iter_mut() {
        *quad = Vec4u::new(quad.x, quad.w, quad.z, quad.y);
    }

    return Some(mesh);
}

pub fn make_capped_uvsphere(arena: &Arena, steps: Vec2u, scale: f32, uvscale: Vec2, cap: f32) -> Option<Mesh> {
    let mut mesh = make_uv_sphere(arena, steps, scale, uvscale)?;

    if cap != 0.0 {
        let cap = cap.min(scale / 2.0);
        let zflip = scale - cap;
        for (position, normal) in mesh.vertices.positions.iter_mut().zip(mesh.vertices.normals.iter_mut()) {
            if position.z > zflip {
                position.z = 2.0 * zflip - position.z;
                normal.x = -normal.x;
                normal.y = -normal.y;
            } else if position.z < -zflip {
                position.z = 2.0 * -zflip - position.z;
                normal.x = -normal.x;
                normal.y = -normal.y;
            }
        }
    }

    return Some(mesh);
}

pub fn make_capped_uvsphere_y(arena: &Arena, steps: Vec2u, scale: f32, uvscale: Vec2, cap: f32) -> Option<Mesh> {
    let mut mesh = make_uv_sphere_y(arena, steps, scale, uvscale)?;

    if cap != 0.0 {
        let cap = cap.min(scale / 2.0);
        let zflip = scale - cap;
        for (position, normal) in mesh.vertices.positions.iter_mut().zip(mesh.vertices.normals.iter_mut()) {
            if position.z > zflip {
                position.z = 2.0 * zflip - position.z;
                normal.x = -normal.x;
                normal.y = -normal.y;
            } else if position.z < -zflip {
                position.z = 2.0 * -zflip - position.z;
                normal.x = -normal.x;
                normal.y = -normal.y;
            }
        }
    }

    return Some(mesh);
}

pub fn make_disk(arena: &Arena, steps: u32, scale: f32, uvscale: f32) -> Option<Mesh> {
    let mut mesh = make_rect(arena, Vec2u::new(steps, steps), Vec2::new(1.0, 1.0), Vec2::new(uvscale, uvscale))?;

    for position in mesh.vertices.positions.iter_mut() {
        *position = Vec3::new(position.x, position.z, position.y);
    }

    for normal in mesh.vertices.normals.iter_mut() {
        *normal = Vec3::new(normal.x, normal.z, normal.y);
    }

    for texcoords in mesh.vertices.texcoords.iter_mut() {
        *texcoords = Vec2::new(texcoords.x, 1.0 - texcoords.y);
    }

    let elements = match mesh.elements {
        Element::Quad(ref mut quads) => quads,
        _ => return None,
    };

    for quad in elements.iter_mut() {
        *quad = Vec4u::new(quad.x, quad.w, quad.z, quad.y);
    }

    return Some(mesh);
}

pub fn make_bulged_disk(arena: &Arena, steps: u32, scale: f32, uvscale: f32, height: f32) -> Option<Mesh> {
    let mut disk = make_disk(arena, steps, scale, uvscale)?;

    if height != 0.0 {
        let height = height.min(scale);
        let radius = (1.0 + height * height) / (2.0 * height);
        let center = Vec3::new(0.0, 0.0, -radius + height);
        for (position, normal) in disk.vertices.positions.iter_mut().zip(disk.vertices.normals.iter_mut()) {
            let pn = (*position - center).normalize();
            *position = center + pn * radius;
            *normal = pn;
        }
    }

    return Some(disk);
}

pub fn make_uv_disk(arena: &Arena, steps: Vec2u, scale: f32, uvscale: Vec2) -> Option<Mesh> {
    let mut disk = make_rect(arena, steps, Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0))?;

    for i in 0..disk.vertices.positions.len() {
        let uv = disk.vertices.texcoords[i];
        let phi = 2.0 * core::f32::consts::PI * uv.x;

        disk.vertices.positions[i] = Vec3::new(phi.cos() * uv.y, phi.sin() * uv.y, 0.0) * scale;
        disk.vertices.normals[i] = Vec3::new(0.0, 0.0, 1.0);
        disk.vertices.texcoords[i] = uv * uvscale;
    }

    return Some(disk);
}

pub fn make_lines(arena: &Arena, steps: Vec2u, scale: Vec2, uvscale: Vec2, rad: Vec2) -> Option<Mesh> {
    let mut positions = arena.allocate::<Vec3>(((steps.x + 1) * steps.y) as usize).unwrap();
    let mut normals = arena.allocate::<Vec3>(((steps.x + 1) * steps.y) as usize).unwrap();
    let mut texcoords = arena.allocate::<Vec2>(((steps.x + 1) * steps.y) as usize).unwrap();
    let mut radius = arena.allocate::<f32>(((steps.x + 1) * steps.y) as usize).unwrap();
    let mut lines = arena.allocate::<Vec2u>((steps.x * steps.y) as usize).unwrap();

    if steps.y > 1 {
        for y in 0..steps.y {
            for x in 0..steps.x + 1 {
                let uv = vec2(x as f32 / steps.x as f32, y as f32 / (steps.y - 1) as f32);
                let index = (y * (steps.x + 1) + x) as usize;

                positions[index] = Vec3::new(
                    (uv.x - 0.5) * scale.x,
                    (uv.y - 0.5) * scale.y,
                    0.0
                );
                normals[index] = Vec3::new(0.0, 1.0, 0.0);
                texcoords[index] = Vec2::new(uv.x, 1.0 - uv.y) * uvscale;
                radius[index] = lerp(rad.x, rad.y, uv.y);
            }
        }
    } else {
        for x in 0..steps.x + 1 {
            let uv = vec2(x as f32 / steps.x as f32, 0.0);
            let index = x as usize;

            positions[index] = Vec3::new(
                (uv.x - 0.5) * scale.x,
                0.0,
                0.0
            );
            normals[index] = Vec3::new(1.0, 0.0, 0.0);
            texcoords[index] = uv * uvscale;
            radius[index] = lerp(rad.x, rad.y, uv.x);
        }
    }

    for y in 0..steps.y {
        for x in 0..steps.x {
            let index = (y * steps.x + x) as usize;
            lines[index] = Vec2u::new(
                y * (steps.x + 1) + x,
                y * (steps.x + 1) + (x + 1),
            );
        }
    }

    Some(Mesh {
        vertices: VertexData {
            positions,
            normals,
            texcoords,
        },
        elements: Element::Line(lines),
    })
}

pub fn quads_to_triangles(arena: &Arena, quads: &ArenaView<Vec4u>) -> Option<ArenaView<Vec3u>> {
    let mut triangles = arena.allocate::<Vec3u>(quads.len() * 2)?;
    let mut triangle_count = 0;

    for quad in quads.iter() {
        triangles[triangle_count] = Vec3u::new(quad.x, quad.y, quad.w);
        triangle_count += 1;

        if quad.z != quad.w {
            triangles[triangle_count] = Vec3u::new(quad.z, quad.w, quad.y);
            triangle_count += 1;
        }
    }

    return Some(triangles);
}

pub fn triangles_to_quads(arena: &Arena, triangles: ArenaView<Vec3u>) -> Option<ArenaView<Vec4u>> {
    let mut quads = arena.allocate::<Vec4u>(triangles.len())?;
    let mut quad_count = 0;

    for triangle in triangles.iter() {
        quads[quad_count] = Vec4u::new(triangle.x, triangle.y, triangle.z, triangle.z);
        quad_count += 1;
    }

    return Some(quads);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_data() {
        let arena = Arena::new(1024);

        let positions = arena.allocate::<Vec3>(10).unwrap();
        let normals = arena.allocate::<Vec3>(10).unwrap();
        let texcoords = arena.allocate::<Vec2>(10).unwrap();

        let vertices = VertexData {
            positions,
            normals,
            texcoords,
        };

        assert_eq!(vertices.len(), 10);
        assert_eq!(vertices.is_empty(), false);
        assert_eq!(vertices.is_valid(), true);

        assert_eq!(vertices.positions().len(), 10);
        assert_eq!(vertices.normals().len(), 10);
        assert_eq!(vertices.texcoords().len(), 10);
    }

    #[test]
    fn test_mesh() {
        let arena = Arena::new(1024);

        let positions = arena.allocate::<Vec3>(10).unwrap();
        let normals = arena.allocate::<Vec3>(10).unwrap();
        let texcoords = arena.allocate::<Vec2>(10).unwrap();
        let indices = arena.allocate::<u32>(10).unwrap();

        let vertices = VertexData {
            positions,
            normals,
            texcoords,
        };

        let mesh = Mesh::new(vertices, Element::Point(indices));

        assert_eq!(mesh.len(), 10);
        assert_eq!(mesh.is_empty(), false);
        assert_eq!(mesh.is_valid(), true);

        assert_eq!(mesh.positions().len(), 10);
        assert_eq!(mesh.normals().len(), 10);
        assert_eq!(mesh.texcoords().len(), 10);
        assert_eq!(match mesh.elements() {
            Element::Point(_) => true,
            _ => false,
        }, true);
        assert_eq!(match mesh.elements() {
            Element::Point(indices) => indices.len(),
            _ => 0,
        }, 10);
    }

    #[test]
    fn test_make_quad() {
        let arena = Arena::new(1024);

        let mesh = make_quads(&arena, Vec2u::new(1, 1), Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0)).unwrap();

        assert_eq!(mesh.len(), 4);
        assert_eq!(mesh.is_empty(), false);
        assert_eq!(mesh.is_valid(), true);

        assert_eq!(mesh.positions().len(), 4);
        assert_eq!(mesh.normals().len(), 4);
        assert_eq!(mesh.texcoords().len(), 4);
        assert_eq!(match mesh.elements() {
            Element::Quad(_) => true,
            _ => false,
        }, true);
        assert_eq!(match mesh.elements() {
            Element::Quad(indices) => indices.len(),
            _ => 0,
        }, 1);
    }
}
