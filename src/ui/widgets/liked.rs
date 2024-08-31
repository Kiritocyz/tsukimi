use std::sync::atomic::Ordering;

use crate::client::client::EMBY_CLIENT;
use crate::client::error::UserFacingError;
use crate::client::structs::*;
use crate::utils::{spawn, spawn_tokio};
use crate::{fraction, fraction_reset, toast};
use gettextrs::gettext;
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use glib::subclass::InitializingObject;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    use crate::ui::widgets::hortu_scrolled::HortuScrolled;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/moe/tsukimi/liked.ui")]
    pub struct LikedPage {
        #[template_child]
        pub moviehortu: TemplateChild<HortuScrolled>,
        #[template_child]
        pub serieshortu: TemplateChild<HortuScrolled>,
        #[template_child]
        pub episodehortu: TemplateChild<HortuScrolled>,
        #[template_child]
        pub peoplehortu: TemplateChild<HortuScrolled>,
        #[template_child]
        pub albumhortu: TemplateChild<HortuScrolled>,
        #[template_child]
        pub boxsethortu: TemplateChild<HortuScrolled>,
        #[template_child]
        pub tvhortu: TemplateChild<HortuScrolled>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for LikedPage {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "LikedPage";
        type Type = super::LikedPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for LikedPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.update();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for LikedPage {}

    // Trait shared by all windows
    impl WindowImpl for LikedPage {}

    // Trait shared by all application windows
    impl ApplicationWindowImpl for LikedPage {}

    impl adw::subclass::navigation_page::NavigationPageImpl for LikedPage {}
}

glib::wrapper! {
    pub struct LikedPage(ObjectSubclass<imp::LikedPage>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget ,adw::NavigationPage,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Default for LikedPage {
    fn default() -> Self {
        Self::new()
    }
}

impl LikedPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn update(&self) {
        spawn(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            async move {
                obj.set_lists().await;
            }
        ));
    }

    pub async fn set_lists(&self) {
        fraction_reset!(self);
        self.sets("Movie").await;
        self.sets("Series").await;
        self.sets("Episode").await;
        self.sets("People").await;
        self.sets("MusicAlbum").await;
        self.sets("BoxSet").await;
        self.sets("TvChannel").await;
        self.ensure_items();
        fraction!(self);
    }

    fn ensure_items(&self) {
        let imp = self.imp();
        if !imp.moviehortu.is_visible()
            && !imp.serieshortu.is_visible()
            && !imp.episodehortu.is_visible()
            && !imp.peoplehortu.is_visible()
            && !imp.albumhortu.is_visible()
            && !imp.boxsethortu.is_visible()
            && !imp.tvhortu.is_visible()
        {
            imp.stack.set_visible_child_name("fallback");
        }
    }

    pub async fn sets(&self, types: &str) {
        let hortu = match types {
            "Movie" => self.imp().moviehortu.get(),
            "Series" => self.imp().serieshortu.get(),
            "Episode" => self.imp().episodehortu.get(),
            "People" => self.imp().peoplehortu.get(),
            "MusicAlbum" => self.imp().albumhortu.get(),
            "BoxSet" => self.imp().boxsethortu.get(),
            "TvChannel" => self.imp().tvhortu.get(),
            _ => return,
        };

        hortu.set_title(&format!("{} {}", gettext("Favourite"), gettext(types)));

        let types = types.to_string();

        let type_ = types.clone();

        let results = match spawn_tokio(
            async move { EMBY_CLIENT.get_favourite(&types, 0, 12).await },
        )
        .await
        {
            Ok(history) => history,
            Err(e) => {
                toast!(self, e.to_user_facing());
                List::default()
            }
        };

        if results.items.is_empty() {
            hortu.set_visible(false);
            return;
        }

        hortu.set_items(&results.items);

        hortu.connect_morebutton(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                let tag = format!("{} {}", "Favourite", type_);
                let page = crate::ui::widgets::single_grid::SingleGrid::new();
                let types = type_.clone();
                let type_1 = type_.clone();
                page.connect_realize(glib::clone!(
                    #[weak]
                    obj,
                    move |page| {
                        let types_clone = types.clone();
                        spawn(glib::clone!(
                            #[weak]
                            page,
                            #[weak]
                            obj,
                            async move {
                                fraction_reset!(obj);
                                let result = match spawn_tokio(async move {
                                    EMBY_CLIENT.get_favourite(&types_clone, 0, 50).await
                                })
                                .await
                                {
                                    Ok(history) => history,
                                    Err(e) => {
                                        toast!(obj, e.to_user_facing());
                                        List::default()
                                    }
                                };
                                page.add_items::<false>(result.items);
                                page.set_item_number(result.total_record_count);
                                fraction!(obj);
                            }
                        ));
                    }
                ));
                page.imp().scrolled.connect_end_edge_reached(glib::clone!(
                    #[weak]
                    obj,
                    move |scrolled, lock| {
                        let types_clone = type_1.clone();
                        spawn(glib::clone!(
                            #[weak]
                            obj,
                            #[weak]
                            scrolled,
                            async move {
                                fraction_reset!(obj);
                                let n_items = scrolled.n_items();
                                let search_results = match spawn_tokio(async move {
                                    EMBY_CLIENT.get_favourite(&types_clone, n_items, 50).await
                                })
                                .await
                                {
                                    Ok(history) => history,
                                    Err(e) => {
                                        toast!(obj, e.to_user_facing());
                                        List::default()
                                    }
                                };

                                scrolled.set_grid::<false>(search_results.items);

                                lock.store(false, Ordering::SeqCst);
                                fraction!(obj);
                            },
                        ))
                    }
                ));
                push_page_with_tag(&obj, page, tag);
            }
        ));
    }
}