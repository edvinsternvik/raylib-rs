use crate::core::color::*;
use crate::core::math::*;
use crate::core::*;
use crate::ffi;
use std::ffi::CString;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source_rec: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub type_: crate::consts::NPatchType,
}

impl From<ffi::NPatchInfo> for NPatchInfo {
    fn from(v: ffi::NPatchInfo) -> NPatchInfo {
        unsafe { std::mem::transmute(v) }
    }
}

impl Into<ffi::NPatchInfo> for NPatchInfo {
    fn into(self) -> ffi::NPatchInfo {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<ffi::NPatchInfo> for &NPatchInfo {
    fn into(self) -> ffi::NPatchInfo {
        ffi::NPatchInfo {
            sourceRec: self.source_rec.into(),
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            type_: (self.type_ as u32) as i32,
        }
    }
}

make_thin_wrapper!(Image, ffi::Image, ffi::UnloadImage);
make_thin_wrapper!(Texture2D, ffi::Texture2D, ffi::UnloadTexture);
make_thin_wrapper!(
    RenderTexture2D,
    ffi::RenderTexture2D,
    ffi::UnloadRenderTexture
);

// Prevent Textures from being sent to other threads
impl !Send for Texture2D {}
impl !Sync for Texture2D {}
impl !Send for RenderTexture2D {}
impl !Sync for RenderTexture2D {}

impl Clone for Image {
    fn clone(&self) -> Image {
        unsafe { Image(ffi::ImageCopy(self.0)) }
    }
}

impl Image {
    #[inline]
    pub fn format(&self) -> crate::consts::PixelFormat {
        let i: u32 = self.format as u32;
        unsafe { std::mem::transmute(i) }
    }

    /// Exports image as a PNG file.
    #[inline]
    pub fn export_image(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportImage(self.0, c_filename.as_ptr());
        }
    }

    /// Exports image as a PNG file.
    #[inline]
    pub fn export_image_as_code(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportImageAsCode(self.0, c_filename.as_ptr());
        }
    }

    /// Gets pixel data from `image` as a Vec of Color structs.
    #[inline]
    pub fn get_image_data(&self) -> Vec<Color> {
        unsafe {
            let image_data = ffi::GetImageData(self.0);
            let image_data_len = (self.width * self.height) as usize;
            let mut safe_image_data: Vec<Color> = Vec::with_capacity(image_data_len);
            safe_image_data.set_len(image_data_len);
            std::ptr::copy(
                image_data,
                safe_image_data.as_mut_ptr() as *mut ffi::Color,
                image_data_len,
            );
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    /// Gets normalized (`0.0` to `1.0`) pixel data from `image` as a Vec of Vector4 structs.
    #[inline]
    pub fn get_image_data_normalized(&self) -> Vec<Vector4> {
        unsafe {
            let image_data = ffi::GetImageDataNormalized(self.0);
            let image_data_len = (self.width * self.height) as usize;
            let mut safe_image_data: Vec<Vector4> = Vec::with_capacity(image_data_len);
            safe_image_data.set_len(image_data_len);
            std::ptr::copy(
                image_data,
                safe_image_data.as_mut_ptr() as *mut ffi::Vector4,
                image_data_len,
            );
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    #[inline]
    pub fn image_extract_palette(&self, max_palette_size: u32) -> Vec<Color> {
        unsafe {
            let mut palette_len = 0;
            let image_data =
                ffi::ImageExtractPalette(self.0, max_palette_size as i32, &mut palette_len);
            let mut safe_image_data: Vec<Color> = Vec::with_capacity(palette_len as usize);
            safe_image_data.set_len(palette_len as usize);
            std::ptr::copy(
                image_data,
                safe_image_data.as_mut_ptr() as *mut ffi::Color,
                palette_len as usize,
            );
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    /// Converts `image` to POT (power-of-two).
    #[inline]
    pub fn image_to_pot(&mut self, fill_color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageToPOT(&mut self.0, fill_color.into().into());
        }
    }

    /// Converts `image` data to desired pixel format.
    #[inline]
    pub fn image_format(&mut self, new_format: crate::consts::PixelFormat) {
        unsafe {
            ffi::ImageFormat(&mut self.0, (new_format as u32) as i32);
        }
    }

    /// Applies alpha mask to `image`.
    /// Alpha mask must be same size as the image. If alpha mask is not greyscale
    /// Ensure the colors are white (255, 255, 255, 255) or black (0, 0, 0, 0)
    #[inline]
    pub fn image_alpha_mask(&mut self, alpha_mask: &Image) {
        unsafe {
            ffi::ImageAlphaMask(&mut self.0, alpha_mask.0);
        }
    }

    /// Clears alpha channel on `image` to desired color.
    #[inline]
    pub fn image_alpha_clear(&mut self, color: impl Into<ffi::Color>, threshold: f32) {
        unsafe {
            ffi::ImageAlphaClear(&mut self.0, color.into().into(), threshold);
        }
    }

    /// Crops `image` depending on alpha value.
    #[inline]
    pub fn image_alpha_crop(&mut self, threshold: f32) {
        unsafe {
            ffi::ImageAlphaCrop(&mut self.0, threshold);
        }
    }

    /// Premultiplies alpha channel on `image`.
    #[inline]
    pub fn image_alpha_premultiply(&mut self) {
        unsafe {
            ffi::ImageAlphaPremultiply(&mut self.0);
        }
    }

    /// Crops `image` to a defined rectangle.
    #[inline]
    pub fn image_crop(&mut self, crop: impl Into<ffi::Rectangle>) {
        unsafe {
            ffi::ImageCrop(&mut self.0, crop.into());
        }
    }

    /// Resizes `image` (bilinear filtering).
    #[inline]
    pub fn image_resize(&mut self, new_width: i32, new_height: i32) {
        unsafe {
            ffi::ImageResize(&mut self.0, new_width, new_height);
        }
    }

    /// Resizes `image` (nearest-neighbor scaling).
    #[inline]
    pub fn image_resize_nn(&mut self, new_width: i32, new_height: i32) {
        unsafe {
            ffi::ImageResizeNN(&mut self.0, new_width, new_height);
        }
    }

    /// Resizes `image` canvas and fills with `color`.
    #[inline]
    pub fn image_resize_canvas(
        &mut self,
        new_width: i32,
        new_height: i32,
        offset_x: i32,
        offset_y: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageResizeCanvas(
                &mut self.0,
                new_width,
                new_height,
                offset_x,
                offset_y,
                color.into().into(),
            );
        }
    }

    /// Generates all mipmap levels for a provided `image`.
    #[inline]
    pub fn image_mipmaps(&mut self) {
        unsafe {
            ffi::ImageMipmaps(&mut self.0);
        }
    }

    /// Dithers `image` data to 16bpp or lower (Floyd-Steinberg dithering).
    #[inline]
    pub fn image_dither(&mut self, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
        unsafe {
            ffi::ImageDither(&mut self.0, r_bpp, g_bpp, b_bpp, a_bpp);
        }
    }

    /// Draws a source image within a destination image.
    #[inline]
    pub fn image_draw(&mut self, src: &Image, src_rec: Rectangle, dst_rec: Rectangle) {
        unsafe {
            ffi::ImageDraw(&mut self.0, src.0, src_rec.into(), dst_rec.into());
        }
    }

    /// Draws a rectangle within an image.
    #[inline]
    pub fn image_draw_rectangle(&mut self, rec: Rectangle, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageDrawRectangle(&mut self.0, rec.into(), color.into());
        }
    }

    /// Draws a rectangle within an image.
    #[inline]
    pub fn image_draw_rectangle_lines(
        &mut self,
        rec: Rectangle,
        thickness: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawRectangleLines(&mut self.0, rec.into(), thickness, color.into());
        }
    }

    /// Draws text (default font) within an image (destination).
    #[inline]
    pub fn image_draw_text(
        &mut self,
        position: impl Into<ffi::Vector2>,
        text: &str,
        font_size: i32,
        color: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::ImageDrawText(
                &mut self.0,
                position.into().into(),
                c_text.as_ptr(),
                font_size,
                color.into().into(),
            );
        }
    }

    /// Flips `image` vertically.
    #[inline]
    pub fn image_flip_vertical(&mut self) {
        unsafe {
            ffi::ImageFlipVertical(&mut self.0);
        }
    }

    /// Flips `image` horizontally.
    #[inline]
    pub fn image_flip_horizontal(&mut self) {
        unsafe {
            ffi::ImageFlipHorizontal(&mut self.0);
        }
    }

    /// Rotates `image` clockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn image_rotate_cw(&mut self) {
        unsafe {
            ffi::ImageRotateCW(&mut self.0);
        }
    }

    /// Rotates `image` counterclockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn image_rotate_ccw(&mut self) {
        unsafe {
            ffi::ImageRotateCCW(&mut self.0);
        }
    }

    /// Tints colors in `image` using specified `color`.
    #[inline]
    pub fn image_color_tint(&mut self, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageColorTint(&mut self.0, color.into().into());
        }
    }

    /// Inverts the colors in `image`.
    #[inline]
    pub fn image_color_invert(&mut self) {
        unsafe {
            ffi::ImageColorInvert(&mut self.0);
        }
    }

    /// Converts `image color to grayscale.
    #[inline]
    pub fn image_color_grayscale(&mut self) {
        unsafe {
            ffi::ImageColorGrayscale(&mut self.0);
        }
    }

    /// Adjusts the contrast of `image`.
    #[inline]
    pub fn image_color_contrast(&mut self, contrast: f32) {
        unsafe {
            ffi::ImageColorContrast(&mut self.0, contrast);
        }
    }

    /// Adjusts the brightness of `image`.
    #[inline]
    pub fn image_color_brightness(&mut self, brightness: i32) {
        unsafe {
            ffi::ImageColorBrightness(&mut self.0, brightness);
        }
    }

    /// Searches `image` for all occurences of `color` and replaces them with `replace` color.
    #[inline]
    pub fn image_color_replace(
        &mut self,
        color: impl Into<ffi::Color>,
        replace: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageColorReplace(&mut self.0, color.into().into(), replace.into().into());
        }
    }

    /// Generates a plain `color` Image.
    #[inline]
    pub fn gen_image_color(width: i32, height: i32, color: impl Into<ffi::Color>) -> Image {
        unsafe { Image(ffi::GenImageColor(width, height, color.into().into())) }
    }

    /// Generates an Image containing a vertical gradient.
    #[inline]
    pub fn gen_image_gradient_v(
        width: i32,
        height: i32,
        top: impl Into<ffi::Color>,
        bottom: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientV(
                width,
                height,
                top.into().into(),
                bottom.into().into(),
            ))
        }
    }

    /// Generates an Image containing a horizonal gradient.
    #[inline]
    pub fn gen_image_gradient_h(
        width: i32,
        height: i32,
        left: impl Into<ffi::Color>,
        right: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientH(
                width,
                height,
                left.into().into(),
                right.into().into(),
            ))
        }
    }

    /// Generates an Image containing a radial gradient.
    #[inline]
    pub fn gen_image_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: impl Into<ffi::Color>,
        outer: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientRadial(
                width,
                height,
                density,
                inner.into().into(),
                outer.into().into(),
            ))
        }
    }

    /// Generates an Image containing a checkerboard pattern.
    #[inline]
    pub fn gen_image_checked(
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: impl Into<ffi::Color>,
        col2: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageChecked(
                width,
                height,
                checks_x,
                checks_y,
                col1.into().into(),
                col2.into().into(),
            ))
        }
    }

    /// Generates an Image containing white noise.
    #[inline]
    pub fn gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
        unsafe { Image(ffi::GenImageWhiteNoise(width, height, factor)) }
    }

    /// Generates an Image containing perlin noise.
    #[inline]
    pub fn gen_image_perlin_noise(
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Image {
        unsafe {
            Image(ffi::GenImagePerlinNoise(
                width, height, offset_x, offset_y, scale,
            ))
        }
    }

    /// Generates an Image using a cellular algorithm. Bigger `tile_size` means bigger cells.
    #[inline]
    pub fn gen_image_cellular(width: i32, height: i32, tile_size: i32) -> Image {
        unsafe { Image(ffi::GenImageCellular(width, height, tile_size)) }
    }

    /// Loads image from file into CPU memory (RAM).
    pub fn load_image(filename: &str) -> Result<Image, String> {
        let c_filename = CString::new(filename).unwrap();
        let i = unsafe { ffi::LoadImage(c_filename.as_ptr()) };
        if i.data.is_null() {
            return Err(format!(
            "Image data is null. Either the file doesnt exist or the image type is unsupported."
        ));
        }
        Ok(Image(i))
    }

    /// Loads image from Color array data (RGBA - 32bit).
    pub fn load_image_ex(pixels: &[Color], width: i32, height: i32) -> Result<Image, String> {
        let expected_len = (width * height) as usize;
        if pixels.len() != expected_len {
            return Err(format!(
                "load_image_ex: Data is wrong size. Expected {}, got {}",
                expected_len,
                pixels.len()
            ));
        }
        unsafe {
            // An examination of Raylib source (textures.c) shows that it does not mutate the given pixels
            // this never fails no need for null check
            Ok(Image(ffi::LoadImageEx(
                pixels.as_ptr() as *mut ffi::Color,
                width,
                height,
            )))
        }
    }

    /// Loads image from raw data with parameters.
    pub fn load_image_pro(
        data: &[u8],
        width: i32,
        height: i32,
        format: crate::consts::PixelFormat,
    ) -> Result<Image, String> {
        let expected_len = get_pixel_data_size(width, height, format) as usize;
        if data.len() != expected_len {
            return Err(format!(
                "load_image_pro: Data is wrong size. Expected {}, got {}",
                expected_len,
                data.len()
            ));
        }
        unsafe {
            Ok(Image(ffi::LoadImagePro(
                data.as_ptr() as *mut std::os::raw::c_void,
                width,
                height,
                format as i32,
            )))
        }
    }

    /// Loads image from RAW file data.
    pub fn load_image_raw(
        filename: &str,
        width: i32,
        height: i32,
        format: i32,
        header_size: i32,
    ) -> Result<Image, String> {
        let c_filename = CString::new(filename).unwrap();
        let i =
            unsafe { ffi::LoadImageRaw(c_filename.as_ptr(), width, height, format, header_size) };
        if i.data.is_null() {
            return Err(format!(
            "Image data is null. Either the file doesnt exist or the image type is unsupported."
        ));
        }
        Ok(Image(i))
    }

    /// Creates an image from `text` (custom font).
    #[inline]
    pub fn image_text_ex(
        font: impl std::convert::AsRef<ffi::Font>,
        text: &str,
        font_size: f32,
        spacing: f32,
        tint: impl Into<Color>,
    ) -> Image {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Image(ffi::ImageTextEx(
                *font.as_ref(),
                c_text.as_ptr(),
                font_size,
                spacing,
                tint.into().into(),
            ))
        }
    }
}

impl Texture2D {
    /// Updates GPU texture with new data.
    #[inline]
    pub fn update_texture(&self, pixels: &[u8]) {
        let expected_len = unsafe {
            get_pixel_data_size(
                self.width,
                self.height,
                std::mem::transmute::<i32, ffi::PixelFormat>(self.format),
            ) as usize
        };
        if pixels.len() != expected_len {
            panic!(
                "update_texture: Data is wrong size. Expected {}, got {}",
                expected_len,
                pixels.len()
            );
        }
        unsafe {
            ffi::UpdateTexture(self.0, pixels.as_ptr() as *const std::os::raw::c_void);
        }
    }
}

/// Gets pixel data size in bytes (image or texture).
#[inline]
pub fn get_pixel_data_size(width: i32, height: i32, format: ffi::PixelFormat) -> i32 {
    unsafe { ffi::GetPixelDataSize(width, height, format as i32) }
}

impl Texture2D {
    /// Gets pixel data from GPU texture and returns an `Image`.
    /// Fairly sure this would never fail. If it does wrap in result.
    #[inline]
    pub fn get_texture_data(&self) -> Result<Image, String> {
        let i = unsafe { ffi::GetTextureData(self.0) };
        if i.data.is_null() {
            return Err(format!("Texture cannot be rendered to an image"));
        }
        Ok(Image(i))
    }
}

impl Texture2D {
    /// Generates GPU mipmaps for a `texture`.
    #[inline]
    pub fn gen_texture_mipmaps(&mut self) {
        unsafe {
            ffi::GenTextureMipmaps(&mut self.0);
        }
    }

    /// Sets `texture` scaling filter mode.
    #[inline]
    pub fn set_texture_filter(&mut self, filter_mode: crate::consts::TextureFilterMode) {
        unsafe {
            ffi::SetTextureFilter(self.0, filter_mode as i32);
        }
    }

    /// Sets texture wrapping mode.
    #[inline]
    pub fn set_texture_wrap(&mut self, wrap_mode: crate::consts::TextureWrapMode) {
        unsafe {
            ffi::SetTextureWrap(self.0, wrap_mode as i32);
        }
    }
}

impl RaylibHandle {
    /// Loads texture from file into GPU memory (VRAM).
    pub fn load_texture(&mut self, _: &RaylibThread, filename: &str) -> Result<Texture2D, String> {
        let c_filename = CString::new(filename).unwrap();
        let t = unsafe { ffi::LoadTexture(c_filename.as_ptr()) };
        if t.id == 0 {
            return Err(format!("failed to load {} as a texture.", filename));
        }
        Ok(Texture2D(t))
    }

    /// Loads texture from image data.
    #[inline]
    pub fn load_texture_from_image(
        &mut self,
        _: &RaylibThread,
        image: &Image,
    ) -> Result<Texture2D, String> {
        let t = unsafe { ffi::LoadTextureFromImage(image.0) };
        if t.id == 0 {
            return Err(format!("failed to load image as a texture."));
        }
        Ok(Texture2D(t))
    }
    /// Loads texture for rendering (framebuffer).
    pub fn load_render_texture(
        &mut self,
        _: &RaylibThread,
        width: u32,
        height: u32,
    ) -> Result<RenderTexture2D, String> {
        let t = unsafe { ffi::LoadRenderTexture(width as i32, height as i32) };
        if t.id == 0 {
            return Err(format!("failed to create render texture."));
        }
        Ok(RenderTexture2D(t))
    }
}

#[cfg(test)]
mod texture_test {
    use super::*;
    use crate::core::color::*;
    use crate::tests::*;
    #[test]
    fn test_image_loading() {
        let i = Image::load_image("resources/billboard.png").expect("image not found");
        Image::load_image("resources/doesnt_exist.png").expect_err("image found?");
        i.export_image("test_out/billboard.png");
        i.export_image_as_code("test_out/billboard_code.h");
    }

    #[test]
    fn test_image_load_ex() {
        let mut col = Vec::new();
        for _ in 0..32 {
            for _ in 0..32 {
                col.push(Color::RED);
            }
        }
        let i = Image::load_image_ex(&col, 32, 32).expect("failed to load binary image");
        assert_eq!(
            i.get_image_data().len(),
            32 * 32,
            "failed to read pixels of image"
        );
        assert_eq!(
            i.get_image_data_normalized().len(),
            32 * 32,
            "failed to read pixels of image normalized"
        );
    }

    ray_test!(test_texture_load);
    fn test_texture_load(thread: &RaylibThread) {
        let i =
            Image::load_image("resources/billboard.png").expect("could not load image billboard");
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let _ = rl
            .load_texture(thread, "resources/billboard.png")
            .expect("could not load texture billboard");
        let t = rl
            .load_texture_from_image(thread, &i)
            .expect("could not load texture from image");
        let _ = t
            .get_texture_data()
            .expect("can't get an image from a texture created from an image...");
        i.export_image("test_out/billboard_texture.png");
    }

    ray_test!(test_render_texture);
    fn test_render_texture(t: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.load_render_texture(t, 256, 256)
            .expect("render texture created");
    }

    #[test]
    fn test_image_manipulations() {
        // Just checking that nothing segfaults. Not ensuring they work as expected.
        let mut col = Vec::new();
        let mut alpha = Vec::new();
        let mut blank = Vec::new();
        for i in 0..32 {
            for j in 0..32 {
                col.push(Color::RED);
                blank.push(Color::new(0, 0, 0, 0));
                if (i / 8) % 2 == (j / 8) % 2 {
                    alpha.push(Color::new(255, 255, 255, 255))
                } else {
                    alpha.push(Color::new(0, 0, 0, 0))
                }
            }
        }
        let mut i = Image::load_image_ex(&col, 32, 32).expect("failed to load binary image");
        let mut canvas = Image::load_image_ex(&blank, 32, 32).expect("failed to load canvas image");
        let mask = Image::load_image_ex(&alpha, 32, 32).expect("failed to load alpha image");
        let mut c = i.clone();
        c.image_alpha_mask(&mask);
        c.image_alpha_clear(Color::BLUE, 0.5);
        // shouldn't do anything
        c.image_alpha_crop(0.5);
        // shouldn't do anything
        c.image_alpha_premultiply();
        let mut blurry = c.clone();
        blurry.image_resize(256, 256);
        blurry.export_image("test_out/chessboard_blurry.png");
        c.image_resize_nn(256, 256);
        i.image_resize_canvas(256, 256, 10, 10, Color::BLUE);
        i.export_image("test_out/resized.png");
        c.export_image("test_out/chessboard.png");
        c.image_mipmaps();
        blurry.image_dither(128, 128, 128, 128);
        let colors = c.image_extract_palette(100);
        assert_eq!(colors.len(), 2, "color palette extraction failed");
        canvas.image_draw(
            &i,
            Rectangle::new(0.0, 0.0, 20.0, 20.0),
            Rectangle::new(0.0, 0.0, 20.0, 20.0),
        );
        canvas.image_draw_rectangle_lines(Rectangle::new(20.0, 0.0, 20.0, 20.0), 4, Color::GREEN);
        canvas.image_draw_rectangle(Rectangle::new(40.0, 0.0, 20.0, 20.0), Color::ORANGE);
        canvas.image_flip_vertical();
        canvas.image_flip_horizontal();
        canvas.image_rotate_cw();
        canvas.image_rotate_ccw();
        canvas.image_color_tint(Color::PINK);
        canvas.image_color_invert();
        canvas.image_color_contrast(0.5);
        canvas.image_color_brightness(128);
        canvas.image_color_replace(Color::GREEN, Color::RED);
        canvas.export_image("test_out/canvas.png");

        // Test generation functions
        let g = Image::gen_image_color(64, 64, Color::BLUE);
        g.export_image("test_out/generated_color.png");
        let g = Image::gen_image_gradient_v(64, 64, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_gradient_v.png");
        let g = Image::gen_image_gradient_h(64, 64, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_gradient_h.png");
        let g = Image::gen_image_gradient_radial(64, 64, 0.5, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_gradient_radial.png");
        let g = Image::gen_image_checked(64, 64, 8, 8, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_checked.png");
        let g = Image::gen_image_white_noise(64, 64, 0.7);
        g.export_image("test_out/generated_white.png");
        let g = Image::gen_image_perlin_noise(64, 64, 0, 0, 0.7);
        g.export_image("test_out/generated_perlin.png");
        let g = Image::gen_image_cellular(64, 64, 4);
        g.export_image("test_out/generated_cellular.png");

        // c.image_format(crate::consts::PixelFormat::UNCOMPRESSED_R32G32B32A32);
    }
}
