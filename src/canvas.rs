use crate::geometry::{Point, Scale};
use crate::history::{Diff, DiffDirection, SparsePixelDelta};
use crate::SdlCanvas;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Canvas {
    data: Vec<u8>,
    width: u32,
    height: u32,
    sdl_canvas: Rc<RefCell<SdlCanvas>>,
    texture: Texture<'static>,
}

impl Canvas {
    pub fn new(width: u32, height: u32, sdl_canvas: Rc<RefCell<SdlCanvas>>) -> Canvas {
        let data_size = width as usize * height as usize * Self::BPP;
        let mut data = Vec::new();

        // FIXME: this is a very ugly and inelegant solution to deal with a "struct referencing
        // itself" problem. This piece of code should preferrably be rewritten to allow proper
        // resource management. This is not urgent or critical, however, because this program is
        // not expected to create more than one instance of `Canvas`, so a single minor memory
        // leak here may be tolerable.
        let texture_creator = Box::leak(Box::new(sdl_canvas.borrow().texture_creator()));
        data.resize(data_size, 255);
        let texture = Self::create_sdl_texture(texture_creator, width, height);

        Canvas {
            width,
            height,
            data,
            sdl_canvas,
            texture,
        }
    }

    pub fn build_image(&self) -> Vec<u8> {
        let mut buf = self.data.clone();
        for slice in buf.chunks_mut(4) {
            slice.swap(0, 2);
        }
        buf
    }

    pub fn area(&self) -> usize {
        self.width as usize * self.height as usize
    }

    #[allow(dead_code)]
    pub fn get_at(&self, x: u32, y: u32) -> Color {
        self.try_get_at(x, y).unwrap()
    }

    pub fn try_get_at(&self, x: u32, y: u32) -> Option<Color> {
        // TODO: avoid multiple bound checking
        let offset = self.calc_offset(x, y)?;
        let slice = &self.data[offset..offset + Self::BPP];
        Some(Self::color_from_slice(slice))
    }

    fn color_from_slice(slice: &[u8]) -> Color {
        assert!(slice.len() == 4);
        let b = slice[0];
        let g = slice[1];
        let r = slice[2];
        let a = slice[3];
        Color::RGBA(r, g, b, a)
    }

    fn color_to_slice(color: Color, slice: &mut [u8]) {
        assert!(slice.len() == 4);
        slice[0] = color.b;
        slice[1] = color.g;
        slice[2] = color.r;
        slice[3] = color.a;
    }

    #[allow(dead_code)]
    pub fn set_at(&mut self, x: u32, y: u32, color: Color) {
        self.try_set_at(x, y, color).unwrap();
    }

    pub fn try_set_at(&mut self, x: u32, y: u32, color: Color) -> Option<()> {
        // TODO: avoid multiple bound checking
        let offset = self.calc_offset(x, y)?;
        let slice = &mut self.data[offset..offset + Self::BPP];
        Self::color_to_slice(color, slice);
        Some(())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn sdl_canvas(&self) -> &RefCell<SdlCanvas> {
        self.sdl_canvas.as_ref()
    }

    pub fn draw(&mut self, scale: Scale, visible_rect: Rect, left_top_offset: Point<i32>) {
        self.update_sdl_texture(visible_rect);
        let texture = &self.texture;
        let query = texture.query();
        let mut texture_scaled_rect =
            Rect::new(0, 0, scale.apply(query.width), scale.apply(query.height));
        texture_scaled_rect.reposition((left_top_offset.x, left_top_offset.y));
        self.sdl_canvas
            .borrow_mut()
            .copy(texture, None, Some(texture_scaled_rect))
            .expect("Failed to draw texture");
    }

    pub fn create_shadow_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn update_shadow_data(&self, shadow_data: &mut Vec<u8>) {
        shadow_data.clear();
        shadow_data.extend_from_slice(&self.data);
    }

    pub fn compare_shadow_data(&self, shadow_data: &Vec<u8>) -> Diff {
        let mut deltas = Vec::new();
        for index in 0..self.area() {
            let left = index * Self::BPP;
            let right = left + Self::BPP;
            let before = Self::color_from_slice(&shadow_data[left..right]);
            let after = Self::color_from_slice(&self.data[left..right]);
            if before != after {
                deltas.push(SparsePixelDelta {
                    index,
                    before,
                    after,
                });
            }
        }
        Diff::Sparse(deltas)
    }

    pub fn apply_diff(&mut self, diff: &Diff, direction: DiffDirection) {
        match diff {
            Diff::Sparse(deltas) => {
                for delta in deltas.iter() {
                    let left = delta.index * Self::BPP;
                    let right = left + Self::BPP;
                    let slice = &mut self.data[left..right];
                    let color = match direction {
                        DiffDirection::Normal => delta.after,
                        DiffDirection::Reverse => delta.before,
                    };

                    Self::color_to_slice(color, slice);
                }
            }
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= 0.0
            && point.y >= 0.0
            && point.x < self.width() as f64
            && point.y < self.height() as f64
    }

    fn create_sdl_texture(
        texture_creator: &TextureCreator<WindowContext>,
        width: u32,
        height: u32,
    ) -> Texture {
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, width, height)
            .expect("Failed to create a texture for the canvas");

        texture
            .with_lock(None, |data, _| {
                for chunk in data.chunks_mut(4) {
                    chunk[0] = 100;
                    chunk[1] = 100;
                    chunk[2] = 100;
                    chunk[3] = 255;
                }
            })
            .expect("Failed to initialize the texture");

        texture
    }

    fn update_sdl_texture(&mut self, visible_rect: Rect) {
        let visible_rect = visible_rect
            .intersection(Rect::new(0, 0, self.width, self.height))
            .unwrap();
        let start_offset = self
            .calc_offset(visible_rect.left() as u32, visible_rect.top() as u32)
            .unwrap();
        let end_offset = self
            .calc_offset(
                visible_rect.right() as u32 - 1,
                visible_rect.bottom() as u32 - 1,
            )
            .unwrap();
        let slice = &self.data[start_offset..=end_offset];
        let pitch_pixels = self.width as usize;
        let pitch = pitch_pixels * Self::BPP;

        // Workaround due to numerous bugs in the input validation in "safe" sdl2 API,
        // which lead to undefined behavior in case of wrong input.
        //assert!(slice.len() >= pitch * visible_rect.height() as usize);

        self.texture
            .update(visible_rect, slice, pitch)
            .expect("Failed to fill the texture with the image data");
    }

    fn calc_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let x = x as usize;
            let y = y as usize;
            let w = self.width as usize;
            let b = Self::BPP;
            Some((y * w + x) * b)
        }
    }

    const BPP: usize = 4;
}
