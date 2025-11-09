use crate::color::Color;

#[derive(Debug, Clone)] // Añade Debug y Clone aquí
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Texture {
    pub fn load(path: &str) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgba8();
        let (width, height) = img.dimensions();
        let pixels = img.pixels()
            .map(|p| Color::new(p[0], p[1], p[2]))
            .collect();

        Ok(Self { width, height, pixels })
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);
        self.pixels[(y * self.width + x) as usize]
    }
}

// --- Skybox simple face-based ---
pub struct Skybox {
    // Orden: +X, -X, +Y, -Y, +Z, -Z
    px: Texture,
    nx: Texture,
    py: Texture,
    ny: Texture,
    pz: Texture,
    nz: Texture,
}

impl Skybox {
    pub fn load(px: &str, nx: &str, py: &str, ny: &str, pz: &str, nz: &str) -> Result<Self, String> {
        Ok(Skybox {
            px: Texture::load(px).map_err(|e| format!("skybox px: {}", e))?,
            nx: Texture::load(nx).map_err(|e| format!("skybox nx: {}", e))?,
            py: Texture::load(py).map_err(|e| format!("skybox py: {}", e))?,
            ny: Texture::load(ny).map_err(|e| format!("skybox ny: {}", e))?,
            pz: Texture::load(pz).map_err(|e| format!("skybox pz: {}", e))?,
            nz: Texture::load(nz).map_err(|e| format!("skybox nz: {}", e))?,
        })
    }

    // direction: Vec3 en espacio de cámara (normalizado).
    // devuelve Color
    pub fn sample(&self, dir: &nalgebra_glm::Vec3) -> crate::color::Color {
        use nalgebra_glm as glm;
        let x = dir.x;
        let y = dir.y;
        let z = dir.z;

        // Encuentra el mayor componente absoluto para determinar la cara
        let ax = x.abs();
        let ay = y.abs();
        let az = z.abs();

        // u,v deben estar en [0,1] para la textura seleccionada
        let (tex, u, v) = if ax >= ay && ax >= az {
            // +/- X faces
            if x > 0.0 {
                // +X face (px)
                let u = ( -z / ax + 1.0) * 0.5;
                let v = ( -y / ax + 1.0) * 0.5;
                (&self.px, u, v)
            } else {
                // -X face (nx)
                let u = ( z / ax + 1.0) * 0.5;
                let v = ( -y / ax + 1.0) * 0.5;
                (&self.nx, u, v)
            }
        } else if ay >= ax && ay >= az {
            // +/- Y faces
            if y > 0.0 {
                // +Y (py)
                let u = ( x / ay + 1.0) * 0.5;
                let v = ( z / ay + 1.0) * 0.5;
                (&self.py, u, v)
            } else {
                // -Y (ny)
                let u = ( x / ay + 1.0) * 0.5;
                let v = ( -z / ay + 1.0) * 0.5;
                (&self.ny, u, v)
            }
        } else {
            // +/- Z faces
            if z > 0.0 {
                // +Z (pz)
                let u = ( x / az + 1.0) * 0.5;
                let v = ( -y / az + 1.0) * 0.5;
                (&self.pz, u, v)
            } else {
                // -Z (nz)
                let u = ( -x / az + 1.0) * 0.5;
                let v = ( -y / az + 1.0) * 0.5;
                (&self.nz, u, v)
            }
        };

        // sample(u,v) devuelve Color (tu Texture ya tiene sample)
        tex.sample(u % 1.0, v % 1.0)
    }
}
