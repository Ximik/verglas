use iced::{
    widget::{center, column, container, horizontal_space, row},
    {Element, Size, Theme},
};

mod icon;
mod macros;

fn main() -> iced::Result {
    iced::application("verglas | example", App::update, App::view)
        .font(icon::FONT_BYTES)
        .window_size(Size::new(300.0, 200.0))
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App;

impl App {
    pub fn update(&mut self, _message: ()) {}

    pub fn view(&self) -> Element<()> {
        center(
            container(
                column![
                    row![icon::book(), "Here is some text:"].spacing(3),
                    row![
                        horizontal_space(),
                        icon::artboard(),
                        icon::copy(),
                        icon::qrcode(),
                        icon::arrow_to(),
                        icon::arrow_from(),
                        icon::at(),
                        horizontal_space(),
                    ],
                ]
                .spacing(25),
            )
            .width(200)
            .height(150)
            .padding(10)
            .style(container::rounded_box),
        )
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::CatppuccinLatte
    }
}
