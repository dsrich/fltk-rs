/* automatically generated by rust-bindgen 0.56.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Surface_Device {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Surface_Device_set_current(self_: *mut Fl_Surface_Device);
}
extern "C" {
    pub fn Fl_Surface_Device_is_current(self_: *mut Fl_Surface_Device) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Surface_Device_surface() -> *mut Fl_Surface_Device;
}
extern "C" {
    pub fn Fl_Surface_Device_push_current(new_current: *mut Fl_Surface_Device);
}
extern "C" {
    pub fn Fl_Surface_Device_pop_current() -> *mut Fl_Surface_Device;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Image_Surface {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Image_Surface_new(
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        high_res: ::std::os::raw::c_int,
    ) -> *mut Fl_Image_Surface;
}
extern "C" {
    pub fn Fl_Image_Surface_delete(s: *mut Fl_Image_Surface);
}
extern "C" {
    pub fn Fl_Image_Surface_set_current(self_: *mut Fl_Image_Surface);
}
extern "C" {
    pub fn Fl_Image_Surface_is_current(self_: *mut Fl_Image_Surface) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_Surface_image(self_: *mut Fl_Image_Surface) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Image_Surface_highres_image(
        self_: *mut Fl_Image_Surface,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Image_Surface_origin(
        self_: *mut Fl_Image_Surface,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_Surface_set_origin(
        self_: *mut Fl_Image_Surface,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_Surface_rescale(self_: *mut Fl_Image_Surface);
}
extern "C" {
    pub fn Fl_Image_Surface_draw(
        self_: *mut Fl_Image_Surface,
        widget: *mut ::std::os::raw::c_void,
        delta_x: ::std::os::raw::c_int,
        delta_y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_Surface_draw_decorated_window(
        self_: *mut Fl_Image_Surface,
        widget: *mut ::std::os::raw::c_void,
        delta_x: ::std::os::raw::c_int,
        delta_y: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_SVG_File_Surface {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_SVG_File_Surface_new(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        file: *const ::std::os::raw::c_char,
    ) -> *mut Fl_SVG_File_Surface;
}
extern "C" {
    pub fn Fl_SVG_File_Surface_delete(self_: *mut Fl_SVG_File_Surface);
}
extern "C" {
    pub fn Fl_SVG_File_Surface_origin(
        self_: *mut Fl_SVG_File_Surface,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_File_Surface_printable_rect(
        self_: *mut Fl_SVG_File_Surface,
        w: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_File_Surface_draw(
        self_: *mut Fl_SVG_File_Surface,
        widget: *mut ::std::os::raw::c_void,
        delta_x: ::std::os::raw::c_int,
        delta_y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_File_Surface_draw_decorated_window(
        self_: *mut Fl_SVG_File_Surface,
        widget: *mut ::std::os::raw::c_void,
        delta_x: ::std::os::raw::c_int,
        delta_y: ::std::os::raw::c_int,
    );
}
