use crate::canvas::Canvas;
use crate::color::FloatRgbColor;
use cairo::Context as CairoContext;
use std::sync::{Arc, RwLock};

const BACKGROUND_COLOR: FloatRgbColor = FloatRgbColor::new(0.9, 0.9, 0.9);
const MIN_MARGIN_SIZE: u32 = 50;

pub struct CanvasRenderer {
    canvas: Arc<RwLock<Canvas>>,
    allocation_width: u32,
    allocation_height: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CanvasGeometry {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct MarginGeometry {
    pub width: u32,
    pub height: u32,
}

impl CanvasRenderer {
    pub fn new(canvas: Arc<RwLock<Canvas>>, allocation_width: u32, allocation_height: u32) -> Self {
        Self {
            canvas,
            allocation_width,
            allocation_height,
        }
    }

    pub fn draw(&self, cairo: &CairoContext) {
        let canvas_geometry = self.canvas_geometry();
        let margin_geometry = self.margin_geometry(canvas_geometry);

        self.fill_background(cairo);
        self.draw_shadow(cairo, canvas_geometry, margin_geometry);
        self.draw_image(cairo, canvas_geometry, margin_geometry);
    }

    pub fn set_size_allocation(&mut self, width: u32, height: u32) {
        self.allocation_width = width;
        self.allocation_height = height;
    }

    pub fn min_total_size(&self) -> (u32, u32) {
        let canvas_geometry = self.canvas_geometry();
        let min_total_width = canvas_geometry.width + 2 * MIN_MARGIN_SIZE;
        let min_total_height = canvas_geometry.height + 2 * MIN_MARGIN_SIZE;
        (min_total_width, min_total_height)
    }

    fn fill_background(&self, cairo: &CairoContext) {
        cairo.set_source_rgb(
            BACKGROUND_COLOR.red,
            BACKGROUND_COLOR.green,
            BACKGROUND_COLOR.blue,
        );
        cairo.paint().unwrap();
    }

    fn canvas_geometry(&self) -> CanvasGeometry {
        let lock = self.canvas.read().unwrap();
        let canvas_width = lock.width();
        let canvas_height = lock.height();

        CanvasGeometry {
            width: canvas_width,
            height: canvas_height,
        }
    }

    fn margin_geometry(&self, canvas_geometry: CanvasGeometry) -> MarginGeometry {
        let margin_width_allocation =
            self.allocation_width.saturating_sub(canvas_geometry.width) / 2;
        let margin_height_allocation = self
            .allocation_height
            .saturating_sub(canvas_geometry.height)
            / 2;

        let margin_width = margin_width_allocation.max(MIN_MARGIN_SIZE);
        let margin_height = margin_height_allocation.max(MIN_MARGIN_SIZE);
        MarginGeometry {
            width: margin_width,
            height: margin_height,
        }
    }

    fn draw_shadow(
        &self,
        cairo: &CairoContext,
        canvas_geometry: CanvasGeometry,
        margin_geometry: MarginGeometry,
    ) {
        let base_x = margin_geometry.width as f64;
        let base_y = margin_geometry.height as f64;
        let base_width = canvas_geometry.width as f64;
        let base_height = canvas_geometry.height as f64;

        // (pixel offset, shade strength) pairs.
        let shadow = [(0.5, 0.6), (1.5, 0.3), (2.5, 0.15)];
        for (offset, shade) in shadow {
            let lightness = 1.0 - shade;
            cairo.set_source_rgb(lightness, lightness, lightness);
            cairo.set_line_width(1.0);
            cairo.rectangle(
                base_x - offset,
                base_y - offset,
                base_width + 2.0 * offset,
                base_height + 2.0 * offset,
            );
            cairo.stroke().unwrap();
        }
    }

    fn draw_image(
        &self,
        cairo: &CairoContext,
        canvas_geometry: CanvasGeometry,
        margin_geometry: MarginGeometry,
    ) {
        // TODO: actual image.
        cairo.set_source_rgb(1.0, 1.0, 1.0);

        let x = margin_geometry.width as f64;
        let y = margin_geometry.height as f64;
        let width = canvas_geometry.width as f64;
        let height = canvas_geometry.height as f64;
        cairo.rectangle(x, y, width, height);
        cairo.fill().unwrap();
    }
}
