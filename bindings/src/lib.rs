extern crate glib;
extern crate gtk;
extern crate gtk_sys;
#[cfg(feature = "dynamic")]
extern crate libayatana_appindicator_sys;

use gtk::glib::translate::ToGlibPtr;
#[cfg(feature = "dynamic")]
use libayatana_appindicator_sys::AppIndicator as AppIndicatorRaw;
#[cfg(feature = "dynamic")]
pub use libayatana_appindicator_sys::*;

#[cfg(not(feature = "dynamic"))]
mod bindings;
#[cfg(not(feature = "dynamic"))]
use crate::bindings::AppIndicator as AppIndicatorRaw;
#[cfg(not(feature = "dynamic"))]
pub use crate::bindings::*;

pub struct AppIndicator {
    air: *mut AppIndicatorRaw,
}
pub enum AppIndicatorCategory {
    ApplicationStatus = 0,
    Communications = 1,
    SystemServices = 2,
    Hardware = 3,
    Other = 4,
}
pub enum AppIndicatorStatus {
    Passive = 0,
    Active = 1,
    Attention = 2,
}
impl AppIndicator {
    pub fn new(title: &str, icon: &str) -> AppIndicator {
        AppIndicator {
            air: unsafe {
                app_indicator_new(
                    title.to_glib_none().0,
                    icon.to_glib_none().0,
                    AppIndicatorCategory::ApplicationStatus as u32,
                )
            },
        }
    }

    pub fn with_path(
        title: &str,
        icon: &str,
        theme_path: &str,
    ) -> AppIndicator {
        AppIndicator {
            air: unsafe {
                app_indicator_new_with_path(
                    title.to_glib_none().0,
                    icon.to_glib_none().0,
                    AppIndicatorCategory::ApplicationStatus as u32,
                    theme_path.to_glib_none().0,
                )
            },
        }
    }

    pub fn set_status(&mut self, status: AppIndicatorStatus) {
        unsafe {
            app_indicator_set_status(self.air, status as u32);
        }
    }

    pub fn set_menu(&mut self, menu: &mut gtk::Menu) {
        unsafe {
            app_indicator_set_menu(self.air, menu.to_glib_none().0);
        }
    }

    pub fn set_label(&mut self, label: &str, guide: &str) {
        unsafe {
            app_indicator_set_label(
                self.air,
                label.to_glib_none().0,
                guide.to_glib_none().0,
            );
        }
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe {
            app_indicator_set_title(self.air, title.to_glib_none().0);
        }
    }

    pub fn set_icon(&mut self, name: &str) {
        unsafe {
            app_indicator_set_icon(self.air, name.to_glib_none().0);
        }
    }
    pub fn set_icon_theme_path(&mut self, path: &str) {
        unsafe {
            app_indicator_set_icon_theme_path(self.air, path.to_glib_none().0);
        }
    }

    pub fn set_icon_full(&mut self, name: &str, desc: &str) {
        unsafe {
            app_indicator_set_icon_full(
                self.air,
                name.to_glib_none().0,
                desc.to_glib_none().0,
            );
        }
    }

    pub fn set_attention_icon(&mut self, name: &str) {
        unsafe {
            app_indicator_set_attention_icon(self.air, name.to_glib_none().0);
        }
    }

    pub fn set_attention_icon_full(&mut self, name: &str, desc: &str) {
        unsafe {
            app_indicator_set_attention_icon_full(
                self.air,
                name.to_glib_none().0,
                desc.to_glib_none().0,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
