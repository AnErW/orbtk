use std::{collections::VecDeque, rc::Rc};

use crate::prelude::*;

// The `WindowState` handles the window events.
#[derive(Default, AsAny)]
struct WindowState {
    events: VecDeque<WindowEvent>,
}

impl WindowState {
    fn push_event(&mut self, event: WindowEvent) {
        self.events.push_front(event);
    }

    fn resize(&self, width: f64, height: f64, ctx: &mut Context) {
        ctx.window()
            .get_mut::<Rectangle>("bounds")
            .set_size(width, height);
        ctx.window()
            .get_mut::<Constraint>("constraint")
            .set_size(width, height);
    }

    fn active_changed(&self, active: bool, ctx: &mut Context) {
        ctx.window().set("active", active);

        if !active {
            // remove focus if the window is not active
            if let Some(focused_widget) = ctx.window().get::<Global>("global").focused_widget {
                ctx.window().get_mut::<Global>("global").focused_widget = None;
                ctx.get_widget(focused_widget).set("focus", false);
                ctx.get_widget(focused_widget).update_theme_by_state(false);
            }
        }
    }
}

impl State for WindowState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(event) = self.events.pop_front() {
            match event {
                WindowEvent::Resize { width, height } => {
                    self.resize(width, height, ctx);
                }
                WindowEvent::ActiveChanged(active) => {
                    self.active_changed(active, ctx);
                }
                _ => {}
            }
        }
    }
}

widget!(
    /// The `Window` widget provides access to the properties of a application window.
    /// It also contains global properties like keyboard modifier and focused widget.
    ///
    /// **CSS element:** `window`
    Window<WindowState> {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the title property.
        title: String,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the resizeable property.
        resizeable: bool,

        /// Sets or shares a value that describes if the current window is active.
        active: bool,

        /// Sets or shares the theme property.
        theme: Theme
    }
);

impl Window {
    fn on_window_event<H: Fn(&mut StatesContext, WindowEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(WindowEventHandler {
            handler: Rc::new(handler),
        })
    }
}

impl Template for Window {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("Window")
            .background(colors::BRIGHT_GRAY_COLOR)
            .size(100.0, 100.0)
            .selector("window")
            .title("Window")
            .theme(default_theme())
            .resizeable(false)
            .on_window_event(move |ctx, event| {
                ctx.get_mut::<WindowState>(id).push_event(event);
                true
            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(ClearRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(GridLayout::new())
    }
}
