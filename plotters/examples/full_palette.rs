use plotters::prelude::*;

const OUT_FILE_NAME: &str = "plotters-doc-data/full_palette.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (2000, 850)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Demonstration of full_palette Colors", ("sans-serif", 50))
        .build_cartesian_2d(-0.5f32..19f32, -1f32..15f32)?;

    use full_palette::*;
    let colors = [
        [
            RED, RED_50, RED_100, RED_200, RED_300, RED_400, RED_500, RED_600, RED_700, RED_800,
            RED_900, RED_A100, RED_A200, RED_A400, RED_A700,
        ],
        [
            PINK, PINK_50, PINK_100, PINK_200, PINK_300, PINK_400, PINK_500, PINK_600, PINK_700,
            PINK_800, PINK_900, PINK_A100, PINK_A200, PINK_A400, PINK_A700,
        ],
        [
            PURPLE,
            PURPLE_50,
            PURPLE_100,
            PURPLE_200,
            PURPLE_300,
            PURPLE_400,
            PURPLE_500,
            PURPLE_600,
            PURPLE_700,
            PURPLE_800,
            PURPLE_900,
            PURPLE_A100,
            PURPLE_A200,
            PURPLE_A400,
            PURPLE_A700,
        ],
        [
            DEEPPURPLE,
            DEEPPURPLE_50,
            DEEPPURPLE_100,
            DEEPPURPLE_200,
            DEEPPURPLE_300,
            DEEPPURPLE_400,
            DEEPPURPLE_500,
            DEEPPURPLE_600,
            DEEPPURPLE_700,
            DEEPPURPLE_800,
            DEEPPURPLE_900,
            DEEPPURPLE_A100,
            DEEPPURPLE_A200,
            DEEPPURPLE_A400,
            DEEPPURPLE_A700,
        ],
        [
            INDIGO,
            INDIGO_50,
            INDIGO_100,
            INDIGO_200,
            INDIGO_300,
            INDIGO_400,
            INDIGO_500,
            INDIGO_600,
            INDIGO_700,
            INDIGO_800,
            INDIGO_900,
            INDIGO_A100,
            INDIGO_A200,
            INDIGO_A400,
            INDIGO_A700,
        ],
        [
            BLUE, BLUE_50, BLUE_100, BLUE_200, BLUE_300, BLUE_400, BLUE_500, BLUE_600, BLUE_700,
            BLUE_800, BLUE_900, BLUE_A100, BLUE_A200, BLUE_A400, BLUE_A700,
        ],
        [
            LIGHTBLUE,
            LIGHTBLUE_50,
            LIGHTBLUE_100,
            LIGHTBLUE_200,
            LIGHTBLUE_300,
            LIGHTBLUE_400,
            LIGHTBLUE_500,
            LIGHTBLUE_600,
            LIGHTBLUE_700,
            LIGHTBLUE_800,
            LIGHTBLUE_900,
            LIGHTBLUE_A100,
            LIGHTBLUE_A200,
            LIGHTBLUE_A400,
            LIGHTBLUE_A700,
        ],
        [
            CYAN, CYAN_50, CYAN_100, CYAN_200, CYAN_300, CYAN_400, CYAN_500, CYAN_600, CYAN_700,
            CYAN_800, CYAN_900, CYAN_A100, CYAN_A200, CYAN_A400, CYAN_A700,
        ],
        [
            TEAL, TEAL_50, TEAL_100, TEAL_200, TEAL_300, TEAL_400, TEAL_500, TEAL_600, TEAL_700,
            TEAL_800, TEAL_900, TEAL_A100, TEAL_A200, TEAL_A400, TEAL_A700,
        ],
        [
            GREEN, GREEN_50, GREEN_100, GREEN_200, GREEN_300, GREEN_400, GREEN_500, GREEN_600,
            GREEN_700, GREEN_800, GREEN_900, GREEN_A100, GREEN_A200, GREEN_A400, GREEN_A700,
        ],
        [
            LIGHTGREEN,
            LIGHTGREEN_50,
            LIGHTGREEN_100,
            LIGHTGREEN_200,
            LIGHTGREEN_300,
            LIGHTGREEN_400,
            LIGHTGREEN_500,
            LIGHTGREEN_600,
            LIGHTGREEN_700,
            LIGHTGREEN_800,
            LIGHTGREEN_900,
            LIGHTGREEN_A100,
            LIGHTGREEN_A200,
            LIGHTGREEN_A400,
            LIGHTGREEN_A700,
        ],
        [
            LIME, LIME_50, LIME_100, LIME_200, LIME_300, LIME_400, LIME_500, LIME_600, LIME_700,
            LIME_800, LIME_900, LIME_A100, LIME_A200, LIME_A400, LIME_A700,
        ],
        [
            YELLOW,
            YELLOW_50,
            YELLOW_100,
            YELLOW_200,
            YELLOW_300,
            YELLOW_400,
            YELLOW_500,
            YELLOW_600,
            YELLOW_700,
            YELLOW_800,
            YELLOW_900,
            YELLOW_A100,
            YELLOW_A200,
            YELLOW_A400,
            YELLOW_A700,
        ],
        [
            AMBER, AMBER_50, AMBER_100, AMBER_200, AMBER_300, AMBER_400, AMBER_500, AMBER_600,
            AMBER_700, AMBER_800, AMBER_900, AMBER_A100, AMBER_A200, AMBER_A400, AMBER_A700,
        ],
        [
            ORANGE,
            ORANGE_50,
            ORANGE_100,
            ORANGE_200,
            ORANGE_300,
            ORANGE_400,
            ORANGE_500,
            ORANGE_600,
            ORANGE_700,
            ORANGE_800,
            ORANGE_900,
            ORANGE_A100,
            ORANGE_A200,
            ORANGE_A400,
            ORANGE_A700,
        ],
        [
            DEEPORANGE,
            DEEPORANGE_50,
            DEEPORANGE_100,
            DEEPORANGE_200,
            DEEPORANGE_300,
            DEEPORANGE_400,
            DEEPORANGE_500,
            DEEPORANGE_600,
            DEEPORANGE_700,
            DEEPORANGE_800,
            DEEPORANGE_900,
            DEEPORANGE_A100,
            DEEPORANGE_A200,
            DEEPORANGE_A400,
            DEEPORANGE_A700,
        ],
        [
            BROWN, BROWN_50, BROWN_100, BROWN_200, BROWN_300, BROWN_400, BROWN_500, BROWN_600,
            BROWN_700, BROWN_800, BROWN_900, BROWN_A100, BROWN_A200, BROWN_A400, BROWN_A700,
        ],
        [
            GREY, GREY_50, GREY_100, GREY_200, GREY_300, GREY_400, GREY_500, GREY_600, GREY_700,
            GREY_800, GREY_900, GREY_A100, GREY_A200, GREY_A400, GREY_A700,
        ],
        [
            BLUEGREY,
            BLUEGREY_50,
            BLUEGREY_100,
            BLUEGREY_200,
            BLUEGREY_300,
            BLUEGREY_400,
            BLUEGREY_500,
            BLUEGREY_600,
            BLUEGREY_700,
            BLUEGREY_800,
            BLUEGREY_900,
            BLUEGREY_A100,
            BLUEGREY_A200,
            BLUEGREY_A400,
            BLUEGREY_A700,
        ],
    ];
    let color_names = [
        [
            "RED", "RED_50", "RED_100", "RED_200", "RED_300", "RED_400", "RED_500", "RED_600",
            "RED_700", "RED_800", "RED_900", "RED_A100", "RED_A200", "RED_A400", "RED_A700",
        ],
        [
            "PINK",
            "PINK_50",
            "PINK_100",
            "PINK_200",
            "PINK_300",
            "PINK_400",
            "PINK_500",
            "PINK_600",
            "PINK_700",
            "PINK_800",
            "PINK_900",
            "PINK_A100",
            "PINK_A200",
            "PINK_A400",
            "PINK_A700",
        ],
        [
            "PURPLE",
            "PURPLE_50",
            "PURPLE_100",
            "PURPLE_200",
            "PURPLE_300",
            "PURPLE_400",
            "PURPLE_500",
            "PURPLE_600",
            "PURPLE_700",
            "PURPLE_800",
            "PURPLE_900",
            "PURPLE_A100",
            "PURPLE_A200",
            "PURPLE_A400",
            "PURPLE_A700",
        ],
        [
            "DEEPPURPLE",
            "DEEPPURPLE_50",
            "DEEPPURPLE_100",
            "DEEPPURPLE_200",
            "DEEPPURPLE_300",
            "DEEPPURPLE_400",
            "DEEPPURPLE_500",
            "DEEPPURPLE_600",
            "DEEPPURPLE_700",
            "DEEPPURPLE_800",
            "DEEPPURPLE_900",
            "DEEPPURPLE_A100",
            "DEEPPURPLE_A200",
            "DEEPPURPLE_A400",
            "DEEPPURPLE_A700",
        ],
        [
            "INDIGO",
            "INDIGO_50",
            "INDIGO_100",
            "INDIGO_200",
            "INDIGO_300",
            "INDIGO_400",
            "INDIGO_500",
            "INDIGO_600",
            "INDIGO_700",
            "INDIGO_800",
            "INDIGO_900",
            "INDIGO_A100",
            "INDIGO_A200",
            "INDIGO_A400",
            "INDIGO_A700",
        ],
        [
            "BLUE",
            "BLUE_50",
            "BLUE_100",
            "BLUE_200",
            "BLUE_300",
            "BLUE_400",
            "BLUE_500",
            "BLUE_600",
            "BLUE_700",
            "BLUE_800",
            "BLUE_900",
            "BLUE_A100",
            "BLUE_A200",
            "BLUE_A400",
            "BLUE_A700",
        ],
        [
            "LIGHTBLUE",
            "LIGHTBLUE_50",
            "LIGHTBLUE_100",
            "LIGHTBLUE_200",
            "LIGHTBLUE_300",
            "LIGHTBLUE_400",
            "LIGHTBLUE_500",
            "LIGHTBLUE_600",
            "LIGHTBLUE_700",
            "LIGHTBLUE_800",
            "LIGHTBLUE_900",
            "LIGHTBLUE_A100",
            "LIGHTBLUE_A200",
            "LIGHTBLUE_A400",
            "LIGHTBLUE_A700",
        ],
        [
            "CYAN",
            "CYAN_50",
            "CYAN_100",
            "CYAN_200",
            "CYAN_300",
            "CYAN_400",
            "CYAN_500",
            "CYAN_600",
            "CYAN_700",
            "CYAN_800",
            "CYAN_900",
            "CYAN_A100",
            "CYAN_A200",
            "CYAN_A400",
            "CYAN_A700",
        ],
        [
            "TEAL",
            "TEAL_50",
            "TEAL_100",
            "TEAL_200",
            "TEAL_300",
            "TEAL_400",
            "TEAL_500",
            "TEAL_600",
            "TEAL_700",
            "TEAL_800",
            "TEAL_900",
            "TEAL_A100",
            "TEAL_A200",
            "TEAL_A400",
            "TEAL_A700",
        ],
        [
            "GREEN",
            "GREEN_50",
            "GREEN_100",
            "GREEN_200",
            "GREEN_300",
            "GREEN_400",
            "GREEN_500",
            "GREEN_600",
            "GREEN_700",
            "GREEN_800",
            "GREEN_900",
            "GREEN_A100",
            "GREEN_A200",
            "GREEN_A400",
            "GREEN_A700",
        ],
        [
            "LIGHTGREEN",
            "LIGHTGREEN_50",
            "LIGHTGREEN_100",
            "LIGHTGREEN_200",
            "LIGHTGREEN_300",
            "LIGHTGREEN_400",
            "LIGHTGREEN_500",
            "LIGHTGREEN_600",
            "LIGHTGREEN_700",
            "LIGHTGREEN_800",
            "LIGHTGREEN_900",
            "LIGHTGREEN_A100",
            "LIGHTGREEN_A200",
            "LIGHTGREEN_A400",
            "LIGHTGREEN_A700",
        ],
        [
            "LIME",
            "LIME_50",
            "LIME_100",
            "LIME_200",
            "LIME_300",
            "LIME_400",
            "LIME_500",
            "LIME_600",
            "LIME_700",
            "LIME_800",
            "LIME_900",
            "LIME_A100",
            "LIME_A200",
            "LIME_A400",
            "LIME_A700",
        ],
        [
            "YELLOW",
            "YELLOW_50",
            "YELLOW_100",
            "YELLOW_200",
            "YELLOW_300",
            "YELLOW_400",
            "YELLOW_500",
            "YELLOW_600",
            "YELLOW_700",
            "YELLOW_800",
            "YELLOW_900",
            "YELLOW_A100",
            "YELLOW_A200",
            "YELLOW_A400",
            "YELLOW_A700",
        ],
        [
            "AMBER",
            "AMBER_50",
            "AMBER_100",
            "AMBER_200",
            "AMBER_300",
            "AMBER_400",
            "AMBER_500",
            "AMBER_600",
            "AMBER_700",
            "AMBER_800",
            "AMBER_900",
            "AMBER_A100",
            "AMBER_A200",
            "AMBER_A400",
            "AMBER_A700",
        ],
        [
            "ORANGE",
            "ORANGE_50",
            "ORANGE_100",
            "ORANGE_200",
            "ORANGE_300",
            "ORANGE_400",
            "ORANGE_500",
            "ORANGE_600",
            "ORANGE_700",
            "ORANGE_800",
            "ORANGE_900",
            "ORANGE_A100",
            "ORANGE_A200",
            "ORANGE_A400",
            "ORANGE_A700",
        ],
        [
            "DEEPORANGE",
            "DEEPORANGE_50",
            "DEEPORANGE_100",
            "DEEPORANGE_200",
            "DEEPORANGE_300",
            "DEEPORANGE_400",
            "DEEPORANGE_500",
            "DEEPORANGE_600",
            "DEEPORANGE_700",
            "DEEPORANGE_800",
            "DEEPORANGE_900",
            "DEEPORANGE_A100",
            "DEEPORANGE_A200",
            "DEEPORANGE_A400",
            "DEEPORANGE_A700",
        ],
        [
            "BROWN",
            "BROWN_50",
            "BROWN_100",
            "BROWN_200",
            "BROWN_300",
            "BROWN_400",
            "BROWN_500",
            "BROWN_600",
            "BROWN_700",
            "BROWN_800",
            "BROWN_900",
            "BROWN_A100",
            "BROWN_A200",
            "BROWN_A400",
            "BROWN_A700",
        ],
        [
            "GREY",
            "GREY_50",
            "GREY_100",
            "GREY_200",
            "GREY_300",
            "GREY_400",
            "GREY_500",
            "GREY_600",
            "GREY_700",
            "GREY_800",
            "GREY_900",
            "GREY_A100",
            "GREY_A200",
            "GREY_A400",
            "GREY_A700",
        ],
        [
            "BLUEGREY",
            "BLUEGREY_50",
            "BLUEGREY_100",
            "BLUEGREY_200",
            "BLUEGREY_300",
            "BLUEGREY_400",
            "BLUEGREY_500",
            "BLUEGREY_600",
            "BLUEGREY_700",
            "BLUEGREY_800",
            "BLUEGREY_900",
            "BLUEGREY_A100",
            "BLUEGREY_A200",
            "BLUEGREY_A400",
            "BLUEGREY_A700",
        ],
    ];

    use plotters::style::text_anchor::*;
    let centered = Pos::new(HPos::Center, VPos::Top);
    let label_style = TextStyle::from(("monospace", 14.0).into_font()).pos(centered);

    for (col, colors) in colors.iter().enumerate() {
        chart.draw_series(colors.iter().zip(color_names[col].iter()).enumerate().map(
            |(row, (color, &name))| {
                let row = row as f32;
                let col = col as f32;
                EmptyElement::at((col, row))
                    + Circle::new((0, 0), 15, color.filled())
                    + Text::new(name, (0, 16), &label_style)
            },
        ))?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}
