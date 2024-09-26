pub const ter_u20b: ::embedded_graphics::mono_font::MonoFont = ::embedded_graphics::mono_font::MonoFont {
    image: ::embedded_graphics::image::ImageRaw::new(
        include_bytes!("ter_u20b.data"),
        32u32,
    ),
    glyph_mapping: &::embedded_graphics::mono_font::mapping::StrGlyphMapping::new(
        "\0!~",
        30usize,
    ),
    character_size: ::embedded_graphics::geometry::Size::new(2u32, 13u32),
    character_spacing: 0u32,
    baseline: 15u32,
    underline: ::embedded_graphics::mono_font::DecorationDimensions::new(17u32, 1u32),
    strikethrough: ::embedded_graphics::mono_font::DecorationDimensions::new(10u32, 1u32),
};
