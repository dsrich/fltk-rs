/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type custom_draw_callback =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Widget_x(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_y(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_width(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_height(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_label(arg1: *mut Fl_Widget) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Widget_set_label(arg1: *mut Fl_Widget, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Widget_redraw(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_show(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_hide(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_activate(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_deactivate(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_redraw_label(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_resize(
        arg1: *mut Fl_Widget,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Widget_tooltip(arg1: *mut Fl_Widget) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Widget_set_tooltip(arg1: *mut Fl_Widget, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Widget_get_type(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_type(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_color(arg1: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_color(arg1: *mut Fl_Widget, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Widget_label_color(arg1: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_label_color(arg1: *mut Fl_Widget, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Widget_label_font(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_font(arg1: *mut Fl_Widget, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_label_size(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_size(arg1: *mut Fl_Widget, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_label_type(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_label_type(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_box(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_box(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_changed(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_changed(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_clear_changed(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_align(arg1: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_align(arg1: *mut Fl_Widget, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_delete(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_set_image(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Widget_set_image_with_size(
        arg1: *mut Fl_Widget,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Widget_set_handler(
        self_: *mut Fl_Widget,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_set_draw(
        self_: *mut Fl_Widget,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Widget_set_trigger(arg1: *mut Fl_Widget, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_image(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_parent(self_: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_selection_color(arg1: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_selection_color(arg1: *mut Fl_Widget, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Widget_do_callback(arg1: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_inside(
        self_: *const Fl_Widget,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_window(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_top_window(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_takes_events(arg1: *const Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_user_data(arg1: *const Fl_Widget) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Widget_take_focus(self_: *mut Fl_Widget) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_set_visible_focus(self_: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_clear_visible_focus(self_: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_visible_focus(self_: *mut Fl_Widget, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Widget_has_visible_focus(self_: *mut Fl_Widget) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Widget_set_user_data(arg1: *mut Fl_Widget, data: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Menu_Item {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Menu_Bar {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Menu_Bar_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Bar;
}
extern "C" {
    pub fn Fl_Menu_Bar_x(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_y(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_width(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_height(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_label(arg1: *mut Fl_Menu_Bar) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label(arg1: *mut Fl_Menu_Bar, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Menu_Bar_redraw(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_show(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_hide(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_activate(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_deactivate(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_redraw_label(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_resize(
        arg1: *mut Fl_Menu_Bar,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_tooltip(arg1: *mut Fl_Menu_Bar) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_tooltip(arg1: *mut Fl_Menu_Bar, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Menu_Bar_get_type(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_type(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_color(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_color(arg1: *mut Fl_Menu_Bar, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_color(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_color(arg1: *mut Fl_Menu_Bar, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_font(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_font(arg1: *mut Fl_Menu_Bar, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_size(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_size(arg1: *mut Fl_Menu_Bar, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_type(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_type(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_box(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_box(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_changed(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_changed(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_clear_changed(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_align(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_align(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_delete(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_set_image(arg1: *mut Fl_Menu_Bar, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Menu_Bar_set_image_with_size(
        arg1: *mut Fl_Menu_Bar,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_set_handler(
        self_: *mut Fl_Menu_Bar,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_set_draw(
        self_: *mut Fl_Menu_Bar,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_set_trigger(arg1: *mut Fl_Menu_Bar, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_image(arg1: *const Fl_Menu_Bar) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Bar_parent(self_: *const Fl_Menu_Bar) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Bar_selection_color(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_selection_color(arg1: *mut Fl_Menu_Bar, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Bar_do_callback(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_inside(
        self_: *const Fl_Menu_Bar,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_window(arg1: *const Fl_Menu_Bar) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Bar_top_window(arg1: *const Fl_Menu_Bar) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Bar_takes_events(arg1: *const Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_user_data(arg1: *const Fl_Menu_Bar) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Bar_take_focus(self_: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_visible_focus(self_: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_clear_visible_focus(self_: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_visible_focus(self_: *mut Fl_Menu_Bar, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_has_visible_focus(self_: *mut Fl_Menu_Bar) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_user_data(arg1: *mut Fl_Menu_Bar, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Menu_Bar_add(
        arg1: *mut Fl_Menu_Bar,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_insert(
        arg1: *mut Fl_Menu_Bar,
        index: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_get_item(
        arg1: *mut Fl_Menu_Bar,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_item(
        arg1: *mut Fl_Menu_Bar,
        item: *mut Fl_Menu_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_text_font(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_text_font(arg1: *mut Fl_Menu_Bar, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_text_size(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_text_size(arg1: *mut Fl_Menu_Bar, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_text_color(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_text_color(arg1: *mut Fl_Menu_Bar, c: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Bar_add_choice(arg1: *mut Fl_Menu_Bar, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Menu_Bar_get_choice(arg1: *mut Fl_Menu_Bar) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Bar_value(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_value(
        arg1: *mut Fl_Menu_Bar,
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_clear(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_clear_submenu(
        arg1: *mut Fl_Menu_Bar,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_size(arg1: *const Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_text(
        arg1: *const Fl_Menu_Bar,
        idx: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Bar_at(
        arg1: *const Fl_Menu_Bar,
        idx: ::std::os::raw::c_int,
    ) -> *const Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_mode(
        self_: *mut Fl_Menu_Bar,
        i: ::std::os::raw::c_int,
        fl: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_mode(
        self_: *const Fl_Menu_Bar,
        i: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_find_index(
        self_: *const Fl_Menu_Bar,
        label: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_menu(self_: *const Fl_Menu_Bar) -> *const Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Bar_remove(self_: *mut Fl_Menu_Bar, idx: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Menu_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Menu_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Button;
}
extern "C" {
    pub fn Fl_Menu_Button_x(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_y(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_width(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_height(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_label(arg1: *mut Fl_Menu_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label(
        arg1: *mut Fl_Menu_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_redraw(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_show(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_hide(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_activate(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_deactivate(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_redraw_label(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_resize(
        arg1: *mut Fl_Menu_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_tooltip(arg1: *mut Fl_Menu_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Button_set_tooltip(
        arg1: *mut Fl_Menu_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_get_type(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_type(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_color(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Button_set_color(arg1: *mut Fl_Menu_Button, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Button_label_color(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_color(arg1: *mut Fl_Menu_Button, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Button_label_font(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_font(arg1: *mut Fl_Menu_Button, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_label_size(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_size(arg1: *mut Fl_Menu_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_label_type(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_type(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_box(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_box(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_changed(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_changed(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_clear_changed(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_align(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_align(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_delete(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_set_image(arg1: *mut Fl_Menu_Button, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Menu_Button_set_image_with_size(
        arg1: *mut Fl_Menu_Button,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_set_handler(
        self_: *mut Fl_Menu_Button,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_set_draw(
        self_: *mut Fl_Menu_Button,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_set_trigger(arg1: *mut Fl_Menu_Button, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_image(arg1: *const Fl_Menu_Button) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Button_parent(self_: *const Fl_Menu_Button) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Button_selection_color(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Button_set_selection_color(
        arg1: *mut Fl_Menu_Button,
        color: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_do_callback(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_inside(
        self_: *const Fl_Menu_Button,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_window(arg1: *const Fl_Menu_Button) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Button_top_window(arg1: *const Fl_Menu_Button) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Button_takes_events(arg1: *const Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_user_data(arg1: *const Fl_Menu_Button) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Button_take_focus(self_: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_visible_focus(self_: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_clear_visible_focus(self_: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_visible_focus(self_: *mut Fl_Menu_Button, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_has_visible_focus(self_: *mut Fl_Menu_Button) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Button_set_user_data(
        arg1: *mut Fl_Menu_Button,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_add(
        arg1: *mut Fl_Menu_Button,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_insert(
        arg1: *mut Fl_Menu_Button,
        index: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_get_item(
        arg1: *mut Fl_Menu_Button,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Button_set_item(
        arg1: *mut Fl_Menu_Button,
        item: *mut Fl_Menu_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_text_font(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_text_font(arg1: *mut Fl_Menu_Button, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_text_size(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_text_size(arg1: *mut Fl_Menu_Button, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_text_color(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Button_set_text_color(arg1: *mut Fl_Menu_Button, c: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Button_add_choice(
        arg1: *mut Fl_Menu_Button,
        arg2: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_get_choice(arg1: *mut Fl_Menu_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Button_value(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_value(
        arg1: *mut Fl_Menu_Button,
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_clear(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_clear_submenu(
        arg1: *mut Fl_Menu_Button,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_size(arg1: *const Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_text(
        arg1: *const Fl_Menu_Button,
        idx: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Button_at(
        arg1: *const Fl_Menu_Button,
        idx: ::std::os::raw::c_int,
    ) -> *const Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Button_set_mode(
        self_: *mut Fl_Menu_Button,
        i: ::std::os::raw::c_int,
        fl: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_mode(
        self_: *const Fl_Menu_Button,
        i: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_find_index(
        self_: *const Fl_Menu_Button,
        label: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_menu(self_: *const Fl_Menu_Button) -> *const Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Button_remove(self_: *mut Fl_Menu_Button, idx: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Choice {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Choice_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Choice;
}
extern "C" {
    pub fn Fl_Choice_x(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_y(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_width(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_height(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_label(arg1: *mut Fl_Choice) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Choice_set_label(arg1: *mut Fl_Choice, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Choice_redraw(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_show(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_hide(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_activate(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_deactivate(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_redraw_label(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_resize(
        arg1: *mut Fl_Choice,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Choice_tooltip(arg1: *mut Fl_Choice) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Choice_set_tooltip(arg1: *mut Fl_Choice, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Choice_get_type(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_type(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_color(arg1: *mut Fl_Choice) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Choice_set_color(arg1: *mut Fl_Choice, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Choice_label_color(arg1: *mut Fl_Choice) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Choice_set_label_color(arg1: *mut Fl_Choice, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Choice_label_font(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_label_font(arg1: *mut Fl_Choice, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_label_size(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_label_size(arg1: *mut Fl_Choice, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_label_type(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_label_type(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_box(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_box(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_changed(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_changed(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_clear_changed(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_align(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_align(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_delete(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_set_image(arg1: *mut Fl_Choice, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Choice_set_image_with_size(
        arg1: *mut Fl_Choice,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Choice_set_handler(
        self_: *mut Fl_Choice,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Choice_set_draw(
        self_: *mut Fl_Choice,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Choice_set_trigger(arg1: *mut Fl_Choice, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_image(arg1: *const Fl_Choice) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Choice_parent(self_: *const Fl_Choice) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Choice_selection_color(arg1: *mut Fl_Choice) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Choice_set_selection_color(arg1: *mut Fl_Choice, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Choice_do_callback(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_inside(
        self_: *const Fl_Choice,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_window(arg1: *const Fl_Choice) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Choice_top_window(arg1: *const Fl_Choice) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Choice_takes_events(arg1: *const Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_user_data(arg1: *const Fl_Choice) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Choice_take_focus(self_: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_visible_focus(self_: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_clear_visible_focus(self_: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_visible_focus(self_: *mut Fl_Choice, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_has_visible_focus(self_: *mut Fl_Choice) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Choice_set_user_data(arg1: *mut Fl_Choice, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Choice_add(
        arg1: *mut Fl_Choice,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Choice_insert(
        arg1: *mut Fl_Choice,
        index: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Choice_get_item(
        arg1: *mut Fl_Choice,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Choice_set_item(
        arg1: *mut Fl_Choice,
        item: *mut Fl_Menu_Item,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_text_font(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_text_font(arg1: *mut Fl_Choice, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_text_size(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_text_size(arg1: *mut Fl_Choice, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_text_color(arg1: *mut Fl_Choice) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Choice_set_text_color(arg1: *mut Fl_Choice, c: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Choice_add_choice(arg1: *mut Fl_Choice, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Choice_get_choice(arg1: *mut Fl_Choice) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Choice_value(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_value(
        arg1: *mut Fl_Choice,
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_clear(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_clear_submenu(
        arg1: *mut Fl_Choice,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_size(arg1: *const Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_text(
        arg1: *const Fl_Choice,
        idx: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Choice_at(arg1: *const Fl_Choice, idx: ::std::os::raw::c_int) -> *const Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Choice_set_mode(
        self_: *mut Fl_Choice,
        i: ::std::os::raw::c_int,
        fl: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Choice_mode(
        self_: *const Fl_Choice,
        i: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_find_index(
        self_: *const Fl_Choice,
        label: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_menu(self_: *const Fl_Choice) -> *const Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Choice_remove(self_: *mut Fl_Choice, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_new(
        args: *mut *mut ::std::os::raw::c_char,
        sz: ::std::os::raw::c_int,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Item_delete(self_: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_popup(
        self_: *mut Fl_Menu_Item,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> *const Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Item_label(arg1: *mut Fl_Menu_Item) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label(arg1: *mut Fl_Menu_Item, a: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Menu_Item_label_type(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_type(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_label_color(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_color(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Menu_Item_label_font(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_font(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_label_size(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_size(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_value(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_clear(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_visible(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_show(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_hide(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_active(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_activate(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_deactivate(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_submenu(self_: *const Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_next(
        self_: *mut Fl_Menu_Item,
        idx: ::std::os::raw::c_int,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Item_callback(
        self_: *mut Fl_Menu_Item,
        c: Fl_Callback,
        p: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Menu_Item_user_data(arg1: *const Fl_Menu_Item) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Menu_Item_set_user_data(arg1: *mut Fl_Menu_Item, data: *mut ::std::os::raw::c_void);
}
