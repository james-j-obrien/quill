//! Complex example with multiple views
mod button;
mod dialog;
mod enter_exit;
mod slider;
mod splitter;
mod swatch;
mod test_scene;
mod viewport;

use bevy::{prelude::*, ui};
use bevy_mod_picking::{
    picking_core::{CorePlugin, InteractionPlugin},
    prelude::*,
};
use bevy_quill::prelude::*;
use button::{button, ButtonProps, Clicked};
use dialog::{dialog, RequestClose};
use enter_exit::EnterExitPlugin;
use slider::{h_slider, OnChange, SliderPlugin, SliderProps};
use splitter::{v_splitter, SplitterDragged, SplitterPlugin, SplitterProps};
use static_init::dynamic;
use swatch::{swatch, swatch_grid, SwatchGridProps, SwatchProps};
use viewport::{ViewportInset, ViewportInsetElement};

fn main() {
    App::new()
        .init_resource::<ViewportInset>()
        .init_resource::<PanelWidth>()
        .init_resource::<ClickLog>()
        .insert_resource(EditColor {
            color: Color::Rgba {
                red: 1.0,
                green: 0.5,
                blue: 0.0,
                alpha: 1.0,
            },
        })
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    file_path: "examples/complex/assets".to_string(),
                    ..Default::default()
                }),
        )
        .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_plugins(EventListenerPlugin::<Clicked>::default())
        .add_plugins(EventListenerPlugin::<RequestClose>::default())
        .add_plugins((QuillPlugin, SplitterPlugin, SliderPlugin, EnterExitPlugin))
        .add_systems(Startup, (test_scene::setup, setup_view_root))
        .add_event::<Clicked>()
        .add_event::<RequestClose>()
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                test_scene::rotate,
                test_scene::update_viewport_inset,
                test_scene::update_camera_viewport,
            ),
        )
        .run();
}

#[dynamic]
static STYLE_MAIN: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .left(10.)
        .top(10.)
        .bottom(10)
        .right(10.)
        .border(1)
        .border_color("#888")
        .display(ui::Display::Flex)
});

#[dynamic]
static STYLE_ASIDE: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#222")
        .display(ui::Display::Flex)
        .padding(8)
        .gap(8)
        .flex_direction(ui::FlexDirection::Column)
        .width(200)
});

#[dynamic]
static COLOR_EDIT: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .gap(8)
});

#[dynamic]
static STYLE_VIEWPORT: StyleHandle = StyleHandle::build(|ss| {
    ss.flex_grow(1.)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .justify_content(ui::JustifyContent::FlexEnd)
});

#[dynamic]
static STYLE_LOG: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#0008")
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_self(ui::AlignSelf::Stretch)
        .height(ui::Val::Percent(30.))
        .margin(8)
});

#[dynamic]
static STYLE_LOG_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .justify_content(ui::JustifyContent::FlexEnd)
        .align_self(ui::AlignSelf::Stretch)
        .flex_grow(1.)
        .flex_basis(0)
        .overflow(ui::OverflowAxis::Clip)
        .gap(3)
        .margin(8)
});

#[dynamic]
static STYLE_LOG_ENTRY: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .justify_content(ui::JustifyContent::SpaceBetween)
        .align_self(ui::AlignSelf::Stretch)
});

#[dynamic]
static COLORS: Vec<Color> = vec![
    Color::hex("#fff").unwrap(),
    Color::hex("#ffc").unwrap(),
    Color::hex("#ff8").unwrap(),
    Color::hex("#ff4").unwrap(),
    Color::hex("#ff0").unwrap(),
    Color::hex("#fcf").unwrap(),
    Color::hex("#fcc").unwrap(),
    Color::hex("#fc8").unwrap(),
    Color::hex("#fc4").unwrap(),
    Color::hex("#fc0").unwrap(),
    Color::hex("#f8f").unwrap(),
    Color::hex("#f8c").unwrap(),
    Color::hex("#f88").unwrap(),
    Color::hex("#f84").unwrap(),
    Color::hex("#f80").unwrap(),
    Color::hex("#f4f").unwrap(),
    Color::hex("#f4c").unwrap(),
    Color::hex("#f48").unwrap(),
    Color::hex("#f44").unwrap(),
    Color::hex("#f40").unwrap(),
    Color::hex("#f0f").unwrap(),
    Color::hex("#f0c").unwrap(),
    Color::hex("#f08").unwrap(),
    Color::hex("#f04").unwrap(),
    Color::hex("#f00").unwrap(),
];

#[derive(Resource)]
pub struct PanelWidth {
    value: f32,
}

#[derive(Resource)]
pub struct EditColor {
    color: Color,
}

impl Default for PanelWidth {
    fn default() -> Self {
        Self { value: 160. }
    }
}

#[derive(Resource, Default)]
pub struct ClickLog(Vec<String>);

fn setup_view_root(mut commands: Commands) {
    commands.spawn(ViewHandle::new(ui_main, ()));
}

fn ui_main(mut cx: Cx) -> impl View {
    let target = cx.use_view_entity_mut().id();
    let open = cx.use_local(|| false);
    let open_1 = open.clone();
    let mut open_2 = open.clone();
    let open_3 = open.clone();
    cx.use_view_entity_mut().insert(On::<RequestClose>::run(
        move |_ev: Listener<RequestClose>| open_2.set(false),
    ));
    // cx.use_callback::<bool>(|| {
    //     println!("Called back");
    // });
    let width = cx.use_resource::<PanelWidth>();
    Element::new()
        .styled(STYLE_MAIN.clone())
        .once(move |mut e| {
            let id = e.id();
            e.insert((On::<SplitterDragged>::run(
                move |ev: Listener<SplitterDragged>,
                      mut width: ResMut<PanelWidth>,
                      query: Query<(&Node, &GlobalTransform)>| {
                    match query.get(id) {
                        Ok((node, transform)) => {
                            // Measure node width and clamp split position.
                            let node_width = node.logical_rect(transform).width();
                            width.value = ev.value.clamp(100., node_width - 100.);
                        }
                        _ => return,
                    }
                },
            ),));
        })
        .children((
            Element::new()
                .styled((
                    STYLE_ASIDE.clone(),
                    StyleHandle::build(|b| b.width(width.value.floor())),
                ))
                .once(move |mut e| {
                    let mut open_2 = open_1.clone();
                    e.insert(On::<Clicked>::run(
                        move |ev: Listener<Clicked>, mut log: ResMut<ClickLog>| {
                            if ev.id == "save" {
                                open_2.set(true)
                            }
                            log.0
                                .push(format!("Button Clicked: id='{}'", ev.id).to_string());
                        },
                    ));
                })
                .children((
                    button.bind(ButtonProps {
                        id: "save",
                        children: "Save",
                    }),
                    button.bind(ButtonProps {
                        id: "load",
                        children: ViewParam::new(Fragment::new((
                            "Load",
                            swatch.bind(SwatchProps { color: Color::RED }),
                        ))),
                    }),
                    button.bind(ButtonProps {
                        id: "quit",
                        children: "Quit",
                    }),
                    color_edit,
                )),
            v_splitter.bind(SplitterProps {
                id: "",
                value: width.value,
            }),
            Element::new()
                .styled(STYLE_VIEWPORT.clone())
                .insert(ViewportInsetElement {})
                .children(event_log),
            dialog.bind(dialog::DemoDialogProps {
                open: open_3.get(),
                target,
            }),
        ))
}

fn color_edit(mut cx: Cx) -> impl View {
    let edit_color = cx.use_resource::<EditColor>();
    Element::new()
        .styled(COLOR_EDIT.clone())
        .once(|mut e| {
            e.insert((On::<OnChange<f32>>::run(
                move |ev: Listener<OnChange<f32>>, mut color: ResMut<EditColor>| match ev.id {
                    "r" => {
                        color.as_mut().color.set_r(ev.value / 255.0);
                    }
                    "g" => {
                        color.as_mut().color.set_g(ev.value / 255.0);
                    }
                    "b" => {
                        color.as_mut().color.set_b(ev.value / 255.0);
                    }
                    _ => (),
                },
            ),));
        })
        .children((
            swatch.bind(SwatchProps {
                color: edit_color.color,
            }),
            swatch_grid.bind(SwatchGridProps {
                colors: &COLORS,
                row_span: 4,
            }),
            h_slider.bind(SliderProps {
                id: "r",
                min: 0.,
                max: 255.,
                value: edit_color.color.r() * 255.0,
            }),
            h_slider.bind(SliderProps {
                id: "g",
                min: 0.,
                max: 255.,
                value: edit_color.color.g() * 255.0,
            }),
            h_slider.bind(SliderProps {
                id: "b",
                min: 0.,
                max: 255.,
                value: edit_color.color.b() * 255.0,
            }),
        ))
}

fn event_log(mut cx: Cx) -> impl View {
    let log = cx.use_resource::<ClickLog>();
    Element::new().styled(STYLE_LOG.clone()).children(
        Element::new()
            .styled(STYLE_LOG_INNER.clone())
            .children(For::each(&log.0, |item| {
                Element::new()
                    .styled(STYLE_LOG_ENTRY.clone())
                    .children((item.to_owned(), "00:00:00"))
            })),
    )
}
