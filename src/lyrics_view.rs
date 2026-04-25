// SPDX-License-Identifier: GPL-3.0-or-later

use adw::subclass::prelude::*;
use gtk::{glib, prelude::*, CompositeTemplate};
use std::cell::RefCell;

use crate::lyrics::LrcLine;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/bassi/Amberol/lyrics-view.ui")]
    pub struct LyricsView {
        #[template_child]
        pub lyrics_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub click_gesture: TemplateChild<gtk::GestureClick>,

        pub lines: RefCell<Vec<LrcLine>>,
        pub current_active: RefCell<Option<usize>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LyricsView {
        const NAME: &'static str = "AmberolLyricsView";
        type Type = super::LyricsView;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.set_css_name("lyricsview");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LyricsView {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for LyricsView {}
}

glib::wrapper! {
    pub struct LyricsView(ObjectSubclass<imp::LyricsView>)
        @extends gtk::Widget;
}

impl Default for LyricsView {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

impl LyricsView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_lyrics(&self, lines: Option<Vec<LrcLine>>) {
        let imp = self.imp();
        
        // Clear list
        while let Some(child) = imp.lyrics_list.first_child() {
            imp.lyrics_list.remove(&child);
        }
        imp.current_active.replace(None);

        if let Some(lines) = lines {
            imp.lines.replace(lines.clone());
            for line in lines {
                let label = gtk::Label::builder()
                    .label(&line.text)
                    .wrap(true)
                    .justify(gtk::Justification::Center)
                    .margin_top(8)
                    .margin_bottom(8)
                    .build();
                label.add_css_class("title-3");
                label.add_css_class("dim-label");
                imp.lyrics_list.append(&label);
            }
        } else {
            imp.lines.replace(Vec::new());
            let label = gtk::Label::builder()
                .label(&crate::i18n::i18n("No lyrics found"))
                .wrap(true)
                .justify(gtk::Justification::Center)
                .margin_top(24)
                .build();
            label.add_css_class("title-3");
            imp.lyrics_list.append(&label);
        }
    }

    pub fn set_position(&self, position_ms: u64) {
        let imp = self.imp();
        let lines = imp.lines.borrow();
        if lines.is_empty() {
            return;
        }

        let mut active_idx = 0;
        for (i, line) in lines.iter().enumerate() {
            if position_ms >= line.time_ms {
                active_idx = i;
            } else {
                break;
            }
        }

        if imp.current_active.borrow().unwrap_or(usize::MAX) != active_idx {
            imp.current_active.replace(Some(active_idx));
            
            let mut i = 0;
            let mut child = imp.lyrics_list.first_child();
            while let Some(row) = child {
                if let Some(row_widget) = row.downcast_ref::<gtk::ListBoxRow>() {
                    if let Some(label) = row_widget.child().and_then(|c| c.downcast::<gtk::Label>().ok()) {
                        if i == active_idx {
                            label.remove_css_class("dim-label");
                            // Auto scroll
                            if let Some(bounds) = row_widget.compute_bounds(&imp.lyrics_list.get()) {
                                let adj = imp.scrolled_window.vadjustment();
                                let page_size = adj.page_size();
                                let new_value = bounds.y() as f64 - (page_size / 2.0) + (bounds.height() as f64 / 2.0);
                                let max_value = adj.upper() - page_size;
                                let clamped_value = new_value.clamp(0.0, max_value);
                                adj.set_value(clamped_value);
                            }
                        } else {
                            label.add_css_class("dim-label");
                        }
                    }
                }
                child = row.next_sibling();
                i += 1;
            }
        }
    }

    pub fn click_gesture(&self) -> gtk::GestureClick {
        self.imp().click_gesture.get()
    }
}
