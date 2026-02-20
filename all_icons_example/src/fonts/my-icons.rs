// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// a1fab13c939f3b3e59f3c13251d402b06fc3c81c95c81e77f9047950119f32bc
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("a_arrow_down", "\u{E585}"),
    ("a_arrow_up", "\u{E586}"),
    ("a_large_small", "\u{E587}"),
    ("accessibility", "\u{E297}"),
    ("activity", "\u{E038}"),
    ("air_vent", "\u{E34D}"),
    ("airplay", "\u{E039}"),
    ("alarm_clock", "\u{E03A}"),
    ("alarm_clock_check", "\u{E1EC}"),
    ("alarm_clock_minus", "\u{E1ED}"),
    ("alarm_clock_off", "\u{E23B}"),
    ("alarm_clock_plus", "\u{E1EE}"),
    ("alarm_smoke", "\u{E57B}"),
    ("album", "\u{E03B}"),
    ("align_center_horizontal", "\u{E26C}"),
    ("align_center_vertical", "\u{E26D}"),
    ("align_end_horizontal", "\u{E26E}"),
    ("align_end_vertical", "\u{E26F}"),
    ("align_horizontal_distribute_center", "\u{E03C}"),
    ("align_horizontal_distribute_end", "\u{E03D}"),
    ("align_horizontal_distribute_start", "\u{E03E}"),
    ("align_horizontal_justify_center", "\u{E272}"),
    ("align_horizontal_justify_end", "\u{E273}"),
    ("align_horizontal_justify_start", "\u{E274}"),
    ("align_horizontal_space_around", "\u{E275}"),
    ("align_horizontal_space_between", "\u{E276}"),
    ("align_start_horizontal", "\u{E270}"),
    ("align_start_vertical", "\u{E271}"),
    ("align_vertical_distribute_center", "\u{E27E}"),
    ("align_vertical_distribute_end", "\u{E27F}"),
    ("align_vertical_distribute_start", "\u{E280}"),
    ("align_vertical_justify_center", "\u{E277}"),
    ("align_vertical_justify_end", "\u{E278}"),
    ("align_vertical_justify_start", "\u{E279}"),
    ("align_vertical_space_around", "\u{E27A}"),
    ("align_vertical_space_between", "\u{E27B}"),
    ("ambulance", "\u{E5BB}"),
    ("ampersand", "\u{E49C}"),
    ("ampersands", "\u{E49D}"),
    ("amphora", "\u{E61B}"),
    ("anchor", "\u{E03F}"),
    ("angry", "\u{E2FC}"),
    ("annoyed", "\u{E2FD}"),
    ("antenna", "\u{E4E2}"),
    ("anvil", "\u{E580}"),
    ("aperture", "\u{E040}"),
    ("app_window", "\u{E426}"),
    ("app_window_mac", "\u{E5D2}"),
    ("apple", "\u{E34E}"),
    ("archive", "\u{E041}"),
    ("archive_restore", "\u{E2CD}"),
    ("archive_x", "\u{E50C}"),
    ("armchair", "\u{E2C0}"),
    ("arrow_big_down", "\u{E1E1}"),
    ("arrow_big_down_dash", "\u{E41D}"),
    ("arrow_big_left", "\u{E1E2}"),
    ("arrow_big_left_dash", "\u{E41E}"),
    ("arrow_big_right", "\u{E1E3}"),
    ("arrow_big_right_dash", "\u{E41F}"),
    ("arrow_big_up", "\u{E1E4}"),
    ("arrow_big_up_dash", "\u{E420}"),
    ("arrow_down", "\u{E042}"),
    ("arrow_down_0_1", "\u{E413}"),
    ("arrow_down_1_0", "\u{E414}"),
    ("arrow_down_a_z", "\u{E415}"),
    ("arrow_down_from_line", "\u{E454}"),
    ("arrow_down_left", "\u{E043}"),
    ("arrow_down_narrow_wide", "\u{E044}"),
    ("arrow_down_right", "\u{E045}"),
    ("arrow_down_to_dot", "\u{E44D}"),
    ("arrow_down_to_line", "\u{E455}"),
    ("arrow_down_up", "\u{E046}"),
    ("arrow_down_wide_narrow", "\u{E047}"),
    ("arrow_down_z_a", "\u{E416}"),
    ("arrow_left", "\u{E048}"),
    ("arrow_left_from_line", "\u{E456}"),
    ("arrow_left_right", "\u{E24A}"),
    ("arrow_left_to_line", "\u{E457}"),
    ("arrow_right", "\u{E049}"),
    ("arrow_right_from_line", "\u{E458}"),
    ("arrow_right_left", "\u{E417}"),
    ("arrow_right_to_line", "\u{E459}"),
    ("arrow_up", "\u{E04A}"),
    ("arrow_up_0_1", "\u{E418}"),
    ("arrow_up_1_0", "\u{E419}"),
    ("arrow_up_a_z", "\u{E41A}"),
    ("arrow_up_down", "\u{E37D}"),
    ("arrow_up_from_dot", "\u{E44E}"),
    ("arrow_up_from_line", "\u{E45A}"),
    ("arrow_up_left", "\u{E04B}"),
    ("arrow_up_narrow_wide", "\u{E04C}"),
    ("arrow_up_right", "\u{E04D}"),
    ("arrow_up_to_line", "\u{E45B}"),
    ("arrow_up_wide_narrow", "\u{E41B}"),
    ("arrow_up_z_a", "\u{E41C}"),
    ("arrows_up_from_line", "\u{E4D4}"),
    ("asterisk", "\u{E1EF}"),
    ("at_sign", "\u{E04E}"),
    ("atom", "\u{E3D7}"),
    ("audio_lines", "\u{E55A}"),
    ("audio_waveform", "\u{E55B}"),
    ("award", "\u{E04F}"),
    ("axe", "\u{E050}"),
    ("axis_3d", "\u{E2FE}"),
    ("baby", "\u{E2CE}"),
    ("backpack", "\u{E2C8}"),
    ("badge", "\u{E474}"),
    ("badge_alert", "\u{E475}"),
    ("badge_cent", "\u{E50F}"),
    ("badge_check", "\u{E241}"),
    ("badge_dollar_sign", "\u{E476}"),
    ("badge_euro", "\u{E510}"),
    ("badge_indian_rupee", "\u{E511}"),
    ("badge_info", "\u{E477}"),
    ("badge_japanese_yen", "\u{E512}"),
    ("badge_minus", "\u{E478}"),
    ("badge_percent", "\u{E479}"),
    ("badge_plus", "\u{E47A}"),
    ("badge_pound_sterling", "\u{E513}"),
    ("badge_question_mark", "\u{E47B}"),
    ("badge_russian_ruble", "\u{E514}"),
    ("badge_swiss_franc", "\u{E515}"),
    ("badge_turkish_lira", "\u{E67E}"),
    ("badge_x", "\u{E47C}"),
    ("baggage_claim", "\u{E2C9}"),
    ("balloon", "\u{E6AF}"),
    ("ban", "\u{E051}"),
    ("banana", "\u{E34F}"),
    ("bandage", "\u{E61D}"),
    ("banknote", "\u{E052}"),
    ("banknote_arrow_down", "\u{E64C}"),
    ("banknote_arrow_up", "\u{E64D}"),
    ("banknote_x", "\u{E64E}"),
    ("barcode", "\u{E533}"),
    ("barrel", "\u{E675}"),
    ("baseline", "\u{E285}"),
    ("bath", "\u{E2AB}"),
    ("battery", "\u{E053}"),
    ("battery_charging", "\u{E054}"),
    ("battery_full", "\u{E055}"),
    ("battery_low", "\u{E056}"),
    ("battery_medium", "\u{E057}"),
    ("battery_plus", "\u{E63E}"),
    ("battery_warning", "\u{E3AC}"),
    ("beaker", "\u{E058}"),
    ("bean", "\u{E38F}"),
    ("bean_off", "\u{E390}"),
    ("bed", "\u{E2C1}"),
    ("bed_double", "\u{E2C2}"),
    ("bed_single", "\u{E2C3}"),
    ("beef", "\u{E3A5}"),
    ("beer", "\u{E2CF}"),
    ("beer_off", "\u{E5D9}"),
    ("bell", "\u{E059}"),
    ("bell_dot", "\u{E42B}"),
    ("bell_electric", "\u{E57C}"),
    ("bell_minus", "\u{E1F0}"),
    ("bell_off", "\u{E05A}"),
    ("bell_plus", "\u{E1F1}"),
    ("bell_ring", "\u{E224}"),
    ("between_horizontal_end", "\u{E591}"),
    ("between_horizontal_start", "\u{E592}"),
    ("between_vertical_end", "\u{E593}"),
    ("between_vertical_start", "\u{E594}"),
    ("biceps_flexed", "\u{E5EB}"),
    ("bike", "\u{E1D2}"),
    ("binary", "\u{E1F2}"),
    ("binoculars", "\u{E621}"),
    ("biohazard", "\u{E441}"),
    ("bird", "\u{E3C5}"),
    ("birdhouse", "\u{E69A}"),
    ("bitcoin", "\u{E05B}"),
    ("blend", "\u{E59C}"),
    ("blinds", "\u{E3C0}"),
    ("blocks", "\u{E4FA}"),
    ("bluetooth", "\u{E05C}"),
    ("bluetooth_connected", "\u{E1B8}"),
    ("bluetooth_off", "\u{E1B9}"),
    ("bluetooth_searching", "\u{E1BA}"),
    ("bold", "\u{E05D}"),
    ("bolt", "\u{E58C}"),
    ("bomb", "\u{E2FF}"),
    ("bone", "\u{E358}"),
    ("book", "\u{E05E}"),
    ("book_a", "\u{E544}"),
    ("book_alert", "\u{E672}"),
    ("book_audio", "\u{E545}"),
    ("book_check", "\u{E546}"),
    ("book_copy", "\u{E3EC}"),
    ("book_dashed", "\u{E3ED}"),
    ("book_down", "\u{E3EE}"),
    ("book_headphones", "\u{E547}"),
    ("book_heart", "\u{E548}"),
    ("book_image", "\u{E549}"),
    ("book_key", "\u{E3EF}"),
    ("book_lock", "\u{E3F0}"),
    ("book_marked", "\u{E3F1}"),
    ("book_minus", "\u{E3F2}"),
    ("book_open", "\u{E05F}"),
    ("book_open_check", "\u{E381}"),
    ("book_open_text", "\u{E54A}"),
    ("book_plus", "\u{E3F3}"),
    ("book_search", "\u{E6AB}"),
    ("book_text", "\u{E54B}"),
    ("book_type", "\u{E54C}"),
    ("book_up", "\u{E3F4}"),
    ("book_up_2", "\u{E4A6}"),
    ("book_user", "\u{E54D}"),
    ("book_x", "\u{E3F5}"),
    ("bookmark", "\u{E060}"),
    ("bookmark_check", "\u{E51F}"),
    ("bookmark_minus", "\u{E23C}"),
    ("bookmark_plus", "\u{E23D}"),
    ("bookmark_x", "\u{E520}"),
    ("boom_box", "\u{E4EE}"),
    ("bot", "\u{E1BB}"),
    ("bot_message_square", "\u{E5CE}"),
    ("bot_off", "\u{E5E0}"),
    ("bottle_wine", "\u{E67B}"),
    ("bow_arrow", "\u{E65E}"),
    ("box_icon", "\u{E061}"),
    ("boxes", "\u{E2D0}"),
    ("braces", "\u{E36A}"),
    ("brackets", "\u{E443}"),
    ("brain", "\u{E3C6}"),
    ("brain_circuit", "\u{E3C7}"),
    ("brain_cog", "\u{E3C8}"),
    ("brick_wall", "\u{E581}"),
    ("brick_wall_fire", "\u{E653}"),
    ("brick_wall_shield", "\u{E690}"),
    ("briefcase", "\u{E062}"),
    ("briefcase_business", "\u{E5D5}"),
    ("briefcase_conveyor_belt", "\u{E62B}"),
    ("briefcase_medical", "\u{E5D6}"),
    ("bring_to_front", "\u{E4EF}"),
    ("brush", "\u{E1D3}"),
    ("brush_cleaning", "\u{E666}"),
    ("bubbles", "\u{E654}"),
    ("bug", "\u{E20C}"),
    ("bug_off", "\u{E50D}"),
    ("bug_play", "\u{E50E}"),
    ("building", "\u{E1CC}"),
    ("building_2", "\u{E290}"),
    ("bus", "\u{E1D4}"),
    ("bus_front", "\u{E4FB}"),
    ("cable", "\u{E4E3}"),
    ("cable_car", "\u{E4FC}"),
    ("cake", "\u{E344}"),
    ("cake_slice", "\u{E4B9}"),
    ("calculator", "\u{E1BC}"),
    ("calendar", "\u{E063}"),
    ("calendar_1", "\u{E630}"),
    ("calendar_arrow_down", "\u{E5FE}"),
    ("calendar_arrow_up", "\u{E5FF}"),
    ("calendar_check", "\u{E2B7}"),
    ("calendar_check_2", "\u{E2B8}"),
    ("calendar_clock", "\u{E304}"),
    ("calendar_cog", "\u{E5ED}"),
    ("calendar_days", "\u{E2B9}"),
    ("calendar_fold", "\u{E5B4}"),
    ("calendar_heart", "\u{E305}"),
    ("calendar_minus", "\u{E2BA}"),
    ("calendar_minus_2", "\u{E5B5}"),
    ("calendar_off", "\u{E2BB}"),
    ("calendar_plus", "\u{E2BC}"),
    ("calendar_plus_2", "\u{E5B6}"),
    ("calendar_range", "\u{E2BD}"),
    ("calendar_search", "\u{E306}"),
    ("calendar_sync", "\u{E636}"),
    ("calendar_x", "\u{E2BE}"),
    ("calendar_x_2", "\u{E2BF}"),
    ("calendars", "\u{E6A7}"),
    ("camera", "\u{E064}"),
    ("camera_off", "\u{E065}"),
    ("candy", "\u{E391}"),
    ("candy_cane", "\u{E4BA}"),
    ("candy_off", "\u{E392}"),
    ("cannabis", "\u{E5D4}"),
    ("cannabis_off", "\u{E6B7}"),
    ("captions", "\u{E3A4}"),
    ("captions_off", "\u{E5C1}"),
    ("car", "\u{E1D5}"),
    ("car_front", "\u{E4FD}"),
    ("car_taxi_front", "\u{E4FE}"),
    ("caravan", "\u{E539}"),
    ("card_sim", "\u{E671}"),
    ("carrot", "\u{E25A}"),
    ("case_lower", "\u{E3D8}"),
    ("case_sensitive", "\u{E3D9}"),
    ("case_upper", "\u{E3DA}"),
    ("cassette_tape", "\u{E4CA}"),
    ("cast", "\u{E066}"),
    ("castle", "\u{E3E0}"),
    ("cat", "\u{E38C}"),
    ("cctv", "\u{E57D}"),
    ("chart_area", "\u{E4D3}"),
    ("chart_bar", "\u{E2A2}"),
    ("chart_bar_big", "\u{E4A7}"),
    ("chart_bar_decreasing", "\u{E607}"),
    ("chart_bar_increasing", "\u{E608}"),
    ("chart_bar_stacked", "\u{E609}"),
    ("chart_candlestick", "\u{E4A8}"),
    ("chart_column", "\u{E2A3}"),
    ("chart_column_big", "\u{E4A9}"),
    ("chart_column_decreasing", "\u{E067}"),
    ("chart_column_increasing", "\u{E2A4}"),
    ("chart_column_stacked", "\u{E60A}"),
    ("chart_gantt", "\u{E624}"),
    ("chart_line", "\u{E2A5}"),
    ("chart_network", "\u{E60B}"),
    ("chart_no_axes_column", "\u{E068}"),
    ("chart_no_axes_column_decreasing", "\u{E069}"),
    ("chart_no_axes_column_increasing", "\u{E06A}"),
    ("chart_no_axes_combined", "\u{E60C}"),
    ("chart_no_axes_gantt", "\u{E4C4}"),
    ("chart_pie", "\u{E06B}"),
    ("chart_scatter", "\u{E48A}"),
    ("chart_spline", "\u{E60D}"),
    ("check", "\u{E06C}"),
    ("check_check", "\u{E38E}"),
    ("check_line", "\u{E66B}"),
    ("chef_hat", "\u{E2AC}"),
    ("cherry", "\u{E350}"),
    ("chess_bishop", "\u{E6A0}"),
    ("chess_king", "\u{E6A1}"),
    ("chess_knight", "\u{E6A2}"),
    ("chess_pawn", "\u{E6A3}"),
    ("chess_queen", "\u{E6A4}"),
    ("chess_rook", "\u{E6A5}"),
    ("chevron_down", "\u{E06D}"),
    ("chevron_first", "\u{E243}"),
    ("chevron_last", "\u{E244}"),
    ("chevron_left", "\u{E06E}"),
    ("chevron_right", "\u{E06F}"),
    ("chevron_up", "\u{E070}"),
    ("chevrons_down", "\u{E071}"),
    ("chevrons_down_up", "\u{E228}"),
    ("chevrons_left", "\u{E072}"),
    ("chevrons_left_right", "\u{E293}"),
    ("chevrons_left_right_ellipsis", "\u{E61F}"),
    ("chevrons_right", "\u{E073}"),
    ("chevrons_right_left", "\u{E294}"),
    ("chevrons_up", "\u{E074}"),
    ("chevrons_up_down", "\u{E211}"),
    ("chromium", "\u{E075}"),
    ("church", "\u{E3E1}"),
    ("cigarette", "\u{E2C6}"),
    ("cigarette_off", "\u{E2C7}"),
    ("circle", "\u{E076}"),
    ("circle_alert", "\u{E077}"),
    ("circle_arrow_down", "\u{E078}"),
    ("circle_arrow_left", "\u{E079}"),
    ("circle_arrow_out_down_left", "\u{E3F7}"),
    ("circle_arrow_out_down_right", "\u{E3F8}"),
    ("circle_arrow_out_up_left", "\u{E3F9}"),
    ("circle_arrow_out_up_right", "\u{E3FA}"),
    ("circle_arrow_right", "\u{E07A}"),
    ("circle_arrow_up", "\u{E07B}"),
    ("circle_check", "\u{E226}"),
    ("circle_check_big", "\u{E07C}"),
    ("circle_chevron_down", "\u{E4DD}"),
    ("circle_chevron_left", "\u{E4DE}"),
    ("circle_chevron_right", "\u{E4DF}"),
    ("circle_chevron_up", "\u{E4E0}"),
    ("circle_dashed", "\u{E4B0}"),
    ("circle_divide", "\u{E07D}"),
    ("circle_dollar_sign", "\u{E47D}"),
    ("circle_dot", "\u{E345}"),
    ("circle_dot_dashed", "\u{E4B1}"),
    ("circle_ellipsis", "\u{E346}"),
    ("circle_equal", "\u{E400}"),
    ("circle_fading_arrow_up", "\u{E618}"),
    ("circle_fading_plus", "\u{E5BC}"),
    ("circle_gauge", "\u{E4E1}"),
    ("circle_minus", "\u{E07E}"),
    ("circle_off", "\u{E401}"),
    ("circle_parking", "\u{E3C9}"),
    ("circle_parking_off", "\u{E3CA}"),
    ("circle_pause", "\u{E07F}"),
    ("circle_percent", "\u{E51A}"),
    ("circle_pile", "\u{E6B0}"),
    ("circle_play", "\u{E080}"),
    ("circle_plus", "\u{E081}"),
    ("circle_pound_sterling", "\u{E66D}"),
    ("circle_power", "\u{E550}"),
    ("circle_question_mark", "\u{E082}"),
    ("circle_slash", "\u{E402}"),
    ("circle_slash_2", "\u{E213}"),
    ("circle_small", "\u{E640}"),
    ("circle_star", "\u{E68D}"),
    ("circle_stop", "\u{E083}"),
    ("circle_user", "\u{E461}"),
    ("circle_user_round", "\u{E462}"),
    ("circle_x", "\u{E084}"),
    ("circuit_board", "\u{E403}"),
    ("citrus", "\u{E375}"),
    ("clapperboard", "\u{E29B}"),
    ("clipboard", "\u{E085}"),
    ("clipboard_check", "\u{E219}"),
    ("clipboard_clock", "\u{E688}"),
    ("clipboard_copy", "\u{E225}"),
    ("clipboard_list", "\u{E086}"),
    ("clipboard_minus", "\u{E5BE}"),
    ("clipboard_paste", "\u{E3E8}"),
    ("clipboard_pen", "\u{E307}"),
    ("clipboard_pen_line", "\u{E308}"),
    ("clipboard_plus", "\u{E5BF}"),
    ("clipboard_type", "\u{E309}"),
    ("clipboard_x", "\u{E222}"),
    ("clock", "\u{E087}"),
    ("clock_1", "\u{E24B}"),
    ("clock_10", "\u{E24C}"),
    ("clock_11", "\u{E24D}"),
    ("clock_12", "\u{E24E}"),
    ("clock_2", "\u{E24F}"),
    ("clock_3", "\u{E250}"),
    ("clock_4", "\u{E251}"),
    ("clock_5", "\u{E252}"),
    ("clock_6", "\u{E253}"),
    ("clock_7", "\u{E254}"),
    ("clock_8", "\u{E255}"),
    ("clock_9", "\u{E256}"),
    ("clock_alert", "\u{E62A}"),
    ("clock_arrow_down", "\u{E600}"),
    ("clock_arrow_up", "\u{E601}"),
    ("clock_check", "\u{E69E}"),
    ("clock_fading", "\u{E64A}"),
    ("clock_plus", "\u{E667}"),
    ("closed_caption", "\u{E68A}"),
    ("cloud", "\u{E088}"),
    ("cloud_alert", "\u{E633}"),
    ("cloud_backup", "\u{E6B1}"),
    ("cloud_check", "\u{E66E}"),
    ("cloud_cog", "\u{E30A}"),
    ("cloud_download", "\u{E089}"),
    ("cloud_drizzle", "\u{E08A}"),
    ("cloud_fog", "\u{E214}"),
    ("cloud_hail", "\u{E08B}"),
    ("cloud_lightning", "\u{E08C}"),
    ("cloud_moon", "\u{E215}"),
    ("cloud_moon_rain", "\u{E2FA}"),
    ("cloud_off", "\u{E08D}"),
    ("cloud_rain", "\u{E08E}"),
    ("cloud_rain_wind", "\u{E08F}"),
    ("cloud_snow", "\u{E090}"),
    ("cloud_sun", "\u{E216}"),
    ("cloud_sun_rain", "\u{E2FB}"),
    ("cloud_sync", "\u{E6B2}"),
    ("cloud_upload", "\u{E091}"),
    ("cloudy", "\u{E217}"),
    ("clover", "\u{E092}"),
    ("club", "\u{E496}"),
    ("code", "\u{E093}"),
    ("code_xml", "\u{E206}"),
    ("codepen", "\u{E094}"),
    ("codesandbox", "\u{E095}"),
    ("coffee", "\u{E096}"),
    ("cog", "\u{E30B}"),
    ("coins", "\u{E097}"),
    ("columns_2", "\u{E098}"),
    ("columns_3", "\u{E099}"),
    ("columns_3_cog", "\u{E661}"),
    ("columns_4", "\u{E589}"),
    ("combine", "\u{E44C}"),
    ("command", "\u{E09A}"),
    ("compass", "\u{E09B}"),
    ("component", "\u{E2AD}"),
    ("computer", "\u{E4E4}"),
    ("concierge_bell", "\u{E378}"),
    ("cone", "\u{E523}"),
    ("construction", "\u{E3B4}"),
    ("contact", "\u{E09C}"),
    ("contact_round", "\u{E463}"),
    ("container", "\u{E4D5}"),
    ("contrast", "\u{E09D}"),
    ("cookie", "\u{E26B}"),
    ("cooking_pot", "\u{E584}"),
    ("copy", "\u{E09E}"),
    ("copy_check", "\u{E3FB}"),
    ("copy_minus", "\u{E3FC}"),
    ("copy_plus", "\u{E3FD}"),
    ("copy_slash", "\u{E3FE}"),
    ("copy_x", "\u{E3FF}"),
    ("copyleft", "\u{E09F}"),
    ("copyright", "\u{E0A0}"),
    ("corner_down_left", "\u{E0A1}"),
    ("corner_down_right", "\u{E0A2}"),
    ("corner_left_down", "\u{E0A3}"),
    ("corner_left_up", "\u{E0A4}"),
    ("corner_right_down", "\u{E0A5}"),
    ("corner_right_up", "\u{E0A6}"),
    ("corner_up_left", "\u{E0A7}"),
    ("corner_up_right", "\u{E0A8}"),
    ("cpu", "\u{E0A9}"),
    ("creative_commons", "\u{E3B2}"),
    ("credit_card", "\u{E0AA}"),
    ("croissant", "\u{E2AE}"),
    ("crop", "\u{E0AB}"),
    ("cross", "\u{E1E5}"),
    ("crosshair", "\u{E0AC}"),
    ("crown", "\u{E1D6}"),
    ("cuboid", "\u{E524}"),
    ("cup_soda", "\u{E2D1}"),
    ("currency", "\u{E230}"),
    ("cylinder", "\u{E525}"),
    ("dam", "\u{E606}"),
    ("database", "\u{E0AD}"),
    ("database_backup", "\u{E3AB}"),
    ("database_search", "\u{E6BC}"),
    ("database_zap", "\u{E50B}"),
    ("decimals_arrow_left", "\u{E65C}"),
    ("decimals_arrow_right", "\u{E65D}"),
    ("delete", "\u{E0AE}"),
    ("dessert", "\u{E4BB}"),
    ("diameter", "\u{E526}"),
    ("diamond", "\u{E2D2}"),
    ("diamond_minus", "\u{E5E1}"),
    ("diamond_percent", "\u{E51B}"),
    ("diamond_plus", "\u{E5E2}"),
    ("dice_1", "\u{E287}"),
    ("dice_2", "\u{E288}"),
    ("dice_3", "\u{E289}"),
    ("dice_4", "\u{E28A}"),
    ("dice_5", "\u{E28B}"),
    ("dice_6", "\u{E28C}"),
    ("dices", "\u{E2C5}"),
    ("diff", "\u{E30C}"),
    ("disc", "\u{E0AF}"),
    ("disc_2", "\u{E3F6}"),
    ("disc_3", "\u{E494}"),
    ("disc_album", "\u{E55C}"),
    ("divide", "\u{E0B0}"),
    ("dna", "\u{E393}"),
    ("dna_off", "\u{E394}"),
    ("dock", "\u{E5D3}"),
    ("dog", "\u{E38D}"),
    ("dollar_sign", "\u{E0B1}"),
    ("donut", "\u{E4BC}"),
    ("door_closed", "\u{E3D5}"),
    ("door_closed_locked", "\u{E664}"),
    ("door_open", "\u{E3D6}"),
    ("dot", "\u{E44F}"),
    ("download", "\u{E0B2}"),
    ("drafting_compass", "\u{E527}"),
    ("drama", "\u{E521}"),
    ("dribbble", "\u{E0B3}"),
    ("drill", "\u{E58D}"),
    ("drone", "\u{E676}"),
    ("droplet", "\u{E0B4}"),
    ("droplet_off", "\u{E638}"),
    ("droplets", "\u{E0B5}"),
    ("drum", "\u{E55D}"),
    ("drumstick", "\u{E25B}"),
    ("dumbbell", "\u{E3A1}"),
    ("ear", "\u{E382}"),
    ("ear_off", "\u{E383}"),
    ("earth", "\u{E1F3}"),
    ("earth_lock", "\u{E5CC}"),
    ("eclipse", "\u{E59D}"),
    ("egg", "\u{E25D}"),
    ("egg_fried", "\u{E351}"),
    ("egg_off", "\u{E395}"),
    ("ellipsis", "\u{E0B6}"),
    ("ellipsis_vertical", "\u{E0B7}"),
    ("equal", "\u{E1BD}"),
    ("equal_approximately", "\u{E634}"),
    ("equal_not", "\u{E1BE}"),
    ("eraser", "\u{E28F}"),
    ("ethernet_port", "\u{E620}"),
    ("euro", "\u{E0B8}"),
    ("ev_charger", "\u{E697}"),
    ("expand", "\u{E21A}"),
    ("external_link", "\u{E0B9}"),
    ("eye", "\u{E0BA}"),
    ("eye_closed", "\u{E62E}"),
    ("eye_off", "\u{E0BB}"),
    ("facebook", "\u{E0BC}"),
    ("factory", "\u{E29F}"),
    ("fan", "\u{E379}"),
    ("fast_forward", "\u{E0BD}"),
    ("feather", "\u{E0BE}"),
    ("fence", "\u{E582}"),
    ("ferris_wheel", "\u{E47F}"),
    ("figma", "\u{E0BF}"),
    ("file", "\u{E0C0}"),
    ("file_archive", "\u{E30D}"),
    ("file_axis_3d", "\u{E30E}"),
    ("file_badge", "\u{E30F}"),
    ("file_box", "\u{E310}"),
    ("file_braces", "\u{E36B}"),
    ("file_braces_corner", "\u{E36C}"),
    ("file_chart_column", "\u{E311}"),
    ("file_chart_column_increasing", "\u{E312}"),
    ("file_chart_line", "\u{E313}"),
    ("file_chart_pie", "\u{E314}"),
    ("file_check", "\u{E0C1}"),
    ("file_check_corner", "\u{E0C2}"),
    ("file_clock", "\u{E315}"),
    ("file_code", "\u{E0C3}"),
    ("file_code_corner", "\u{E45E}"),
    ("file_cog", "\u{E316}"),
    ("file_diff", "\u{E317}"),
    ("file_digit", "\u{E0C4}"),
    ("file_down", "\u{E318}"),
    ("file_exclamation_point", "\u{E319}"),
    ("file_headphone", "\u{E31A}"),
    ("file_heart", "\u{E31B}"),
    ("file_image", "\u{E31C}"),
    ("file_input", "\u{E0C5}"),
    ("file_key", "\u{E31D}"),
    ("file_lock", "\u{E31E}"),
    ("file_minus", "\u{E0C6}"),
    ("file_minus_corner", "\u{E0C7}"),
    ("file_music", "\u{E55E}"),
    ("file_output", "\u{E0C8}"),
    ("file_pen", "\u{E31F}"),
    ("file_pen_line", "\u{E320}"),
    ("file_play", "\u{E321}"),
    ("file_plus", "\u{E0C9}"),
    ("file_plus_corner", "\u{E0CA}"),
    ("file_question_mark", "\u{E322}"),
    ("file_scan", "\u{E323}"),
    ("file_search", "\u{E0CB}"),
    ("file_search_corner", "\u{E324}"),
    ("file_signal", "\u{E325}"),
    ("file_sliders", "\u{E5A0}"),
    ("file_spreadsheet", "\u{E326}"),
    ("file_stack", "\u{E4A1}"),
    ("file_symlink", "\u{E327}"),
    ("file_terminal", "\u{E328}"),
    ("file_text", "\u{E0CC}"),
    ("file_type", "\u{E329}"),
    ("file_type_corner", "\u{E36D}"),
    ("file_up", "\u{E32A}"),
    ("file_user", "\u{E62D}"),
    ("file_video_camera", "\u{E32B}"),
    ("file_volume", "\u{E32C}"),
    ("file_x", "\u{E0CD}"),
    ("file_x_corner", "\u{E0CE}"),
    ("files", "\u{E0CF}"),
    ("film", "\u{E0D0}"),
    ("fingerprint_pattern", "\u{E2CB}"),
    ("fire_extinguisher", "\u{E57E}"),
    ("fish", "\u{E3A6}"),
    ("fish_off", "\u{E3B0}"),
    ("fish_symbol", "\u{E4F4}"),
    ("fishing_hook", "\u{E6B6}"),
    ("flag", "\u{E0D1}"),
    ("flag_off", "\u{E292}"),
    ("flag_triangle_left", "\u{E237}"),
    ("flag_triangle_right", "\u{E238}"),
    ("flame", "\u{E0D2}"),
    ("flame_kindling", "\u{E53A}"),
    ("flashlight", "\u{E0D3}"),
    ("flashlight_off", "\u{E0D4}"),
    ("flask_conical", "\u{E0D5}"),
    ("flask_conical_off", "\u{E396}"),
    ("flask_round", "\u{E0D6}"),
    ("flip_horizontal_2", "\u{E35D}"),
    ("flip_vertical_2", "\u{E35E}"),
    ("flower", "\u{E2D3}"),
    ("flower_2", "\u{E2D4}"),
    ("focus", "\u{E29E}"),
    ("fold_horizontal", "\u{E43B}"),
    ("fold_vertical", "\u{E43C}"),
    ("folder", "\u{E0D7}"),
    ("folder_archive", "\u{E32D}"),
    ("folder_check", "\u{E32E}"),
    ("folder_clock", "\u{E32F}"),
    ("folder_closed", "\u{E330}"),
    ("folder_code", "\u{E5FB}"),
    ("folder_cog", "\u{E331}"),
    ("folder_dot", "\u{E4C5}"),
    ("folder_down", "\u{E332}"),
    ("folder_git", "\u{E409}"),
    ("folder_git_2", "\u{E40A}"),
    ("folder_heart", "\u{E333}"),
    ("folder_input", "\u{E334}"),
    ("folder_kanban", "\u{E4C6}"),
    ("folder_key", "\u{E335}"),
    ("folder_lock", "\u{E336}"),
    ("folder_minus", "\u{E0D8}"),
    ("folder_open", "\u{E247}"),
    ("folder_open_dot", "\u{E4C7}"),
    ("folder_output", "\u{E337}"),
    ("folder_pen", "\u{E338}"),
    ("folder_plus", "\u{E0D9}"),
    ("folder_root", "\u{E4C8}"),
    ("folder_search", "\u{E339}"),
    ("folder_search_2", "\u{E33A}"),
    ("folder_symlink", "\u{E33B}"),
    ("folder_sync", "\u{E4C9}"),
    ("folder_tree", "\u{E33C}"),
    ("folder_up", "\u{E33D}"),
    ("folder_x", "\u{E33E}"),
    ("folders", "\u{E33F}"),
    ("footprints", "\u{E3B9}"),
    ("forklift", "\u{E3C1}"),
    ("form", "\u{E6A8}"),
    ("forward", "\u{E229}"),
    ("frame", "\u{E291}"),
    ("framer", "\u{E0DA}"),
    ("frown", "\u{E0DB}"),
    ("fuel", "\u{E2AF}"),
    ("fullscreen", "\u{E534}"),
    ("funnel", "\u{E0DC}"),
    ("funnel_plus", "\u{E0DD}"),
    ("funnel_x", "\u{E3B5}"),
    ("gallery_horizontal", "\u{E4CE}"),
    ("gallery_horizontal_end", "\u{E4CF}"),
    ("gallery_thumbnails", "\u{E4D0}"),
    ("gallery_vertical", "\u{E4D1}"),
    ("gallery_vertical_end", "\u{E4D2}"),
    ("gamepad", "\u{E0DE}"),
    ("gamepad_2", "\u{E0DF}"),
    ("gamepad_directional", "\u{E69B}"),
    ("gauge", "\u{E1BF}"),
    ("gavel", "\u{E0E0}"),
    ("gem", "\u{E242}"),
    ("georgian_lari", "\u{E678}"),
    ("ghost", "\u{E20E}"),
    ("gift", "\u{E0E1}"),
    ("git_branch", "\u{E0E2}"),
    ("git_branch_minus", "\u{E69C}"),
    ("git_branch_plus", "\u{E1F4}"),
    ("git_commit_horizontal", "\u{E0E3}"),
    ("git_commit_vertical", "\u{E552}"),
    ("git_compare", "\u{E359}"),
    ("git_compare_arrows", "\u{E553}"),
    ("git_fork", "\u{E28D}"),
    ("git_graph", "\u{E554}"),
    ("git_merge", "\u{E0E4}"),
    ("git_merge_conflict", "\u{E6C3}"),
    ("git_pull_request", "\u{E0E5}"),
    ("git_pull_request_arrow", "\u{E555}"),
    ("git_pull_request_closed", "\u{E35A}"),
    ("git_pull_request_create", "\u{E556}"),
    ("git_pull_request_create_arrow", "\u{E557}"),
    ("git_pull_request_draft", "\u{E35B}"),
    ("github", "\u{E0E6}"),
    ("gitlab", "\u{E0E7}"),
    ("glass_water", "\u{E2D5}"),
    ("glasses", "\u{E20D}"),
    ("globe", "\u{E0E8}"),
    ("globe_lock", "\u{E5CD}"),
    ("globe_off", "\u{E6C1}"),
    ("globe_x", "\u{E6BA}"),
    ("goal", "\u{E4A5}"),
    ("gpu", "\u{E66A}"),
    ("graduation_cap", "\u{E234}"),
    ("grape", "\u{E352}"),
    ("grid_2x2", "\u{E4FF}"),
    ("grid_2x2_check", "\u{E5E4}"),
    ("grid_2x2_plus", "\u{E628}"),
    ("grid_2x2_x", "\u{E5E5}"),
    ("grid_3x2", "\u{E66F}"),
    ("grid_3x3", "\u{E0E9}"),
    ("grip", "\u{E3B1}"),
    ("grip_horizontal", "\u{E0EA}"),
    ("grip_vertical", "\u{E0EB}"),
    ("group", "\u{E464}"),
    ("guitar", "\u{E55F}"),
    ("ham", "\u{E5D7}"),
    ("hamburger", "\u{E665}"),
    ("hammer", "\u{E0EC}"),
    ("hand", "\u{E1D7}"),
    ("hand_coins", "\u{E5B8}"),
    ("hand_fist", "\u{E68B}"),
    ("hand_grab", "\u{E1E6}"),
    ("hand_heart", "\u{E5B9}"),
    ("hand_helping", "\u{E3B8}"),
    ("hand_metal", "\u{E22C}"),
    ("hand_platter", "\u{E5BA}"),
    ("handbag", "\u{E689}"),
    ("handshake", "\u{E5C0}"),
    ("hard_drive", "\u{E0ED}"),
    ("hard_drive_download", "\u{E4E5}"),
    ("hard_drive_upload", "\u{E4E6}"),
    ("hard_hat", "\u{E0EE}"),
    ("hash", "\u{E0EF}"),
    ("hat_glasses", "\u{E683}"),
    ("haze", "\u{E0F0}"),
    ("hd", "\u{E6B5}"),
    ("hdmi_port", "\u{E4E7}"),
    ("heading", "\u{E384}"),
    ("heading_1", "\u{E385}"),
    ("heading_2", "\u{E386}"),
    ("heading_3", "\u{E387}"),
    ("heading_4", "\u{E388}"),
    ("heading_5", "\u{E389}"),
    ("heading_6", "\u{E38A}"),
    ("headphone_off", "\u{E629}"),
    ("headphones", "\u{E0F1}"),
    ("headset", "\u{E5BD}"),
    ("heart", "\u{E0F2}"),
    ("heart_crack", "\u{E2D6}"),
    ("heart_handshake", "\u{E2D7}"),
    ("heart_minus", "\u{E651}"),
    ("heart_off", "\u{E295}"),
    ("heart_plus", "\u{E652}"),
    ("heart_pulse", "\u{E36E}"),
    ("heater", "\u{E58E}"),
    ("helicopter", "\u{E69D}"),
    ("hexagon", "\u{E0F3}"),
    ("highlighter", "\u{E0F4}"),
    ("history", "\u{E1F5}"),
    ("hop", "\u{E397}"),
    ("hop_off", "\u{E398}"),
    ("hospital", "\u{E5D8}"),
    ("hotel", "\u{E3E2}"),
    ("hourglass", "\u{E296}"),
    ("house", "\u{E0F5}"),
    ("house_heart", "\u{E695}"),
    ("house_plug", "\u{E5F0}"),
    ("house_plus", "\u{E5F1}"),
    ("house_wifi", "\u{E63C}"),
    ("ice_cream_bowl", "\u{E3A7}"),
    ("ice_cream_cone", "\u{E353}"),
    ("id_card", "\u{E617}"),
    ("id_card_lanyard", "\u{E670}"),
    ("image", "\u{E0F6}"),
    ("image_down", "\u{E53C}"),
    ("image_minus", "\u{E1F6}"),
    ("image_off", "\u{E1C0}"),
    ("image_play", "\u{E5DF}"),
    ("image_plus", "\u{E1F7}"),
    ("image_up", "\u{E5CB}"),
    ("image_upscale", "\u{E637}"),
    ("images", "\u{E5C4}"),
    ("import", "\u{E22F}"),
    ("inbox", "\u{E0F7}"),
    ("indian_rupee", "\u{E0F8}"),
    ("infinity", "\u{E1E7}"),
    ("info", "\u{E0F9}"),
    ("inspection_panel", "\u{E583}"),
    ("instagram", "\u{E0FA}"),
    ("italic", "\u{E0FB}"),
    ("iteration_ccw", "\u{E423}"),
    ("iteration_cw", "\u{E424}"),
    ("japanese_yen", "\u{E0FC}"),
    ("joystick", "\u{E355}"),
    ("kanban", "\u{E4DC}"),
    ("kayak", "\u{E68F}"),
    ("key", "\u{E0FD}"),
    ("key_round", "\u{E4A3}"),
    ("key_square", "\u{E4A4}"),
    ("keyboard", "\u{E284}"),
    ("keyboard_music", "\u{E560}"),
    ("keyboard_off", "\u{E5DE}"),
    ("lamp", "\u{E2D8}"),
    ("lamp_ceiling", "\u{E2D9}"),
    ("lamp_desk", "\u{E2DA}"),
    ("lamp_floor", "\u{E2DB}"),
    ("lamp_wall_down", "\u{E2DC}"),
    ("lamp_wall_up", "\u{E2DD}"),
    ("land_plot", "\u{E528}"),
    ("landmark", "\u{E23A}"),
    ("languages", "\u{E0FE}"),
    ("laptop", "\u{E1CD}"),
    ("laptop_minimal", "\u{E1D8}"),
    ("laptop_minimal_check", "\u{E632}"),
    ("lasso", "\u{E1CE}"),
    ("lasso_select", "\u{E1CF}"),
    ("laugh", "\u{E300}"),
    ("layers", "\u{E529}"),
    ("layers_2", "\u{E52A}"),
    ("layers_plus", "\u{E6B3}"),
    ("layout_dashboard", "\u{E1C1}"),
    ("layout_grid", "\u{E0FF}"),
    ("layout_list", "\u{E1D9}"),
    ("layout_panel_left", "\u{E470}"),
    ("layout_panel_top", "\u{E471}"),
    ("layout_template", "\u{E207}"),
    ("leaf", "\u{E2DE}"),
    ("leafy_green", "\u{E46F}"),
    ("lectern", "\u{E5E9}"),
    ("lens_concave", "\u{E6BF}"),
    ("lens_convex", "\u{E6C0}"),
    ("library", "\u{E100}"),
    ("library_big", "\u{E54E}"),
    ("life_buoy", "\u{E101}"),
    ("ligature", "\u{E43A}"),
    ("lightbulb", "\u{E1C2}"),
    ("lightbulb_off", "\u{E208}"),
    ("line_dot_right_horizontal", "\u{E6C2}"),
    ("line_squiggle", "\u{E67A}"),
    ("link", "\u{E102}"),
    ("link_2", "\u{E103}"),
    ("link_2_off", "\u{E104}"),
    ("linkedin", "\u{E105}"),
    ("list", "\u{E106}"),
    ("list_check", "\u{E5FA}"),
    ("list_checks", "\u{E1D0}"),
    ("list_chevrons_down_up", "\u{E694}"),
    ("list_chevrons_up_down", "\u{E696}"),
    ("list_collapse", "\u{E59B}"),
    ("list_end", "\u{E2DF}"),
    ("list_filter", "\u{E460}"),
    ("list_filter_plus", "\u{E639}"),
    ("list_indent_decrease", "\u{E107}"),
    ("list_indent_increase", "\u{E108}"),
    ("list_minus", "\u{E23E}"),
    ("list_music", "\u{E2E0}"),
    ("list_ordered", "\u{E1D1}"),
    ("list_plus", "\u{E23F}"),
    ("list_restart", "\u{E452}"),
    ("list_start", "\u{E2E1}"),
    ("list_todo", "\u{E4C3}"),
    ("list_tree", "\u{E408}"),
    ("list_video", "\u{E2E2}"),
    ("list_x", "\u{E240}"),
    ("loader", "\u{E109}"),
    ("loader_circle", "\u{E10A}"),
    ("loader_pinwheel", "\u{E5E6}"),
    ("locate", "\u{E1DA}"),
    ("locate_fixed", "\u{E1DB}"),
    ("locate_off", "\u{E282}"),
    ("lock", "\u{E10B}"),
    ("lock_keyhole", "\u{E531}"),
    ("lock_keyhole_open", "\u{E532}"),
    ("lock_open", "\u{E10C}"),
    ("log_in", "\u{E10D}"),
    ("log_out", "\u{E10E}"),
    ("logs", "\u{E5F4}"),
    ("lollipop", "\u{E4BD}"),
    ("luggage", "\u{E2CA}"),
    ("magnet", "\u{E2B5}"),
    ("mail", "\u{E10F}"),
    ("mail_check", "\u{E361}"),
    ("mail_minus", "\u{E362}"),
    ("mail_open", "\u{E363}"),
    ("mail_plus", "\u{E364}"),
    ("mail_question_mark", "\u{E365}"),
    ("mail_search", "\u{E366}"),
    ("mail_warning", "\u{E367}"),
    ("mail_x", "\u{E368}"),
    ("mailbox", "\u{E3D4}"),
    ("mails", "\u{E369}"),
    ("map", "\u{E110}"),
    ("map_minus", "\u{E686}"),
    ("map_pin", "\u{E111}"),
    ("map_pin_check", "\u{E60F}"),
    ("map_pin_check_inside", "\u{E610}"),
    ("map_pin_house", "\u{E61C}"),
    ("map_pin_minus", "\u{E611}"),
    ("map_pin_minus_inside", "\u{E612}"),
    ("map_pin_off", "\u{E2A6}"),
    ("map_pin_pen", "\u{E655}"),
    ("map_pin_plus", "\u{E613}"),
    ("map_pin_plus_inside", "\u{E614}"),
    ("map_pin_x", "\u{E615}"),
    ("map_pin_x_inside", "\u{E616}"),
    ("map_pinned", "\u{E53D}"),
    ("map_plus", "\u{E63F}"),
    ("mars", "\u{E641}"),
    ("mars_stroke", "\u{E642}"),
    ("martini", "\u{E2E3}"),
    ("maximize", "\u{E112}"),
    ("maximize_2", "\u{E113}"),
    ("medal", "\u{E36F}"),
    ("megaphone", "\u{E235}"),
    ("megaphone_off", "\u{E370}"),
    ("meh", "\u{E114}"),
    ("memory_stick", "\u{E445}"),
    ("menu", "\u{E115}"),
    ("merge", "\u{E43F}"),
    ("message_circle", "\u{E116}"),
    ("message_circle_check", "\u{E6C8}"),
    ("message_circle_code", "\u{E562}"),
    ("message_circle_dashed", "\u{E563}"),
    ("message_circle_heart", "\u{E564}"),
    ("message_circle_more", "\u{E565}"),
    ("message_circle_off", "\u{E566}"),
    ("message_circle_plus", "\u{E567}"),
    ("message_circle_question_mark", "\u{E568}"),
    ("message_circle_reply", "\u{E569}"),
    ("message_circle_warning", "\u{E56A}"),
    ("message_circle_x", "\u{E56B}"),
    ("message_square", "\u{E117}"),
    ("message_square_check", "\u{E6CC}"),
    ("message_square_code", "\u{E56C}"),
    ("message_square_dashed", "\u{E40B}"),
    ("message_square_diff", "\u{E56D}"),
    ("message_square_dot", "\u{E56E}"),
    ("message_square_heart", "\u{E56F}"),
    ("message_square_lock", "\u{E62C}"),
    ("message_square_more", "\u{E570}"),
    ("message_square_off", "\u{E571}"),
    ("message_square_plus", "\u{E40C}"),
    ("message_square_quote", "\u{E572}"),
    ("message_square_reply", "\u{E573}"),
    ("message_square_share", "\u{E574}"),
    ("message_square_text", "\u{E575}"),
    ("message_square_warning", "\u{E576}"),
    ("message_square_x", "\u{E577}"),
    ("messages_square", "\u{E40D}"),
    ("metronome", "\u{E6CD}"),
    ("mic", "\u{E118}"),
    ("mic_off", "\u{E119}"),
    ("mic_vocal", "\u{E349}"),
    ("microchip", "\u{E61A}"),
    ("microscope", "\u{E2E4}"),
    ("microwave", "\u{E37A}"),
    ("milestone", "\u{E298}"),
    ("milk", "\u{E399}"),
    ("milk_off", "\u{E39A}"),
    ("minimize", "\u{E11A}"),
    ("minimize_2", "\u{E11B}"),
    ("minus", "\u{E11C}"),
    ("mirror_rectangular", "\u{E6C4}"),
    ("mirror_round", "\u{E6C5}"),
    ("monitor", "\u{E11D}"),
    ("monitor_check", "\u{E482}"),
    ("monitor_cloud", "\u{E699}"),
    ("monitor_cog", "\u{E603}"),
    ("monitor_dot", "\u{E483}"),
    ("monitor_down", "\u{E421}"),
    ("monitor_off", "\u{E1DC}"),
    ("monitor_pause", "\u{E484}"),
    ("monitor_play", "\u{E485}"),
    ("monitor_smartphone", "\u{E3A2}"),
    ("monitor_speaker", "\u{E210}"),
    ("monitor_stop", "\u{E486}"),
    ("monitor_up", "\u{E422}"),
    ("monitor_x", "\u{E487}"),
    ("moon", "\u{E11E}"),
    ("moon_star", "\u{E410}"),
    ("motorbike", "\u{E698}"),
    ("mountain", "\u{E231}"),
    ("mountain_snow", "\u{E232}"),
    ("mouse", "\u{E28E}"),
    ("mouse_left", "\u{E6C9}"),
    ("mouse_off", "\u{E5DB}"),
    ("mouse_pointer", "\u{E11F}"),
    ("mouse_pointer_2", "\u{E1C3}"),
    ("mouse_pointer_2_off", "\u{E6A6}"),
    ("mouse_pointer_ban", "\u{E5E7}"),
    ("mouse_pointer_click", "\u{E120}"),
    ("mouse_right", "\u{E6CA}"),
    ("move_3d", "\u{E2E5}"),
    ("move_diagonal", "\u{E1C4}"),
    ("move_diagonal_2", "\u{E1C5}"),
    ("move_down", "\u{E48C}"),
    ("move_down_left", "\u{E48D}"),
    ("move_down_right", "\u{E48E}"),
    ("move_horizontal", "\u{E1C6}"),
    ("move_icon", "\u{E121}"),
    ("move_left", "\u{E48F}"),
    ("move_right", "\u{E490}"),
    ("move_up", "\u{E491}"),
    ("move_up_left", "\u{E492}"),
    ("move_up_right", "\u{E493}"),
    ("move_vertical", "\u{E1C7}"),
    ("music", "\u{E122}"),
    ("music_2", "\u{E34A}"),
    ("music_3", "\u{E34B}"),
    ("music_4", "\u{E34C}"),
    ("navigation", "\u{E123}"),
    ("navigation_2", "\u{E124}"),
    ("navigation_2_off", "\u{E2A7}"),
    ("navigation_off", "\u{E2A8}"),
    ("network", "\u{E125}"),
    ("newspaper", "\u{E348}"),
    ("nfc", "\u{E3C3}"),
    ("non_binary", "\u{E643}"),
    ("notebook", "\u{E595}"),
    ("notebook_pen", "\u{E596}"),
    ("notebook_tabs", "\u{E597}"),
    ("notebook_text", "\u{E598}"),
    ("notepad_text", "\u{E599}"),
    ("notepad_text_dashed", "\u{E59A}"),
    ("nut", "\u{E39B}"),
    ("nut_off", "\u{E39C}"),
    ("octagon", "\u{E126}"),
    ("octagon_alert", "\u{E127}"),
    ("octagon_minus", "\u{E627}"),
    ("octagon_pause", "\u{E21B}"),
    ("octagon_x", "\u{E128}"),
    ("omega", "\u{E619}"),
    ("option", "\u{E1F8}"),
    ("orbit", "\u{E3E7}"),
    ("origami", "\u{E5E3}"),
    ("package", "\u{E129}"),
    ("package_2", "\u{E340}"),
    ("package_check", "\u{E266}"),
    ("package_minus", "\u{E267}"),
    ("package_open", "\u{E2CC}"),
    ("package_plus", "\u{E268}"),
    ("package_search", "\u{E269}"),
    ("package_x", "\u{E26A}"),
    ("paint_bucket", "\u{E2E6}"),
    ("paint_roller", "\u{E59E}"),
    ("paintbrush", "\u{E2E7}"),
    ("paintbrush_vertical", "\u{E2E8}"),
    ("palette", "\u{E1DD}"),
    ("panda", "\u{E668}"),
    ("panel_bottom", "\u{E42C}"),
    ("panel_bottom_close", "\u{E42D}"),
    ("panel_bottom_dashed", "\u{E42E}"),
    ("panel_bottom_open", "\u{E42F}"),
    ("panel_left", "\u{E12A}"),
    ("panel_left_close", "\u{E21C}"),
    ("panel_left_dashed", "\u{E430}"),
    ("panel_left_open", "\u{E21D}"),
    ("panel_left_right_dashed", "\u{E692}"),
    ("panel_right", "\u{E431}"),
    ("panel_right_close", "\u{E432}"),
    ("panel_right_dashed", "\u{E433}"),
    ("panel_right_open", "\u{E434}"),
    ("panel_top", "\u{E435}"),
    ("panel_top_bottom_dashed", "\u{E693}"),
    ("panel_top_close", "\u{E436}"),
    ("panel_top_dashed", "\u{E437}"),
    ("panel_top_open", "\u{E438}"),
    ("panels_left_bottom", "\u{E12B}"),
    ("panels_right_bottom", "\u{E588}"),
    ("panels_top_left", "\u{E12C}"),
    ("paperclip", "\u{E12D}"),
    ("parentheses", "\u{E444}"),
    ("parking_meter", "\u{E500}"),
    ("party_popper", "\u{E343}"),
    ("pause", "\u{E12E}"),
    ("paw_print", "\u{E4F5}"),
    ("pc_case", "\u{E446}"),
    ("pen", "\u{E12F}"),
    ("pen_line", "\u{E130}"),
    ("pen_off", "\u{E5EE}"),
    ("pen_tool", "\u{E131}"),
    ("pencil", "\u{E1F9}"),
    ("pencil_line", "\u{E4F0}"),
    ("pencil_off", "\u{E5EF}"),
    ("pencil_ruler", "\u{E4F1}"),
    ("pentagon", "\u{E52B}"),
    ("percent", "\u{E132}"),
    ("person_standing", "\u{E21E}"),
    ("philippine_peso", "\u{E604}"),
    ("phone", "\u{E133}"),
    ("phone_call", "\u{E134}"),
    ("phone_forwarded", "\u{E135}"),
    ("phone_incoming", "\u{E136}"),
    ("phone_missed", "\u{E137}"),
    ("phone_off", "\u{E138}"),
    ("phone_outgoing", "\u{E139}"),
    ("pi", "\u{E472}"),
    ("piano", "\u{E561}"),
    ("pickaxe", "\u{E5C6}"),
    ("picture_in_picture", "\u{E3AE}"),
    ("picture_in_picture_2", "\u{E3AF}"),
    ("piggy_bank", "\u{E13A}"),
    ("pilcrow", "\u{E3A3}"),
    ("pilcrow_left", "\u{E5DC}"),
    ("pilcrow_right", "\u{E5DD}"),
    ("pill", "\u{E3BD}"),
    ("pill_bottle", "\u{E5EA}"),
    ("pin", "\u{E259}"),
    ("pin_off", "\u{E2B6}"),
    ("pipette", "\u{E13B}"),
    ("pizza", "\u{E354}"),
    ("plane", "\u{E1DE}"),
    ("plane_landing", "\u{E3CD}"),
    ("plane_takeoff", "\u{E3CE}"),
    ("play", "\u{E13C}"),
    ("plug", "\u{E37F}"),
    ("plug_2", "\u{E380}"),
    ("plug_zap", "\u{E45C}"),
    ("plus", "\u{E13D}"),
    ("pocket", "\u{E13E}"),
    ("pocket_knife", "\u{E4A0}"),
    ("podcast", "\u{E1FA}"),
    ("pointer", "\u{E1E8}"),
    ("pointer_off", "\u{E57F}"),
    ("popcorn", "\u{E4BE}"),
    ("popsicle", "\u{E4BF}"),
    ("pound_sterling", "\u{E13F}"),
    ("power", "\u{E140}"),
    ("power_off", "\u{E209}"),
    ("presentation", "\u{E4AE}"),
    ("printer", "\u{E141}"),
    ("printer_check", "\u{E5F5}"),
    ("printer_x", "\u{E6BB}"),
    ("projector", "\u{E4AF}"),
    ("proportions", "\u{E5CF}"),
    ("puzzle", "\u{E29C}"),
    ("pyramid", "\u{E52C}"),
    ("qr_code", "\u{E1DF}"),
    ("quote", "\u{E239}"),
    ("rabbit", "\u{E4F6}"),
    ("radar", "\u{E497}"),
    ("radiation", "\u{E442}"),
    ("radical", "\u{E5C2}"),
    ("radio", "\u{E142}"),
    ("radio_receiver", "\u{E1FB}"),
    ("radio_tower", "\u{E404}"),
    ("radius", "\u{E52D}"),
    ("rail_symbol", "\u{E501}"),
    ("rainbow", "\u{E4C2}"),
    ("rat", "\u{E3EB}"),
    ("ratio", "\u{E4E8}"),
    ("receipt", "\u{E3D3}"),
    ("receipt_cent", "\u{E5A5}"),
    ("receipt_euro", "\u{E5A6}"),
    ("receipt_indian_rupee", "\u{E5A7}"),
    ("receipt_japanese_yen", "\u{E5A8}"),
    ("receipt_pound_sterling", "\u{E5A9}"),
    ("receipt_russian_ruble", "\u{E5AA}"),
    ("receipt_swiss_franc", "\u{E5AB}"),
    ("receipt_text", "\u{E5AC}"),
    ("receipt_turkish_lira", "\u{E67F}"),
    ("rectangle_circle", "\u{E673}"),
    ("rectangle_ellipsis", "\u{E21F}"),
    ("rectangle_goggles", "\u{E656}"),
    ("rectangle_horizontal", "\u{E376}"),
    ("rectangle_vertical", "\u{E377}"),
    ("recycle", "\u{E2E9}"),
    ("redo", "\u{E143}"),
    ("redo_2", "\u{E2A0}"),
    ("redo_dot", "\u{E450}"),
    ("refresh_ccw", "\u{E144}"),
    ("refresh_ccw_dot", "\u{E4B2}"),
    ("refresh_cw", "\u{E145}"),
    ("refresh_cw_off", "\u{E498}"),
    ("refrigerator", "\u{E37B}"),
    ("regex", "\u{E1FC}"),
    ("remove_formatting", "\u{E3B3}"),
    ("repeat", "\u{E146}"),
    ("repeat_1", "\u{E1FD}"),
    ("repeat_2", "\u{E411}"),
    ("replace", "\u{E3DB}"),
    ("replace_all", "\u{E3DC}"),
    ("reply", "\u{E22A}"),
    ("reply_all", "\u{E22B}"),
    ("rewind", "\u{E147}"),
    ("ribbon", "\u{E558}"),
    ("rocket", "\u{E286}"),
    ("rocking_chair", "\u{E233}"),
    ("roller_coaster", "\u{E480}"),
    ("rose", "\u{E691}"),
    ("rotate_3d", "\u{E2EA}"),
    ("rotate_ccw", "\u{E148}"),
    ("rotate_ccw_key", "\u{E650}"),
    ("rotate_ccw_square", "\u{E5D0}"),
    ("rotate_cw", "\u{E149}"),
    ("rotate_cw_square", "\u{E5D1}"),
    ("route", "\u{E53E}"),
    ("route_off", "\u{E53F}"),
    ("router", "\u{E3BF}"),
    ("rows_2", "\u{E439}"),
    ("rows_3", "\u{E58A}"),
    ("rows_4", "\u{E58B}"),
    ("rss", "\u{E14A}"),
    ("ruler", "\u{E14B}"),
    ("ruler_dimension_line", "\u{E662}"),
    ("russian_ruble", "\u{E14C}"),
    ("sailboat", "\u{E37E}"),
    ("salad", "\u{E3A8}"),
    ("sandwich", "\u{E3A9}"),
    ("satellite", "\u{E447}"),
    ("satellite_dish", "\u{E448}"),
    ("saudi_riyal", "\u{E64B}"),
    ("save", "\u{E14D}"),
    ("save_all", "\u{E40F}"),
    ("save_off", "\u{E5F3}"),
    ("scale", "\u{E212}"),
    ("scale_3d", "\u{E2EB}"),
    ("scaling", "\u{E2EC}"),
    ("scan", "\u{E257}"),
    ("scan_barcode", "\u{E535}"),
    ("scan_eye", "\u{E536}"),
    ("scan_face", "\u{E371}"),
    ("scan_heart", "\u{E63A}"),
    ("scan_line", "\u{E258}"),
    ("scan_qr_code", "\u{E5F6}"),
    ("scan_search", "\u{E537}"),
    ("scan_text", "\u{E538}"),
    ("school", "\u{E3E3}"),
    ("scissors", "\u{E14E}"),
    ("scissors_line_dashed", "\u{E4E9}"),
    ("scooter", "\u{E6AC}"),
    ("screen_share", "\u{E14F}"),
    ("screen_share_off", "\u{E150}"),
    ("scroll", "\u{E2ED}"),
    ("scroll_text", "\u{E45F}"),
    ("search", "\u{E151}"),
    ("search_alert", "\u{E6B4}"),
    ("search_check", "\u{E4AA}"),
    ("search_code", "\u{E4AB}"),
    ("search_slash", "\u{E4AC}"),
    ("search_x", "\u{E4AD}"),
    ("section", "\u{E5E8}"),
    ("send", "\u{E152}"),
    ("send_horizontal", "\u{E4F2}"),
    ("send_to_back", "\u{E4F3}"),
    ("separator_horizontal", "\u{E1C8}"),
    ("separator_vertical", "\u{E1C9}"),
    ("server", "\u{E153}"),
    ("server_cog", "\u{E341}"),
    ("server_crash", "\u{E1E9}"),
    ("server_off", "\u{E1EA}"),
    ("settings", "\u{E154}"),
    ("settings_2", "\u{E245}"),
    ("shapes", "\u{E4B3}"),
    ("share", "\u{E155}"),
    ("share_2", "\u{E156}"),
    ("sheet", "\u{E157}"),
    ("shell", "\u{E4F7}"),
    ("shelving_unit", "\u{E6C7}"),
    ("shield", "\u{E158}"),
    ("shield_alert", "\u{E1FE}"),
    ("shield_ban", "\u{E159}"),
    ("shield_check", "\u{E1FF}"),
    ("shield_ellipsis", "\u{E516}"),
    ("shield_half", "\u{E517}"),
    ("shield_minus", "\u{E518}"),
    ("shield_off", "\u{E15A}"),
    ("shield_plus", "\u{E519}"),
    ("shield_question_mark", "\u{E40E}"),
    ("shield_user", "\u{E647}"),
    ("shield_x", "\u{E200}"),
    ("ship", "\u{E3BA}"),
    ("ship_wheel", "\u{E502}"),
    ("shirt", "\u{E1CA}"),
    ("shopping_bag", "\u{E15B}"),
    ("shopping_basket", "\u{E4EA}"),
    ("shopping_cart", "\u{E15C}"),
    ("shovel", "\u{E15D}"),
    ("shower_head", "\u{E37C}"),
    ("shredder", "\u{E65B}"),
    ("shrimp", "\u{E649}"),
    ("shrink", "\u{E220}"),
    ("shrub", "\u{E2EE}"),
    ("shuffle", "\u{E15E}"),
    ("sigma", "\u{E201}"),
    ("signal", "\u{E25F}"),
    ("signal_high", "\u{E260}"),
    ("signal_low", "\u{E261}"),
    ("signal_medium", "\u{E262}"),
    ("signal_zero", "\u{E263}"),
    ("signature", "\u{E5F2}"),
    ("signpost", "\u{E540}"),
    ("signpost_big", "\u{E541}"),
    ("siren", "\u{E2EF}"),
    ("skip_back", "\u{E15F}"),
    ("skip_forward", "\u{E160}"),
    ("skull", "\u{E221}"),
    ("slack", "\u{E161}"),
    ("slash", "\u{E51D}"),
    ("slice", "\u{E2F0}"),
    ("sliders_horizontal", "\u{E29A}"),
    ("sliders_vertical", "\u{E162}"),
    ("smartphone", "\u{E163}"),
    ("smartphone_charging", "\u{E22E}"),
    ("smartphone_nfc", "\u{E3C4}"),
    ("smile", "\u{E164}"),
    ("smile_plus", "\u{E301}"),
    ("snail", "\u{E4F8}"),
    ("snowflake", "\u{E165}"),
    ("soap_dispenser_droplet", "\u{E669}"),
    ("sofa", "\u{E2C4}"),
    ("solar_panel", "\u{E69F}"),
    ("soup", "\u{E3AA}"),
    ("space", "\u{E3DD}"),
    ("spade", "\u{E499}"),
    ("sparkle", "\u{E47E}"),
    ("sparkles", "\u{E412}"),
    ("speaker", "\u{E166}"),
    ("speech", "\u{E51E}"),
    ("spell_check", "\u{E49A}"),
    ("spell_check_2", "\u{E49B}"),
    ("spline", "\u{E38B}"),
    ("spline_pointer", "\u{E64F}"),
    ("split", "\u{E440}"),
    ("spool", "\u{E677}"),
    ("spotlight", "\u{E682}"),
    ("spray_can", "\u{E495}"),
    ("sprout", "\u{E1EB}"),
    ("square", "\u{E167}"),
    ("square_activity", "\u{E4B4}"),
    ("square_arrow_down", "\u{E427}"),
    ("square_arrow_down_left", "\u{E4B5}"),
    ("square_arrow_down_right", "\u{E4B6}"),
    ("square_arrow_left", "\u{E428}"),
    ("square_arrow_out_down_left", "\u{E5A1}"),
    ("square_arrow_out_down_right", "\u{E5A2}"),
    ("square_arrow_out_up_left", "\u{E5A3}"),
    ("square_arrow_out_up_right", "\u{E5A4}"),
    ("square_arrow_right", "\u{E429}"),
    ("square_arrow_right_enter", "\u{E6CE}"),
    ("square_arrow_right_exit", "\u{E6CF}"),
    ("square_arrow_up", "\u{E42A}"),
    ("square_arrow_up_left", "\u{E4B7}"),
    ("square_arrow_up_right", "\u{E4B8}"),
    ("square_asterisk", "\u{E168}"),
    ("square_bottom_dashed_scissors", "\u{E4EB}"),
    ("square_centerline_dashed_horizontal", "\u{E35F}"),
    ("square_centerline_dashed_vertical", "\u{E360}"),
    ("square_chart_gantt", "\u{E169}"),
    ("square_check", "\u{E559}"),
    ("square_check_big", "\u{E16A}"),
    ("square_chevron_down", "\u{E3CF}"),
    ("square_chevron_left", "\u{E3D0}"),
    ("square_chevron_right", "\u{E3D1}"),
    ("square_chevron_up", "\u{E3D2}"),
    ("square_code", "\u{E16B}"),
    ("square_dashed", "\u{E1CB}"),
    ("square_dashed_bottom", "\u{E4C0}"),
    ("square_dashed_bottom_code", "\u{E4C1}"),
    ("square_dashed_kanban", "\u{E16C}"),
    ("square_dashed_mouse_pointer", "\u{E509}"),
    ("square_dashed_top_solid", "\u{E66C}"),
    ("square_divide", "\u{E16D}"),
    ("square_dot", "\u{E16E}"),
    ("square_equal", "\u{E16F}"),
    ("square_function", "\u{E22D}"),
    ("square_kanban", "\u{E170}"),
    ("square_library", "\u{E54F}"),
    ("square_m", "\u{E503}"),
    ("square_menu", "\u{E453}"),
    ("square_minus", "\u{E171}"),
    ("square_mouse_pointer", "\u{E202}"),
    ("square_parking", "\u{E3CB}"),
    ("square_parking_off", "\u{E3CC}"),
    ("square_pause", "\u{E684}"),
    ("square_pen", "\u{E172}"),
    ("square_percent", "\u{E51C}"),
    ("square_pi", "\u{E488}"),
    ("square_pilcrow", "\u{E48B}"),
    ("square_play", "\u{E481}"),
    ("square_plus", "\u{E173}"),
    ("square_power", "\u{E551}"),
    ("square_radical", "\u{E5C3}"),
    ("square_round_corner", "\u{E648}"),
    ("square_scissors", "\u{E4EC}"),
    ("square_sigma", "\u{E489}"),
    ("square_slash", "\u{E174}"),
    ("square_split_horizontal", "\u{E3B6}"),
    ("square_split_vertical", "\u{E3B7}"),
    ("square_square", "\u{E60E}"),
    ("square_stack", "\u{E4A2}"),
    ("square_star", "\u{E68E}"),
    ("square_stop", "\u{E685}"),
    ("square_terminal", "\u{E20A}"),
    ("square_user", "\u{E465}"),
    ("square_user_round", "\u{E466}"),
    ("square_x", "\u{E175}"),
    ("squares_exclude", "\u{E657}"),
    ("squares_intersect", "\u{E658}"),
    ("squares_subtract", "\u{E659}"),
    ("squares_unite", "\u{E65A}"),
    ("squircle", "\u{E57A}"),
    ("squircle_dashed", "\u{E679}"),
    ("squirrel", "\u{E49F}"),
    ("stamp", "\u{E3BB}"),
    ("star", "\u{E176}"),
    ("star_half", "\u{E20B}"),
    ("star_off", "\u{E2B0}"),
    ("step_back", "\u{E3E9}"),
    ("step_forward", "\u{E3EA}"),
    ("stethoscope", "\u{E2F1}"),
    ("sticker", "\u{E302}"),
    ("sticky_note", "\u{E303}"),
    ("stone", "\u{E6B8}"),
    ("store", "\u{E3E4}"),
    ("stretch_horizontal", "\u{E27C}"),
    ("stretch_vertical", "\u{E27D}"),
    ("strikethrough", "\u{E177}"),
    ("subscript", "\u{E25C}"),
    ("sun", "\u{E178}"),
    ("sun_dim", "\u{E299}"),
    ("sun_medium", "\u{E2B1}"),
    ("sun_moon", "\u{E2B2}"),
    ("sun_snow", "\u{E372}"),
    ("sunrise", "\u{E179}"),
    ("sunset", "\u{E17A}"),
    ("superscript", "\u{E25E}"),
    ("swatch_book", "\u{E59F}"),
    ("swiss_franc", "\u{E17B}"),
    ("switch_camera", "\u{E17C}"),
    ("sword", "\u{E2B3}"),
    ("swords", "\u{E2B4}"),
    ("syringe", "\u{E2F2}"),
    ("table", "\u{E17D}"),
    ("table_2", "\u{E2F9}"),
    ("table_cells_merge", "\u{E5C7}"),
    ("table_cells_split", "\u{E5C8}"),
    ("table_columns_split", "\u{E5C9}"),
    ("table_of_contents", "\u{E61E}"),
    ("table_properties", "\u{E4DB}"),
    ("table_rows_split", "\u{E5CA}"),
    ("tablet", "\u{E17E}"),
    ("tablet_smartphone", "\u{E50A}"),
    ("tablets", "\u{E3BE}"),
    ("tag", "\u{E17F}"),
    ("tags", "\u{E35C}"),
    ("tally_1", "\u{E4D6}"),
    ("tally_2", "\u{E4D7}"),
    ("tally_3", "\u{E4D8}"),
    ("tally_4", "\u{E4D9}"),
    ("tally_5", "\u{E4DA}"),
    ("tangent", "\u{E52E}"),
    ("target", "\u{E180}"),
    ("telescope", "\u{E5C5}"),
    ("tent", "\u{E227}"),
    ("tent_tree", "\u{E53B}"),
    ("terminal", "\u{E181}"),
    ("test_tube", "\u{E405}"),
    ("test_tube_diagonal", "\u{E406}"),
    ("test_tubes", "\u{E407}"),
    ("text_align_center", "\u{E182}"),
    ("text_align_end", "\u{E183}"),
    ("text_align_justify", "\u{E184}"),
    ("text_align_start", "\u{E185}"),
    ("text_cursor", "\u{E264}"),
    ("text_cursor_input", "\u{E265}"),
    ("text_initial", "\u{E605}"),
    ("text_quote", "\u{E49E}"),
    ("text_search", "\u{E5AD}"),
    ("text_select", "\u{E3DE}"),
    ("text_wrap", "\u{E248}"),
    ("theater", "\u{E522}"),
    ("thermometer", "\u{E186}"),
    ("thermometer_snowflake", "\u{E187}"),
    ("thermometer_sun", "\u{E188}"),
    ("thumbs_down", "\u{E189}"),
    ("thumbs_up", "\u{E18A}"),
    ("ticket", "\u{E20F}"),
    ("ticket_check", "\u{E5AE}"),
    ("ticket_minus", "\u{E5AF}"),
    ("ticket_percent", "\u{E5B0}"),
    ("ticket_plus", "\u{E5B1}"),
    ("ticket_slash", "\u{E5B2}"),
    ("ticket_x", "\u{E5B3}"),
    ("tickets", "\u{E622}"),
    ("tickets_plane", "\u{E623}"),
    ("timer", "\u{E1E0}"),
    ("timer_off", "\u{E249}"),
    ("timer_reset", "\u{E236}"),
    ("toggle_left", "\u{E18B}"),
    ("toggle_right", "\u{E18C}"),
    ("toilet", "\u{E635}"),
    ("tool_case", "\u{E67D}"),
    ("toolbox", "\u{E6B9}"),
    ("tornado", "\u{E218}"),
    ("torus", "\u{E52F}"),
    ("touchpad", "\u{E449}"),
    ("touchpad_off", "\u{E44A}"),
    ("towel_rack", "\u{E6C6}"),
    ("tower_control", "\u{E3BC}"),
    ("toy_brick", "\u{E347}"),
    ("tractor", "\u{E504}"),
    ("traffic_cone", "\u{E505}"),
    ("train_front", "\u{E506}"),
    ("train_front_tunnel", "\u{E507}"),
    ("train_track", "\u{E508}"),
    ("tram_front", "\u{E2A9}"),
    ("transgender", "\u{E644}"),
    ("trash", "\u{E18D}"),
    ("trash_2", "\u{E18E}"),
    ("tree_deciduous", "\u{E2F3}"),
    ("tree_palm", "\u{E281}"),
    ("tree_pine", "\u{E2F4}"),
    ("trees", "\u{E2F5}"),
    ("trello", "\u{E18F}"),
    ("trending_down", "\u{E190}"),
    ("trending_up", "\u{E191}"),
    ("trending_up_down", "\u{E625}"),
    ("triangle", "\u{E192}"),
    ("triangle_alert", "\u{E193}"),
    ("triangle_dashed", "\u{E63D}"),
    ("triangle_right", "\u{E4ED}"),
    ("trophy", "\u{E373}"),
    ("truck", "\u{E194}"),
    ("truck_electric", "\u{E65F}"),
    ("turkish_lira", "\u{E680}"),
    ("turntable", "\u{E68C}"),
    ("turtle", "\u{E4F9}"),
    ("tv", "\u{E195}"),
    ("tv_minimal", "\u{E203}"),
    ("tv_minimal_play", "\u{E5EC}"),
    ("twitch", "\u{E196}"),
    ("twitter", "\u{E197}"),
    ("type_icon", "\u{E198}"),
    ("type_outline", "\u{E602}"),
    ("umbrella", "\u{E199}"),
    ("umbrella_off", "\u{E543}"),
    ("underline", "\u{E19A}"),
    ("undo", "\u{E19B}"),
    ("undo_2", "\u{E2A1}"),
    ("undo_dot", "\u{E451}"),
    ("unfold_horizontal", "\u{E43D}"),
    ("unfold_vertical", "\u{E43E}"),
    ("ungroup", "\u{E467}"),
    ("university", "\u{E3E5}"),
    ("unlink", "\u{E19C}"),
    ("unlink_2", "\u{E19D}"),
    ("unplug", "\u{E45D}"),
    ("upload", "\u{E19E}"),
    ("usb", "\u{E356}"),
    ("user", "\u{E19F}"),
    ("user_check", "\u{E1A0}"),
    ("user_cog", "\u{E342}"),
    ("user_key", "\u{E6BD}"),
    ("user_lock", "\u{E660}"),
    ("user_minus", "\u{E1A1}"),
    ("user_pen", "\u{E5FC}"),
    ("user_plus", "\u{E1A2}"),
    ("user_round", "\u{E468}"),
    ("user_round_check", "\u{E469}"),
    ("user_round_cog", "\u{E46A}"),
    ("user_round_key", "\u{E6BE}"),
    ("user_round_minus", "\u{E46B}"),
    ("user_round_pen", "\u{E5FD}"),
    ("user_round_plus", "\u{E46C}"),
    ("user_round_search", "\u{E578}"),
    ("user_round_x", "\u{E46D}"),
    ("user_search", "\u{E579}"),
    ("user_star", "\u{E687}"),
    ("user_x", "\u{E1A3}"),
    ("users", "\u{E1A4}"),
    ("users_round", "\u{E46E}"),
    ("utensils", "\u{E2F6}"),
    ("utensils_crossed", "\u{E2F7}"),
    ("utility_pole", "\u{E3C2}"),
    ("van", "\u{E6AD}"),
    ("variable", "\u{E473}"),
    ("vault", "\u{E58F}"),
    ("vector_square", "\u{E67C}"),
    ("vegan", "\u{E39D}"),
    ("venetian_mask", "\u{E2AA}"),
    ("venus", "\u{E645}"),
    ("venus_and_mars", "\u{E646}"),
    ("vibrate", "\u{E223}"),
    ("vibrate_off", "\u{E29D}"),
    ("video", "\u{E1A5}"),
    ("video_off", "\u{E1A6}"),
    ("videotape", "\u{E4CB}"),
    ("view", "\u{E1A7}"),
    ("voicemail", "\u{E1A8}"),
    ("volleyball", "\u{E62F}"),
    ("volume", "\u{E1A9}"),
    ("volume_1", "\u{E1AA}"),
    ("volume_2", "\u{E1AB}"),
    ("volume_off", "\u{E626}"),
    ("volume_x", "\u{E1AC}"),
    ("vote", "\u{E3AD}"),
    ("wallet", "\u{E204}"),
    ("wallet_cards", "\u{E4CC}"),
    ("wallet_minimal", "\u{E4CD}"),
    ("wallpaper", "\u{E44B}"),
    ("wand", "\u{E246}"),
    ("wand_sparkles", "\u{E357}"),
    ("warehouse", "\u{E3E6}"),
    ("washing_machine", "\u{E590}"),
    ("watch", "\u{E1AD}"),
    ("waves", "\u{E283}"),
    ("waves_arrow_down", "\u{E6A9}"),
    ("waves_arrow_up", "\u{E6AA}"),
    ("waves_ladder", "\u{E63B}"),
    ("waypoints", "\u{E542}"),
    ("webcam", "\u{E205}"),
    ("webhook", "\u{E374}"),
    ("webhook_off", "\u{E5B7}"),
    ("weight", "\u{E530}"),
    ("weight_tilde", "\u{E6AE}"),
    ("wheat", "\u{E39E}"),
    ("wheat_off", "\u{E39F}"),
    ("whole_word", "\u{E3DF}"),
    ("wifi", "\u{E1AE}"),
    ("wifi_cog", "\u{E674}"),
    ("wifi_high", "\u{E5F7}"),
    ("wifi_low", "\u{E5F8}"),
    ("wifi_off", "\u{E1AF}"),
    ("wifi_pen", "\u{E663}"),
    ("wifi_sync", "\u{E681}"),
    ("wifi_zero", "\u{E5F9}"),
    ("wind", "\u{E1B0}"),
    ("wind_arrow_down", "\u{E631}"),
    ("wine", "\u{E2F8}"),
    ("wine_off", "\u{E3A0}"),
    ("workflow", "\u{E425}"),
    ("worm", "\u{E5DA}"),
    ("wrench", "\u{E1B1}"),
    ("x", "\u{E1B2}"),
    ("x_line_top", "\u{E6CB}"),
    ("youtube", "\u{E1B3}"),
    ("zap", "\u{E1B4}"),
    ("zap_off", "\u{E1B5}"),
    ("zoom_in", "\u{E1B6}"),
    ("zoom_out", "\u{E1B7}"),
];

pub fn a_arrow_down<'a>() -> Text<'a> {
    icon("\u{E585}")
}

pub fn a_arrow_up<'a>() -> Text<'a> {
    icon("\u{E586}")
}

pub fn a_large_small<'a>() -> Text<'a> {
    icon("\u{E587}")
}

pub fn accessibility<'a>() -> Text<'a> {
    icon("\u{E297}")
}

pub fn activity<'a>() -> Text<'a> {
    icon("\u{E038}")
}

pub fn air_vent<'a>() -> Text<'a> {
    icon("\u{E34D}")
}

pub fn airplay<'a>() -> Text<'a> {
    icon("\u{E039}")
}

pub fn alarm_clock<'a>() -> Text<'a> {
    icon("\u{E03A}")
}

pub fn alarm_clock_check<'a>() -> Text<'a> {
    icon("\u{E1EC}")
}

pub fn alarm_clock_minus<'a>() -> Text<'a> {
    icon("\u{E1ED}")
}

pub fn alarm_clock_off<'a>() -> Text<'a> {
    icon("\u{E23B}")
}

pub fn alarm_clock_plus<'a>() -> Text<'a> {
    icon("\u{E1EE}")
}

pub fn alarm_smoke<'a>() -> Text<'a> {
    icon("\u{E57B}")
}

pub fn album<'a>() -> Text<'a> {
    icon("\u{E03B}")
}

pub fn align_center_horizontal<'a>() -> Text<'a> {
    icon("\u{E26C}")
}

pub fn align_center_vertical<'a>() -> Text<'a> {
    icon("\u{E26D}")
}

pub fn align_end_horizontal<'a>() -> Text<'a> {
    icon("\u{E26E}")
}

pub fn align_end_vertical<'a>() -> Text<'a> {
    icon("\u{E26F}")
}

pub fn align_horizontal_distribute_center<'a>() -> Text<'a> {
    icon("\u{E03C}")
}

pub fn align_horizontal_distribute_end<'a>() -> Text<'a> {
    icon("\u{E03D}")
}

pub fn align_horizontal_distribute_start<'a>() -> Text<'a> {
    icon("\u{E03E}")
}

pub fn align_horizontal_justify_center<'a>() -> Text<'a> {
    icon("\u{E272}")
}

pub fn align_horizontal_justify_end<'a>() -> Text<'a> {
    icon("\u{E273}")
}

pub fn align_horizontal_justify_start<'a>() -> Text<'a> {
    icon("\u{E274}")
}

pub fn align_horizontal_space_around<'a>() -> Text<'a> {
    icon("\u{E275}")
}

pub fn align_horizontal_space_between<'a>() -> Text<'a> {
    icon("\u{E276}")
}

pub fn align_start_horizontal<'a>() -> Text<'a> {
    icon("\u{E270}")
}

pub fn align_start_vertical<'a>() -> Text<'a> {
    icon("\u{E271}")
}

pub fn align_vertical_distribute_center<'a>() -> Text<'a> {
    icon("\u{E27E}")
}

pub fn align_vertical_distribute_end<'a>() -> Text<'a> {
    icon("\u{E27F}")
}

pub fn align_vertical_distribute_start<'a>() -> Text<'a> {
    icon("\u{E280}")
}

pub fn align_vertical_justify_center<'a>() -> Text<'a> {
    icon("\u{E277}")
}

pub fn align_vertical_justify_end<'a>() -> Text<'a> {
    icon("\u{E278}")
}

pub fn align_vertical_justify_start<'a>() -> Text<'a> {
    icon("\u{E279}")
}

pub fn align_vertical_space_around<'a>() -> Text<'a> {
    icon("\u{E27A}")
}

pub fn align_vertical_space_between<'a>() -> Text<'a> {
    icon("\u{E27B}")
}

pub fn ambulance<'a>() -> Text<'a> {
    icon("\u{E5BB}")
}

pub fn ampersand<'a>() -> Text<'a> {
    icon("\u{E49C}")
}

pub fn ampersands<'a>() -> Text<'a> {
    icon("\u{E49D}")
}

pub fn amphora<'a>() -> Text<'a> {
    icon("\u{E61B}")
}

pub fn anchor<'a>() -> Text<'a> {
    icon("\u{E03F}")
}

pub fn angry<'a>() -> Text<'a> {
    icon("\u{E2FC}")
}

pub fn annoyed<'a>() -> Text<'a> {
    icon("\u{E2FD}")
}

pub fn antenna<'a>() -> Text<'a> {
    icon("\u{E4E2}")
}

pub fn anvil<'a>() -> Text<'a> {
    icon("\u{E580}")
}

pub fn aperture<'a>() -> Text<'a> {
    icon("\u{E040}")
}

pub fn app_window<'a>() -> Text<'a> {
    icon("\u{E426}")
}

pub fn app_window_mac<'a>() -> Text<'a> {
    icon("\u{E5D2}")
}

pub fn apple<'a>() -> Text<'a> {
    icon("\u{E34E}")
}

pub fn archive<'a>() -> Text<'a> {
    icon("\u{E041}")
}

pub fn archive_restore<'a>() -> Text<'a> {
    icon("\u{E2CD}")
}

pub fn archive_x<'a>() -> Text<'a> {
    icon("\u{E50C}")
}

pub fn armchair<'a>() -> Text<'a> {
    icon("\u{E2C0}")
}

pub fn arrow_big_down<'a>() -> Text<'a> {
    icon("\u{E1E1}")
}

pub fn arrow_big_down_dash<'a>() -> Text<'a> {
    icon("\u{E41D}")
}

pub fn arrow_big_left<'a>() -> Text<'a> {
    icon("\u{E1E2}")
}

pub fn arrow_big_left_dash<'a>() -> Text<'a> {
    icon("\u{E41E}")
}

pub fn arrow_big_right<'a>() -> Text<'a> {
    icon("\u{E1E3}")
}

pub fn arrow_big_right_dash<'a>() -> Text<'a> {
    icon("\u{E41F}")
}

pub fn arrow_big_up<'a>() -> Text<'a> {
    icon("\u{E1E4}")
}

pub fn arrow_big_up_dash<'a>() -> Text<'a> {
    icon("\u{E420}")
}

pub fn arrow_down<'a>() -> Text<'a> {
    icon("\u{E042}")
}

pub fn arrow_down_0_1<'a>() -> Text<'a> {
    icon("\u{E413}")
}

pub fn arrow_down_1_0<'a>() -> Text<'a> {
    icon("\u{E414}")
}

pub fn arrow_down_a_z<'a>() -> Text<'a> {
    icon("\u{E415}")
}

pub fn arrow_down_from_line<'a>() -> Text<'a> {
    icon("\u{E454}")
}

pub fn arrow_down_left<'a>() -> Text<'a> {
    icon("\u{E043}")
}

pub fn arrow_down_narrow_wide<'a>() -> Text<'a> {
    icon("\u{E044}")
}

pub fn arrow_down_right<'a>() -> Text<'a> {
    icon("\u{E045}")
}

pub fn arrow_down_to_dot<'a>() -> Text<'a> {
    icon("\u{E44D}")
}

pub fn arrow_down_to_line<'a>() -> Text<'a> {
    icon("\u{E455}")
}

pub fn arrow_down_up<'a>() -> Text<'a> {
    icon("\u{E046}")
}

pub fn arrow_down_wide_narrow<'a>() -> Text<'a> {
    icon("\u{E047}")
}

pub fn arrow_down_z_a<'a>() -> Text<'a> {
    icon("\u{E416}")
}

pub fn arrow_left<'a>() -> Text<'a> {
    icon("\u{E048}")
}

pub fn arrow_left_from_line<'a>() -> Text<'a> {
    icon("\u{E456}")
}

pub fn arrow_left_right<'a>() -> Text<'a> {
    icon("\u{E24A}")
}

pub fn arrow_left_to_line<'a>() -> Text<'a> {
    icon("\u{E457}")
}

pub fn arrow_right<'a>() -> Text<'a> {
    icon("\u{E049}")
}

pub fn arrow_right_from_line<'a>() -> Text<'a> {
    icon("\u{E458}")
}

pub fn arrow_right_left<'a>() -> Text<'a> {
    icon("\u{E417}")
}

pub fn arrow_right_to_line<'a>() -> Text<'a> {
    icon("\u{E459}")
}

pub fn arrow_up<'a>() -> Text<'a> {
    icon("\u{E04A}")
}

pub fn arrow_up_0_1<'a>() -> Text<'a> {
    icon("\u{E418}")
}

pub fn arrow_up_1_0<'a>() -> Text<'a> {
    icon("\u{E419}")
}

pub fn arrow_up_a_z<'a>() -> Text<'a> {
    icon("\u{E41A}")
}

pub fn arrow_up_down<'a>() -> Text<'a> {
    icon("\u{E37D}")
}

pub fn arrow_up_from_dot<'a>() -> Text<'a> {
    icon("\u{E44E}")
}

pub fn arrow_up_from_line<'a>() -> Text<'a> {
    icon("\u{E45A}")
}

pub fn arrow_up_left<'a>() -> Text<'a> {
    icon("\u{E04B}")
}

pub fn arrow_up_narrow_wide<'a>() -> Text<'a> {
    icon("\u{E04C}")
}

pub fn arrow_up_right<'a>() -> Text<'a> {
    icon("\u{E04D}")
}

pub fn arrow_up_to_line<'a>() -> Text<'a> {
    icon("\u{E45B}")
}

pub fn arrow_up_wide_narrow<'a>() -> Text<'a> {
    icon("\u{E41B}")
}

pub fn arrow_up_z_a<'a>() -> Text<'a> {
    icon("\u{E41C}")
}

pub fn arrows_up_from_line<'a>() -> Text<'a> {
    icon("\u{E4D4}")
}

pub fn asterisk<'a>() -> Text<'a> {
    icon("\u{E1EF}")
}

pub fn at_sign<'a>() -> Text<'a> {
    icon("\u{E04E}")
}

pub fn atom<'a>() -> Text<'a> {
    icon("\u{E3D7}")
}

pub fn audio_lines<'a>() -> Text<'a> {
    icon("\u{E55A}")
}

pub fn audio_waveform<'a>() -> Text<'a> {
    icon("\u{E55B}")
}

pub fn award<'a>() -> Text<'a> {
    icon("\u{E04F}")
}

pub fn axe<'a>() -> Text<'a> {
    icon("\u{E050}")
}

pub fn axis_3d<'a>() -> Text<'a> {
    icon("\u{E2FE}")
}

pub fn baby<'a>() -> Text<'a> {
    icon("\u{E2CE}")
}

pub fn backpack<'a>() -> Text<'a> {
    icon("\u{E2C8}")
}

pub fn badge<'a>() -> Text<'a> {
    icon("\u{E474}")
}

pub fn badge_alert<'a>() -> Text<'a> {
    icon("\u{E475}")
}

pub fn badge_cent<'a>() -> Text<'a> {
    icon("\u{E50F}")
}

pub fn badge_check<'a>() -> Text<'a> {
    icon("\u{E241}")
}

pub fn badge_dollar_sign<'a>() -> Text<'a> {
    icon("\u{E476}")
}

pub fn badge_euro<'a>() -> Text<'a> {
    icon("\u{E510}")
}

pub fn badge_indian_rupee<'a>() -> Text<'a> {
    icon("\u{E511}")
}

pub fn badge_info<'a>() -> Text<'a> {
    icon("\u{E477}")
}

pub fn badge_japanese_yen<'a>() -> Text<'a> {
    icon("\u{E512}")
}

pub fn badge_minus<'a>() -> Text<'a> {
    icon("\u{E478}")
}

pub fn badge_percent<'a>() -> Text<'a> {
    icon("\u{E479}")
}

pub fn badge_plus<'a>() -> Text<'a> {
    icon("\u{E47A}")
}

pub fn badge_pound_sterling<'a>() -> Text<'a> {
    icon("\u{E513}")
}

pub fn badge_question_mark<'a>() -> Text<'a> {
    icon("\u{E47B}")
}

pub fn badge_russian_ruble<'a>() -> Text<'a> {
    icon("\u{E514}")
}

pub fn badge_swiss_franc<'a>() -> Text<'a> {
    icon("\u{E515}")
}

pub fn badge_turkish_lira<'a>() -> Text<'a> {
    icon("\u{E67E}")
}

pub fn badge_x<'a>() -> Text<'a> {
    icon("\u{E47C}")
}

pub fn baggage_claim<'a>() -> Text<'a> {
    icon("\u{E2C9}")
}

pub fn balloon<'a>() -> Text<'a> {
    icon("\u{E6AF}")
}

pub fn ban<'a>() -> Text<'a> {
    icon("\u{E051}")
}

pub fn banana<'a>() -> Text<'a> {
    icon("\u{E34F}")
}

pub fn bandage<'a>() -> Text<'a> {
    icon("\u{E61D}")
}

pub fn banknote<'a>() -> Text<'a> {
    icon("\u{E052}")
}

pub fn banknote_arrow_down<'a>() -> Text<'a> {
    icon("\u{E64C}")
}

pub fn banknote_arrow_up<'a>() -> Text<'a> {
    icon("\u{E64D}")
}

pub fn banknote_x<'a>() -> Text<'a> {
    icon("\u{E64E}")
}

pub fn barcode<'a>() -> Text<'a> {
    icon("\u{E533}")
}

pub fn barrel<'a>() -> Text<'a> {
    icon("\u{E675}")
}

pub fn baseline<'a>() -> Text<'a> {
    icon("\u{E285}")
}

pub fn bath<'a>() -> Text<'a> {
    icon("\u{E2AB}")
}

pub fn battery<'a>() -> Text<'a> {
    icon("\u{E053}")
}

pub fn battery_charging<'a>() -> Text<'a> {
    icon("\u{E054}")
}

pub fn battery_full<'a>() -> Text<'a> {
    icon("\u{E055}")
}

pub fn battery_low<'a>() -> Text<'a> {
    icon("\u{E056}")
}

pub fn battery_medium<'a>() -> Text<'a> {
    icon("\u{E057}")
}

pub fn battery_plus<'a>() -> Text<'a> {
    icon("\u{E63E}")
}

pub fn battery_warning<'a>() -> Text<'a> {
    icon("\u{E3AC}")
}

pub fn beaker<'a>() -> Text<'a> {
    icon("\u{E058}")
}

pub fn bean<'a>() -> Text<'a> {
    icon("\u{E38F}")
}

pub fn bean_off<'a>() -> Text<'a> {
    icon("\u{E390}")
}

pub fn bed<'a>() -> Text<'a> {
    icon("\u{E2C1}")
}

pub fn bed_double<'a>() -> Text<'a> {
    icon("\u{E2C2}")
}

pub fn bed_single<'a>() -> Text<'a> {
    icon("\u{E2C3}")
}

pub fn beef<'a>() -> Text<'a> {
    icon("\u{E3A5}")
}

pub fn beer<'a>() -> Text<'a> {
    icon("\u{E2CF}")
}

pub fn beer_off<'a>() -> Text<'a> {
    icon("\u{E5D9}")
}

pub fn bell<'a>() -> Text<'a> {
    icon("\u{E059}")
}

pub fn bell_dot<'a>() -> Text<'a> {
    icon("\u{E42B}")
}

pub fn bell_electric<'a>() -> Text<'a> {
    icon("\u{E57C}")
}

pub fn bell_minus<'a>() -> Text<'a> {
    icon("\u{E1F0}")
}

pub fn bell_off<'a>() -> Text<'a> {
    icon("\u{E05A}")
}

pub fn bell_plus<'a>() -> Text<'a> {
    icon("\u{E1F1}")
}

pub fn bell_ring<'a>() -> Text<'a> {
    icon("\u{E224}")
}

pub fn between_horizontal_end<'a>() -> Text<'a> {
    icon("\u{E591}")
}

pub fn between_horizontal_start<'a>() -> Text<'a> {
    icon("\u{E592}")
}

pub fn between_vertical_end<'a>() -> Text<'a> {
    icon("\u{E593}")
}

pub fn between_vertical_start<'a>() -> Text<'a> {
    icon("\u{E594}")
}

pub fn biceps_flexed<'a>() -> Text<'a> {
    icon("\u{E5EB}")
}

pub fn bike<'a>() -> Text<'a> {
    icon("\u{E1D2}")
}

pub fn binary<'a>() -> Text<'a> {
    icon("\u{E1F2}")
}

pub fn binoculars<'a>() -> Text<'a> {
    icon("\u{E621}")
}

pub fn biohazard<'a>() -> Text<'a> {
    icon("\u{E441}")
}

pub fn bird<'a>() -> Text<'a> {
    icon("\u{E3C5}")
}

pub fn birdhouse<'a>() -> Text<'a> {
    icon("\u{E69A}")
}

pub fn bitcoin<'a>() -> Text<'a> {
    icon("\u{E05B}")
}

pub fn blend<'a>() -> Text<'a> {
    icon("\u{E59C}")
}

pub fn blinds<'a>() -> Text<'a> {
    icon("\u{E3C0}")
}

pub fn blocks<'a>() -> Text<'a> {
    icon("\u{E4FA}")
}

pub fn bluetooth<'a>() -> Text<'a> {
    icon("\u{E05C}")
}

pub fn bluetooth_connected<'a>() -> Text<'a> {
    icon("\u{E1B8}")
}

pub fn bluetooth_off<'a>() -> Text<'a> {
    icon("\u{E1B9}")
}

pub fn bluetooth_searching<'a>() -> Text<'a> {
    icon("\u{E1BA}")
}

pub fn bold<'a>() -> Text<'a> {
    icon("\u{E05D}")
}

pub fn bolt<'a>() -> Text<'a> {
    icon("\u{E58C}")
}

pub fn bomb<'a>() -> Text<'a> {
    icon("\u{E2FF}")
}

pub fn bone<'a>() -> Text<'a> {
    icon("\u{E358}")
}

pub fn book<'a>() -> Text<'a> {
    icon("\u{E05E}")
}

pub fn book_a<'a>() -> Text<'a> {
    icon("\u{E544}")
}

pub fn book_alert<'a>() -> Text<'a> {
    icon("\u{E672}")
}

pub fn book_audio<'a>() -> Text<'a> {
    icon("\u{E545}")
}

pub fn book_check<'a>() -> Text<'a> {
    icon("\u{E546}")
}

pub fn book_copy<'a>() -> Text<'a> {
    icon("\u{E3EC}")
}

pub fn book_dashed<'a>() -> Text<'a> {
    icon("\u{E3ED}")
}

pub fn book_down<'a>() -> Text<'a> {
    icon("\u{E3EE}")
}

pub fn book_headphones<'a>() -> Text<'a> {
    icon("\u{E547}")
}

pub fn book_heart<'a>() -> Text<'a> {
    icon("\u{E548}")
}

pub fn book_image<'a>() -> Text<'a> {
    icon("\u{E549}")
}

pub fn book_key<'a>() -> Text<'a> {
    icon("\u{E3EF}")
}

pub fn book_lock<'a>() -> Text<'a> {
    icon("\u{E3F0}")
}

pub fn book_marked<'a>() -> Text<'a> {
    icon("\u{E3F1}")
}

pub fn book_minus<'a>() -> Text<'a> {
    icon("\u{E3F2}")
}

pub fn book_open<'a>() -> Text<'a> {
    icon("\u{E05F}")
}

pub fn book_open_check<'a>() -> Text<'a> {
    icon("\u{E381}")
}

pub fn book_open_text<'a>() -> Text<'a> {
    icon("\u{E54A}")
}

pub fn book_plus<'a>() -> Text<'a> {
    icon("\u{E3F3}")
}

pub fn book_search<'a>() -> Text<'a> {
    icon("\u{E6AB}")
}

pub fn book_text<'a>() -> Text<'a> {
    icon("\u{E54B}")
}

pub fn book_type<'a>() -> Text<'a> {
    icon("\u{E54C}")
}

pub fn book_up<'a>() -> Text<'a> {
    icon("\u{E3F4}")
}

pub fn book_up_2<'a>() -> Text<'a> {
    icon("\u{E4A6}")
}

pub fn book_user<'a>() -> Text<'a> {
    icon("\u{E54D}")
}

pub fn book_x<'a>() -> Text<'a> {
    icon("\u{E3F5}")
}

pub fn bookmark<'a>() -> Text<'a> {
    icon("\u{E060}")
}

pub fn bookmark_check<'a>() -> Text<'a> {
    icon("\u{E51F}")
}

pub fn bookmark_minus<'a>() -> Text<'a> {
    icon("\u{E23C}")
}

pub fn bookmark_plus<'a>() -> Text<'a> {
    icon("\u{E23D}")
}

pub fn bookmark_x<'a>() -> Text<'a> {
    icon("\u{E520}")
}

pub fn boom_box<'a>() -> Text<'a> {
    icon("\u{E4EE}")
}

pub fn bot<'a>() -> Text<'a> {
    icon("\u{E1BB}")
}

pub fn bot_message_square<'a>() -> Text<'a> {
    icon("\u{E5CE}")
}

pub fn bot_off<'a>() -> Text<'a> {
    icon("\u{E5E0}")
}

pub fn bottle_wine<'a>() -> Text<'a> {
    icon("\u{E67B}")
}

pub fn bow_arrow<'a>() -> Text<'a> {
    icon("\u{E65E}")
}

pub fn box_icon<'a>() -> Text<'a> {
    icon("\u{E061}")
}

pub fn boxes<'a>() -> Text<'a> {
    icon("\u{E2D0}")
}

pub fn braces<'a>() -> Text<'a> {
    icon("\u{E36A}")
}

pub fn brackets<'a>() -> Text<'a> {
    icon("\u{E443}")
}

pub fn brain<'a>() -> Text<'a> {
    icon("\u{E3C6}")
}

pub fn brain_circuit<'a>() -> Text<'a> {
    icon("\u{E3C7}")
}

pub fn brain_cog<'a>() -> Text<'a> {
    icon("\u{E3C8}")
}

pub fn brick_wall<'a>() -> Text<'a> {
    icon("\u{E581}")
}

pub fn brick_wall_fire<'a>() -> Text<'a> {
    icon("\u{E653}")
}

pub fn brick_wall_shield<'a>() -> Text<'a> {
    icon("\u{E690}")
}

pub fn briefcase<'a>() -> Text<'a> {
    icon("\u{E062}")
}

pub fn briefcase_business<'a>() -> Text<'a> {
    icon("\u{E5D5}")
}

pub fn briefcase_conveyor_belt<'a>() -> Text<'a> {
    icon("\u{E62B}")
}

pub fn briefcase_medical<'a>() -> Text<'a> {
    icon("\u{E5D6}")
}

pub fn bring_to_front<'a>() -> Text<'a> {
    icon("\u{E4EF}")
}

pub fn brush<'a>() -> Text<'a> {
    icon("\u{E1D3}")
}

pub fn brush_cleaning<'a>() -> Text<'a> {
    icon("\u{E666}")
}

pub fn bubbles<'a>() -> Text<'a> {
    icon("\u{E654}")
}

pub fn bug<'a>() -> Text<'a> {
    icon("\u{E20C}")
}

pub fn bug_off<'a>() -> Text<'a> {
    icon("\u{E50D}")
}

pub fn bug_play<'a>() -> Text<'a> {
    icon("\u{E50E}")
}

pub fn building<'a>() -> Text<'a> {
    icon("\u{E1CC}")
}

pub fn building_2<'a>() -> Text<'a> {
    icon("\u{E290}")
}

pub fn bus<'a>() -> Text<'a> {
    icon("\u{E1D4}")
}

pub fn bus_front<'a>() -> Text<'a> {
    icon("\u{E4FB}")
}

pub fn cable<'a>() -> Text<'a> {
    icon("\u{E4E3}")
}

pub fn cable_car<'a>() -> Text<'a> {
    icon("\u{E4FC}")
}

pub fn cake<'a>() -> Text<'a> {
    icon("\u{E344}")
}

pub fn cake_slice<'a>() -> Text<'a> {
    icon("\u{E4B9}")
}

pub fn calculator<'a>() -> Text<'a> {
    icon("\u{E1BC}")
}

pub fn calendar<'a>() -> Text<'a> {
    icon("\u{E063}")
}

pub fn calendar_1<'a>() -> Text<'a> {
    icon("\u{E630}")
}

pub fn calendar_arrow_down<'a>() -> Text<'a> {
    icon("\u{E5FE}")
}

pub fn calendar_arrow_up<'a>() -> Text<'a> {
    icon("\u{E5FF}")
}

pub fn calendar_check<'a>() -> Text<'a> {
    icon("\u{E2B7}")
}

pub fn calendar_check_2<'a>() -> Text<'a> {
    icon("\u{E2B8}")
}

pub fn calendar_clock<'a>() -> Text<'a> {
    icon("\u{E304}")
}

pub fn calendar_cog<'a>() -> Text<'a> {
    icon("\u{E5ED}")
}

pub fn calendar_days<'a>() -> Text<'a> {
    icon("\u{E2B9}")
}

pub fn calendar_fold<'a>() -> Text<'a> {
    icon("\u{E5B4}")
}

pub fn calendar_heart<'a>() -> Text<'a> {
    icon("\u{E305}")
}

pub fn calendar_minus<'a>() -> Text<'a> {
    icon("\u{E2BA}")
}

pub fn calendar_minus_2<'a>() -> Text<'a> {
    icon("\u{E5B5}")
}

pub fn calendar_off<'a>() -> Text<'a> {
    icon("\u{E2BB}")
}

pub fn calendar_plus<'a>() -> Text<'a> {
    icon("\u{E2BC}")
}

pub fn calendar_plus_2<'a>() -> Text<'a> {
    icon("\u{E5B6}")
}

pub fn calendar_range<'a>() -> Text<'a> {
    icon("\u{E2BD}")
}

pub fn calendar_search<'a>() -> Text<'a> {
    icon("\u{E306}")
}

pub fn calendar_sync<'a>() -> Text<'a> {
    icon("\u{E636}")
}

pub fn calendar_x<'a>() -> Text<'a> {
    icon("\u{E2BE}")
}

pub fn calendar_x_2<'a>() -> Text<'a> {
    icon("\u{E2BF}")
}

pub fn calendars<'a>() -> Text<'a> {
    icon("\u{E6A7}")
}

pub fn camera<'a>() -> Text<'a> {
    icon("\u{E064}")
}

pub fn camera_off<'a>() -> Text<'a> {
    icon("\u{E065}")
}

pub fn candy<'a>() -> Text<'a> {
    icon("\u{E391}")
}

pub fn candy_cane<'a>() -> Text<'a> {
    icon("\u{E4BA}")
}

pub fn candy_off<'a>() -> Text<'a> {
    icon("\u{E392}")
}

pub fn cannabis<'a>() -> Text<'a> {
    icon("\u{E5D4}")
}

pub fn cannabis_off<'a>() -> Text<'a> {
    icon("\u{E6B7}")
}

pub fn captions<'a>() -> Text<'a> {
    icon("\u{E3A4}")
}

pub fn captions_off<'a>() -> Text<'a> {
    icon("\u{E5C1}")
}

pub fn car<'a>() -> Text<'a> {
    icon("\u{E1D5}")
}

pub fn car_front<'a>() -> Text<'a> {
    icon("\u{E4FD}")
}

pub fn car_taxi_front<'a>() -> Text<'a> {
    icon("\u{E4FE}")
}

pub fn caravan<'a>() -> Text<'a> {
    icon("\u{E539}")
}

pub fn card_sim<'a>() -> Text<'a> {
    icon("\u{E671}")
}

pub fn carrot<'a>() -> Text<'a> {
    icon("\u{E25A}")
}

pub fn case_lower<'a>() -> Text<'a> {
    icon("\u{E3D8}")
}

pub fn case_sensitive<'a>() -> Text<'a> {
    icon("\u{E3D9}")
}

pub fn case_upper<'a>() -> Text<'a> {
    icon("\u{E3DA}")
}

pub fn cassette_tape<'a>() -> Text<'a> {
    icon("\u{E4CA}")
}

pub fn cast<'a>() -> Text<'a> {
    icon("\u{E066}")
}

pub fn castle<'a>() -> Text<'a> {
    icon("\u{E3E0}")
}

pub fn cat<'a>() -> Text<'a> {
    icon("\u{E38C}")
}

pub fn cctv<'a>() -> Text<'a> {
    icon("\u{E57D}")
}

pub fn chart_area<'a>() -> Text<'a> {
    icon("\u{E4D3}")
}

pub fn chart_bar<'a>() -> Text<'a> {
    icon("\u{E2A2}")
}

pub fn chart_bar_big<'a>() -> Text<'a> {
    icon("\u{E4A7}")
}

pub fn chart_bar_decreasing<'a>() -> Text<'a> {
    icon("\u{E607}")
}

pub fn chart_bar_increasing<'a>() -> Text<'a> {
    icon("\u{E608}")
}

pub fn chart_bar_stacked<'a>() -> Text<'a> {
    icon("\u{E609}")
}

pub fn chart_candlestick<'a>() -> Text<'a> {
    icon("\u{E4A8}")
}

pub fn chart_column<'a>() -> Text<'a> {
    icon("\u{E2A3}")
}

pub fn chart_column_big<'a>() -> Text<'a> {
    icon("\u{E4A9}")
}

pub fn chart_column_decreasing<'a>() -> Text<'a> {
    icon("\u{E067}")
}

pub fn chart_column_increasing<'a>() -> Text<'a> {
    icon("\u{E2A4}")
}

pub fn chart_column_stacked<'a>() -> Text<'a> {
    icon("\u{E60A}")
}

pub fn chart_gantt<'a>() -> Text<'a> {
    icon("\u{E624}")
}

pub fn chart_line<'a>() -> Text<'a> {
    icon("\u{E2A5}")
}

pub fn chart_network<'a>() -> Text<'a> {
    icon("\u{E60B}")
}

pub fn chart_no_axes_column<'a>() -> Text<'a> {
    icon("\u{E068}")
}

pub fn chart_no_axes_column_decreasing<'a>() -> Text<'a> {
    icon("\u{E069}")
}

pub fn chart_no_axes_column_increasing<'a>() -> Text<'a> {
    icon("\u{E06A}")
}

pub fn chart_no_axes_combined<'a>() -> Text<'a> {
    icon("\u{E60C}")
}

pub fn chart_no_axes_gantt<'a>() -> Text<'a> {
    icon("\u{E4C4}")
}

pub fn chart_pie<'a>() -> Text<'a> {
    icon("\u{E06B}")
}

pub fn chart_scatter<'a>() -> Text<'a> {
    icon("\u{E48A}")
}

pub fn chart_spline<'a>() -> Text<'a> {
    icon("\u{E60D}")
}

pub fn check<'a>() -> Text<'a> {
    icon("\u{E06C}")
}

pub fn check_check<'a>() -> Text<'a> {
    icon("\u{E38E}")
}

pub fn check_line<'a>() -> Text<'a> {
    icon("\u{E66B}")
}

pub fn chef_hat<'a>() -> Text<'a> {
    icon("\u{E2AC}")
}

pub fn cherry<'a>() -> Text<'a> {
    icon("\u{E350}")
}

pub fn chess_bishop<'a>() -> Text<'a> {
    icon("\u{E6A0}")
}

pub fn chess_king<'a>() -> Text<'a> {
    icon("\u{E6A1}")
}

pub fn chess_knight<'a>() -> Text<'a> {
    icon("\u{E6A2}")
}

pub fn chess_pawn<'a>() -> Text<'a> {
    icon("\u{E6A3}")
}

pub fn chess_queen<'a>() -> Text<'a> {
    icon("\u{E6A4}")
}

pub fn chess_rook<'a>() -> Text<'a> {
    icon("\u{E6A5}")
}

pub fn chevron_down<'a>() -> Text<'a> {
    icon("\u{E06D}")
}

pub fn chevron_first<'a>() -> Text<'a> {
    icon("\u{E243}")
}

pub fn chevron_last<'a>() -> Text<'a> {
    icon("\u{E244}")
}

pub fn chevron_left<'a>() -> Text<'a> {
    icon("\u{E06E}")
}

pub fn chevron_right<'a>() -> Text<'a> {
    icon("\u{E06F}")
}

pub fn chevron_up<'a>() -> Text<'a> {
    icon("\u{E070}")
}

pub fn chevrons_down<'a>() -> Text<'a> {
    icon("\u{E071}")
}

pub fn chevrons_down_up<'a>() -> Text<'a> {
    icon("\u{E228}")
}

pub fn chevrons_left<'a>() -> Text<'a> {
    icon("\u{E072}")
}

pub fn chevrons_left_right<'a>() -> Text<'a> {
    icon("\u{E293}")
}

pub fn chevrons_left_right_ellipsis<'a>() -> Text<'a> {
    icon("\u{E61F}")
}

pub fn chevrons_right<'a>() -> Text<'a> {
    icon("\u{E073}")
}

pub fn chevrons_right_left<'a>() -> Text<'a> {
    icon("\u{E294}")
}

pub fn chevrons_up<'a>() -> Text<'a> {
    icon("\u{E074}")
}

pub fn chevrons_up_down<'a>() -> Text<'a> {
    icon("\u{E211}")
}

pub fn chromium<'a>() -> Text<'a> {
    icon("\u{E075}")
}

pub fn church<'a>() -> Text<'a> {
    icon("\u{E3E1}")
}

pub fn cigarette<'a>() -> Text<'a> {
    icon("\u{E2C6}")
}

pub fn cigarette_off<'a>() -> Text<'a> {
    icon("\u{E2C7}")
}

pub fn circle<'a>() -> Text<'a> {
    icon("\u{E076}")
}

pub fn circle_alert<'a>() -> Text<'a> {
    icon("\u{E077}")
}

pub fn circle_arrow_down<'a>() -> Text<'a> {
    icon("\u{E078}")
}

pub fn circle_arrow_left<'a>() -> Text<'a> {
    icon("\u{E079}")
}

pub fn circle_arrow_out_down_left<'a>() -> Text<'a> {
    icon("\u{E3F7}")
}

pub fn circle_arrow_out_down_right<'a>() -> Text<'a> {
    icon("\u{E3F8}")
}

pub fn circle_arrow_out_up_left<'a>() -> Text<'a> {
    icon("\u{E3F9}")
}

pub fn circle_arrow_out_up_right<'a>() -> Text<'a> {
    icon("\u{E3FA}")
}

pub fn circle_arrow_right<'a>() -> Text<'a> {
    icon("\u{E07A}")
}

pub fn circle_arrow_up<'a>() -> Text<'a> {
    icon("\u{E07B}")
}

pub fn circle_check<'a>() -> Text<'a> {
    icon("\u{E226}")
}

pub fn circle_check_big<'a>() -> Text<'a> {
    icon("\u{E07C}")
}

pub fn circle_chevron_down<'a>() -> Text<'a> {
    icon("\u{E4DD}")
}

pub fn circle_chevron_left<'a>() -> Text<'a> {
    icon("\u{E4DE}")
}

pub fn circle_chevron_right<'a>() -> Text<'a> {
    icon("\u{E4DF}")
}

pub fn circle_chevron_up<'a>() -> Text<'a> {
    icon("\u{E4E0}")
}

pub fn circle_dashed<'a>() -> Text<'a> {
    icon("\u{E4B0}")
}

pub fn circle_divide<'a>() -> Text<'a> {
    icon("\u{E07D}")
}

pub fn circle_dollar_sign<'a>() -> Text<'a> {
    icon("\u{E47D}")
}

pub fn circle_dot<'a>() -> Text<'a> {
    icon("\u{E345}")
}

pub fn circle_dot_dashed<'a>() -> Text<'a> {
    icon("\u{E4B1}")
}

pub fn circle_ellipsis<'a>() -> Text<'a> {
    icon("\u{E346}")
}

pub fn circle_equal<'a>() -> Text<'a> {
    icon("\u{E400}")
}

pub fn circle_fading_arrow_up<'a>() -> Text<'a> {
    icon("\u{E618}")
}

pub fn circle_fading_plus<'a>() -> Text<'a> {
    icon("\u{E5BC}")
}

pub fn circle_gauge<'a>() -> Text<'a> {
    icon("\u{E4E1}")
}

pub fn circle_minus<'a>() -> Text<'a> {
    icon("\u{E07E}")
}

pub fn circle_off<'a>() -> Text<'a> {
    icon("\u{E401}")
}

pub fn circle_parking<'a>() -> Text<'a> {
    icon("\u{E3C9}")
}

pub fn circle_parking_off<'a>() -> Text<'a> {
    icon("\u{E3CA}")
}

pub fn circle_pause<'a>() -> Text<'a> {
    icon("\u{E07F}")
}

pub fn circle_percent<'a>() -> Text<'a> {
    icon("\u{E51A}")
}

pub fn circle_pile<'a>() -> Text<'a> {
    icon("\u{E6B0}")
}

pub fn circle_play<'a>() -> Text<'a> {
    icon("\u{E080}")
}

pub fn circle_plus<'a>() -> Text<'a> {
    icon("\u{E081}")
}

pub fn circle_pound_sterling<'a>() -> Text<'a> {
    icon("\u{E66D}")
}

pub fn circle_power<'a>() -> Text<'a> {
    icon("\u{E550}")
}

pub fn circle_question_mark<'a>() -> Text<'a> {
    icon("\u{E082}")
}

pub fn circle_slash<'a>() -> Text<'a> {
    icon("\u{E402}")
}

pub fn circle_slash_2<'a>() -> Text<'a> {
    icon("\u{E213}")
}

pub fn circle_small<'a>() -> Text<'a> {
    icon("\u{E640}")
}

pub fn circle_star<'a>() -> Text<'a> {
    icon("\u{E68D}")
}

pub fn circle_stop<'a>() -> Text<'a> {
    icon("\u{E083}")
}

pub fn circle_user<'a>() -> Text<'a> {
    icon("\u{E461}")
}

pub fn circle_user_round<'a>() -> Text<'a> {
    icon("\u{E462}")
}

pub fn circle_x<'a>() -> Text<'a> {
    icon("\u{E084}")
}

pub fn circuit_board<'a>() -> Text<'a> {
    icon("\u{E403}")
}

pub fn citrus<'a>() -> Text<'a> {
    icon("\u{E375}")
}

pub fn clapperboard<'a>() -> Text<'a> {
    icon("\u{E29B}")
}

pub fn clipboard<'a>() -> Text<'a> {
    icon("\u{E085}")
}

pub fn clipboard_check<'a>() -> Text<'a> {
    icon("\u{E219}")
}

pub fn clipboard_clock<'a>() -> Text<'a> {
    icon("\u{E688}")
}

pub fn clipboard_copy<'a>() -> Text<'a> {
    icon("\u{E225}")
}

pub fn clipboard_list<'a>() -> Text<'a> {
    icon("\u{E086}")
}

pub fn clipboard_minus<'a>() -> Text<'a> {
    icon("\u{E5BE}")
}

pub fn clipboard_paste<'a>() -> Text<'a> {
    icon("\u{E3E8}")
}

pub fn clipboard_pen<'a>() -> Text<'a> {
    icon("\u{E307}")
}

pub fn clipboard_pen_line<'a>() -> Text<'a> {
    icon("\u{E308}")
}

pub fn clipboard_plus<'a>() -> Text<'a> {
    icon("\u{E5BF}")
}

pub fn clipboard_type<'a>() -> Text<'a> {
    icon("\u{E309}")
}

pub fn clipboard_x<'a>() -> Text<'a> {
    icon("\u{E222}")
}

pub fn clock<'a>() -> Text<'a> {
    icon("\u{E087}")
}

pub fn clock_1<'a>() -> Text<'a> {
    icon("\u{E24B}")
}

pub fn clock_10<'a>() -> Text<'a> {
    icon("\u{E24C}")
}

pub fn clock_11<'a>() -> Text<'a> {
    icon("\u{E24D}")
}

pub fn clock_12<'a>() -> Text<'a> {
    icon("\u{E24E}")
}

pub fn clock_2<'a>() -> Text<'a> {
    icon("\u{E24F}")
}

pub fn clock_3<'a>() -> Text<'a> {
    icon("\u{E250}")
}

pub fn clock_4<'a>() -> Text<'a> {
    icon("\u{E251}")
}

pub fn clock_5<'a>() -> Text<'a> {
    icon("\u{E252}")
}

pub fn clock_6<'a>() -> Text<'a> {
    icon("\u{E253}")
}

pub fn clock_7<'a>() -> Text<'a> {
    icon("\u{E254}")
}

pub fn clock_8<'a>() -> Text<'a> {
    icon("\u{E255}")
}

pub fn clock_9<'a>() -> Text<'a> {
    icon("\u{E256}")
}

pub fn clock_alert<'a>() -> Text<'a> {
    icon("\u{E62A}")
}

pub fn clock_arrow_down<'a>() -> Text<'a> {
    icon("\u{E600}")
}

pub fn clock_arrow_up<'a>() -> Text<'a> {
    icon("\u{E601}")
}

pub fn clock_check<'a>() -> Text<'a> {
    icon("\u{E69E}")
}

pub fn clock_fading<'a>() -> Text<'a> {
    icon("\u{E64A}")
}

pub fn clock_plus<'a>() -> Text<'a> {
    icon("\u{E667}")
}

pub fn closed_caption<'a>() -> Text<'a> {
    icon("\u{E68A}")
}

pub fn cloud<'a>() -> Text<'a> {
    icon("\u{E088}")
}

pub fn cloud_alert<'a>() -> Text<'a> {
    icon("\u{E633}")
}

pub fn cloud_backup<'a>() -> Text<'a> {
    icon("\u{E6B1}")
}

pub fn cloud_check<'a>() -> Text<'a> {
    icon("\u{E66E}")
}

pub fn cloud_cog<'a>() -> Text<'a> {
    icon("\u{E30A}")
}

pub fn cloud_download<'a>() -> Text<'a> {
    icon("\u{E089}")
}

pub fn cloud_drizzle<'a>() -> Text<'a> {
    icon("\u{E08A}")
}

pub fn cloud_fog<'a>() -> Text<'a> {
    icon("\u{E214}")
}

pub fn cloud_hail<'a>() -> Text<'a> {
    icon("\u{E08B}")
}

pub fn cloud_lightning<'a>() -> Text<'a> {
    icon("\u{E08C}")
}

pub fn cloud_moon<'a>() -> Text<'a> {
    icon("\u{E215}")
}

pub fn cloud_moon_rain<'a>() -> Text<'a> {
    icon("\u{E2FA}")
}

pub fn cloud_off<'a>() -> Text<'a> {
    icon("\u{E08D}")
}

pub fn cloud_rain<'a>() -> Text<'a> {
    icon("\u{E08E}")
}

pub fn cloud_rain_wind<'a>() -> Text<'a> {
    icon("\u{E08F}")
}

pub fn cloud_snow<'a>() -> Text<'a> {
    icon("\u{E090}")
}

pub fn cloud_sun<'a>() -> Text<'a> {
    icon("\u{E216}")
}

pub fn cloud_sun_rain<'a>() -> Text<'a> {
    icon("\u{E2FB}")
}

pub fn cloud_sync<'a>() -> Text<'a> {
    icon("\u{E6B2}")
}

pub fn cloud_upload<'a>() -> Text<'a> {
    icon("\u{E091}")
}

pub fn cloudy<'a>() -> Text<'a> {
    icon("\u{E217}")
}

pub fn clover<'a>() -> Text<'a> {
    icon("\u{E092}")
}

pub fn club<'a>() -> Text<'a> {
    icon("\u{E496}")
}

pub fn code<'a>() -> Text<'a> {
    icon("\u{E093}")
}

pub fn code_xml<'a>() -> Text<'a> {
    icon("\u{E206}")
}

pub fn codepen<'a>() -> Text<'a> {
    icon("\u{E094}")
}

pub fn codesandbox<'a>() -> Text<'a> {
    icon("\u{E095}")
}

pub fn coffee<'a>() -> Text<'a> {
    icon("\u{E096}")
}

pub fn cog<'a>() -> Text<'a> {
    icon("\u{E30B}")
}

pub fn coins<'a>() -> Text<'a> {
    icon("\u{E097}")
}

pub fn columns_2<'a>() -> Text<'a> {
    icon("\u{E098}")
}

pub fn columns_3<'a>() -> Text<'a> {
    icon("\u{E099}")
}

pub fn columns_3_cog<'a>() -> Text<'a> {
    icon("\u{E661}")
}

pub fn columns_4<'a>() -> Text<'a> {
    icon("\u{E589}")
}

pub fn combine<'a>() -> Text<'a> {
    icon("\u{E44C}")
}

pub fn command<'a>() -> Text<'a> {
    icon("\u{E09A}")
}

pub fn compass<'a>() -> Text<'a> {
    icon("\u{E09B}")
}

pub fn component<'a>() -> Text<'a> {
    icon("\u{E2AD}")
}

pub fn computer<'a>() -> Text<'a> {
    icon("\u{E4E4}")
}

pub fn concierge_bell<'a>() -> Text<'a> {
    icon("\u{E378}")
}

pub fn cone<'a>() -> Text<'a> {
    icon("\u{E523}")
}

pub fn construction<'a>() -> Text<'a> {
    icon("\u{E3B4}")
}

pub fn contact<'a>() -> Text<'a> {
    icon("\u{E09C}")
}

pub fn contact_round<'a>() -> Text<'a> {
    icon("\u{E463}")
}

pub fn container<'a>() -> Text<'a> {
    icon("\u{E4D5}")
}

pub fn contrast<'a>() -> Text<'a> {
    icon("\u{E09D}")
}

pub fn cookie<'a>() -> Text<'a> {
    icon("\u{E26B}")
}

pub fn cooking_pot<'a>() -> Text<'a> {
    icon("\u{E584}")
}

pub fn copy<'a>() -> Text<'a> {
    icon("\u{E09E}")
}

pub fn copy_check<'a>() -> Text<'a> {
    icon("\u{E3FB}")
}

pub fn copy_minus<'a>() -> Text<'a> {
    icon("\u{E3FC}")
}

pub fn copy_plus<'a>() -> Text<'a> {
    icon("\u{E3FD}")
}

pub fn copy_slash<'a>() -> Text<'a> {
    icon("\u{E3FE}")
}

pub fn copy_x<'a>() -> Text<'a> {
    icon("\u{E3FF}")
}

pub fn copyleft<'a>() -> Text<'a> {
    icon("\u{E09F}")
}

pub fn copyright<'a>() -> Text<'a> {
    icon("\u{E0A0}")
}

pub fn corner_down_left<'a>() -> Text<'a> {
    icon("\u{E0A1}")
}

pub fn corner_down_right<'a>() -> Text<'a> {
    icon("\u{E0A2}")
}

pub fn corner_left_down<'a>() -> Text<'a> {
    icon("\u{E0A3}")
}

pub fn corner_left_up<'a>() -> Text<'a> {
    icon("\u{E0A4}")
}

pub fn corner_right_down<'a>() -> Text<'a> {
    icon("\u{E0A5}")
}

pub fn corner_right_up<'a>() -> Text<'a> {
    icon("\u{E0A6}")
}

pub fn corner_up_left<'a>() -> Text<'a> {
    icon("\u{E0A7}")
}

pub fn corner_up_right<'a>() -> Text<'a> {
    icon("\u{E0A8}")
}

pub fn cpu<'a>() -> Text<'a> {
    icon("\u{E0A9}")
}

pub fn creative_commons<'a>() -> Text<'a> {
    icon("\u{E3B2}")
}

pub fn credit_card<'a>() -> Text<'a> {
    icon("\u{E0AA}")
}

pub fn croissant<'a>() -> Text<'a> {
    icon("\u{E2AE}")
}

pub fn crop<'a>() -> Text<'a> {
    icon("\u{E0AB}")
}

pub fn cross<'a>() -> Text<'a> {
    icon("\u{E1E5}")
}

pub fn crosshair<'a>() -> Text<'a> {
    icon("\u{E0AC}")
}

pub fn crown<'a>() -> Text<'a> {
    icon("\u{E1D6}")
}

pub fn cuboid<'a>() -> Text<'a> {
    icon("\u{E524}")
}

pub fn cup_soda<'a>() -> Text<'a> {
    icon("\u{E2D1}")
}

pub fn currency<'a>() -> Text<'a> {
    icon("\u{E230}")
}

pub fn cylinder<'a>() -> Text<'a> {
    icon("\u{E525}")
}

pub fn dam<'a>() -> Text<'a> {
    icon("\u{E606}")
}

pub fn database<'a>() -> Text<'a> {
    icon("\u{E0AD}")
}

pub fn database_backup<'a>() -> Text<'a> {
    icon("\u{E3AB}")
}

pub fn database_search<'a>() -> Text<'a> {
    icon("\u{E6BC}")
}

pub fn database_zap<'a>() -> Text<'a> {
    icon("\u{E50B}")
}

pub fn decimals_arrow_left<'a>() -> Text<'a> {
    icon("\u{E65C}")
}

pub fn decimals_arrow_right<'a>() -> Text<'a> {
    icon("\u{E65D}")
}

pub fn delete<'a>() -> Text<'a> {
    icon("\u{E0AE}")
}

pub fn dessert<'a>() -> Text<'a> {
    icon("\u{E4BB}")
}

pub fn diameter<'a>() -> Text<'a> {
    icon("\u{E526}")
}

pub fn diamond<'a>() -> Text<'a> {
    icon("\u{E2D2}")
}

pub fn diamond_minus<'a>() -> Text<'a> {
    icon("\u{E5E1}")
}

pub fn diamond_percent<'a>() -> Text<'a> {
    icon("\u{E51B}")
}

pub fn diamond_plus<'a>() -> Text<'a> {
    icon("\u{E5E2}")
}

pub fn dice_1<'a>() -> Text<'a> {
    icon("\u{E287}")
}

pub fn dice_2<'a>() -> Text<'a> {
    icon("\u{E288}")
}

pub fn dice_3<'a>() -> Text<'a> {
    icon("\u{E289}")
}

pub fn dice_4<'a>() -> Text<'a> {
    icon("\u{E28A}")
}

pub fn dice_5<'a>() -> Text<'a> {
    icon("\u{E28B}")
}

pub fn dice_6<'a>() -> Text<'a> {
    icon("\u{E28C}")
}

pub fn dices<'a>() -> Text<'a> {
    icon("\u{E2C5}")
}

pub fn diff<'a>() -> Text<'a> {
    icon("\u{E30C}")
}

pub fn disc<'a>() -> Text<'a> {
    icon("\u{E0AF}")
}

pub fn disc_2<'a>() -> Text<'a> {
    icon("\u{E3F6}")
}

pub fn disc_3<'a>() -> Text<'a> {
    icon("\u{E494}")
}

pub fn disc_album<'a>() -> Text<'a> {
    icon("\u{E55C}")
}

pub fn divide<'a>() -> Text<'a> {
    icon("\u{E0B0}")
}

pub fn dna<'a>() -> Text<'a> {
    icon("\u{E393}")
}

pub fn dna_off<'a>() -> Text<'a> {
    icon("\u{E394}")
}

pub fn dock<'a>() -> Text<'a> {
    icon("\u{E5D3}")
}

pub fn dog<'a>() -> Text<'a> {
    icon("\u{E38D}")
}

pub fn dollar_sign<'a>() -> Text<'a> {
    icon("\u{E0B1}")
}

pub fn donut<'a>() -> Text<'a> {
    icon("\u{E4BC}")
}

pub fn door_closed<'a>() -> Text<'a> {
    icon("\u{E3D5}")
}

pub fn door_closed_locked<'a>() -> Text<'a> {
    icon("\u{E664}")
}

pub fn door_open<'a>() -> Text<'a> {
    icon("\u{E3D6}")
}

pub fn dot<'a>() -> Text<'a> {
    icon("\u{E44F}")
}

pub fn download<'a>() -> Text<'a> {
    icon("\u{E0B2}")
}

pub fn drafting_compass<'a>() -> Text<'a> {
    icon("\u{E527}")
}

pub fn drama<'a>() -> Text<'a> {
    icon("\u{E521}")
}

pub fn dribbble<'a>() -> Text<'a> {
    icon("\u{E0B3}")
}

pub fn drill<'a>() -> Text<'a> {
    icon("\u{E58D}")
}

pub fn drone<'a>() -> Text<'a> {
    icon("\u{E676}")
}

pub fn droplet<'a>() -> Text<'a> {
    icon("\u{E0B4}")
}

pub fn droplet_off<'a>() -> Text<'a> {
    icon("\u{E638}")
}

pub fn droplets<'a>() -> Text<'a> {
    icon("\u{E0B5}")
}

pub fn drum<'a>() -> Text<'a> {
    icon("\u{E55D}")
}

pub fn drumstick<'a>() -> Text<'a> {
    icon("\u{E25B}")
}

pub fn dumbbell<'a>() -> Text<'a> {
    icon("\u{E3A1}")
}

pub fn ear<'a>() -> Text<'a> {
    icon("\u{E382}")
}

pub fn ear_off<'a>() -> Text<'a> {
    icon("\u{E383}")
}

pub fn earth<'a>() -> Text<'a> {
    icon("\u{E1F3}")
}

pub fn earth_lock<'a>() -> Text<'a> {
    icon("\u{E5CC}")
}

pub fn eclipse<'a>() -> Text<'a> {
    icon("\u{E59D}")
}

pub fn egg<'a>() -> Text<'a> {
    icon("\u{E25D}")
}

pub fn egg_fried<'a>() -> Text<'a> {
    icon("\u{E351}")
}

pub fn egg_off<'a>() -> Text<'a> {
    icon("\u{E395}")
}

pub fn ellipsis<'a>() -> Text<'a> {
    icon("\u{E0B6}")
}

pub fn ellipsis_vertical<'a>() -> Text<'a> {
    icon("\u{E0B7}")
}

pub fn equal<'a>() -> Text<'a> {
    icon("\u{E1BD}")
}

pub fn equal_approximately<'a>() -> Text<'a> {
    icon("\u{E634}")
}

pub fn equal_not<'a>() -> Text<'a> {
    icon("\u{E1BE}")
}

pub fn eraser<'a>() -> Text<'a> {
    icon("\u{E28F}")
}

pub fn ethernet_port<'a>() -> Text<'a> {
    icon("\u{E620}")
}

pub fn euro<'a>() -> Text<'a> {
    icon("\u{E0B8}")
}

pub fn ev_charger<'a>() -> Text<'a> {
    icon("\u{E697}")
}

pub fn expand<'a>() -> Text<'a> {
    icon("\u{E21A}")
}

pub fn external_link<'a>() -> Text<'a> {
    icon("\u{E0B9}")
}

pub fn eye<'a>() -> Text<'a> {
    icon("\u{E0BA}")
}

pub fn eye_closed<'a>() -> Text<'a> {
    icon("\u{E62E}")
}

pub fn eye_off<'a>() -> Text<'a> {
    icon("\u{E0BB}")
}

pub fn facebook<'a>() -> Text<'a> {
    icon("\u{E0BC}")
}

pub fn factory<'a>() -> Text<'a> {
    icon("\u{E29F}")
}

pub fn fan<'a>() -> Text<'a> {
    icon("\u{E379}")
}

pub fn fast_forward<'a>() -> Text<'a> {
    icon("\u{E0BD}")
}

pub fn feather<'a>() -> Text<'a> {
    icon("\u{E0BE}")
}

pub fn fence<'a>() -> Text<'a> {
    icon("\u{E582}")
}

pub fn ferris_wheel<'a>() -> Text<'a> {
    icon("\u{E47F}")
}

pub fn figma<'a>() -> Text<'a> {
    icon("\u{E0BF}")
}

pub fn file<'a>() -> Text<'a> {
    icon("\u{E0C0}")
}

pub fn file_archive<'a>() -> Text<'a> {
    icon("\u{E30D}")
}

pub fn file_axis_3d<'a>() -> Text<'a> {
    icon("\u{E30E}")
}

pub fn file_badge<'a>() -> Text<'a> {
    icon("\u{E30F}")
}

pub fn file_box<'a>() -> Text<'a> {
    icon("\u{E310}")
}

pub fn file_braces<'a>() -> Text<'a> {
    icon("\u{E36B}")
}

pub fn file_braces_corner<'a>() -> Text<'a> {
    icon("\u{E36C}")
}

pub fn file_chart_column<'a>() -> Text<'a> {
    icon("\u{E311}")
}

pub fn file_chart_column_increasing<'a>() -> Text<'a> {
    icon("\u{E312}")
}

pub fn file_chart_line<'a>() -> Text<'a> {
    icon("\u{E313}")
}

pub fn file_chart_pie<'a>() -> Text<'a> {
    icon("\u{E314}")
}

pub fn file_check<'a>() -> Text<'a> {
    icon("\u{E0C1}")
}

pub fn file_check_corner<'a>() -> Text<'a> {
    icon("\u{E0C2}")
}

pub fn file_clock<'a>() -> Text<'a> {
    icon("\u{E315}")
}

pub fn file_code<'a>() -> Text<'a> {
    icon("\u{E0C3}")
}

pub fn file_code_corner<'a>() -> Text<'a> {
    icon("\u{E45E}")
}

pub fn file_cog<'a>() -> Text<'a> {
    icon("\u{E316}")
}

pub fn file_diff<'a>() -> Text<'a> {
    icon("\u{E317}")
}

pub fn file_digit<'a>() -> Text<'a> {
    icon("\u{E0C4}")
}

pub fn file_down<'a>() -> Text<'a> {
    icon("\u{E318}")
}

pub fn file_exclamation_point<'a>() -> Text<'a> {
    icon("\u{E319}")
}

pub fn file_headphone<'a>() -> Text<'a> {
    icon("\u{E31A}")
}

pub fn file_heart<'a>() -> Text<'a> {
    icon("\u{E31B}")
}

pub fn file_image<'a>() -> Text<'a> {
    icon("\u{E31C}")
}

pub fn file_input<'a>() -> Text<'a> {
    icon("\u{E0C5}")
}

pub fn file_key<'a>() -> Text<'a> {
    icon("\u{E31D}")
}

pub fn file_lock<'a>() -> Text<'a> {
    icon("\u{E31E}")
}

pub fn file_minus<'a>() -> Text<'a> {
    icon("\u{E0C6}")
}

pub fn file_minus_corner<'a>() -> Text<'a> {
    icon("\u{E0C7}")
}

pub fn file_music<'a>() -> Text<'a> {
    icon("\u{E55E}")
}

pub fn file_output<'a>() -> Text<'a> {
    icon("\u{E0C8}")
}

pub fn file_pen<'a>() -> Text<'a> {
    icon("\u{E31F}")
}

pub fn file_pen_line<'a>() -> Text<'a> {
    icon("\u{E320}")
}

pub fn file_play<'a>() -> Text<'a> {
    icon("\u{E321}")
}

pub fn file_plus<'a>() -> Text<'a> {
    icon("\u{E0C9}")
}

pub fn file_plus_corner<'a>() -> Text<'a> {
    icon("\u{E0CA}")
}

pub fn file_question_mark<'a>() -> Text<'a> {
    icon("\u{E322}")
}

pub fn file_scan<'a>() -> Text<'a> {
    icon("\u{E323}")
}

pub fn file_search<'a>() -> Text<'a> {
    icon("\u{E0CB}")
}

pub fn file_search_corner<'a>() -> Text<'a> {
    icon("\u{E324}")
}

pub fn file_signal<'a>() -> Text<'a> {
    icon("\u{E325}")
}

pub fn file_sliders<'a>() -> Text<'a> {
    icon("\u{E5A0}")
}

pub fn file_spreadsheet<'a>() -> Text<'a> {
    icon("\u{E326}")
}

pub fn file_stack<'a>() -> Text<'a> {
    icon("\u{E4A1}")
}

pub fn file_symlink<'a>() -> Text<'a> {
    icon("\u{E327}")
}

pub fn file_terminal<'a>() -> Text<'a> {
    icon("\u{E328}")
}

pub fn file_text<'a>() -> Text<'a> {
    icon("\u{E0CC}")
}

pub fn file_type<'a>() -> Text<'a> {
    icon("\u{E329}")
}

pub fn file_type_corner<'a>() -> Text<'a> {
    icon("\u{E36D}")
}

pub fn file_up<'a>() -> Text<'a> {
    icon("\u{E32A}")
}

pub fn file_user<'a>() -> Text<'a> {
    icon("\u{E62D}")
}

pub fn file_video_camera<'a>() -> Text<'a> {
    icon("\u{E32B}")
}

pub fn file_volume<'a>() -> Text<'a> {
    icon("\u{E32C}")
}

pub fn file_x<'a>() -> Text<'a> {
    icon("\u{E0CD}")
}

pub fn file_x_corner<'a>() -> Text<'a> {
    icon("\u{E0CE}")
}

pub fn files<'a>() -> Text<'a> {
    icon("\u{E0CF}")
}

pub fn film<'a>() -> Text<'a> {
    icon("\u{E0D0}")
}

pub fn fingerprint_pattern<'a>() -> Text<'a> {
    icon("\u{E2CB}")
}

pub fn fire_extinguisher<'a>() -> Text<'a> {
    icon("\u{E57E}")
}

pub fn fish<'a>() -> Text<'a> {
    icon("\u{E3A6}")
}

pub fn fish_off<'a>() -> Text<'a> {
    icon("\u{E3B0}")
}

pub fn fish_symbol<'a>() -> Text<'a> {
    icon("\u{E4F4}")
}

pub fn fishing_hook<'a>() -> Text<'a> {
    icon("\u{E6B6}")
}

pub fn flag<'a>() -> Text<'a> {
    icon("\u{E0D1}")
}

pub fn flag_off<'a>() -> Text<'a> {
    icon("\u{E292}")
}

pub fn flag_triangle_left<'a>() -> Text<'a> {
    icon("\u{E237}")
}

pub fn flag_triangle_right<'a>() -> Text<'a> {
    icon("\u{E238}")
}

pub fn flame<'a>() -> Text<'a> {
    icon("\u{E0D2}")
}

pub fn flame_kindling<'a>() -> Text<'a> {
    icon("\u{E53A}")
}

pub fn flashlight<'a>() -> Text<'a> {
    icon("\u{E0D3}")
}

pub fn flashlight_off<'a>() -> Text<'a> {
    icon("\u{E0D4}")
}

pub fn flask_conical<'a>() -> Text<'a> {
    icon("\u{E0D5}")
}

pub fn flask_conical_off<'a>() -> Text<'a> {
    icon("\u{E396}")
}

pub fn flask_round<'a>() -> Text<'a> {
    icon("\u{E0D6}")
}

pub fn flip_horizontal_2<'a>() -> Text<'a> {
    icon("\u{E35D}")
}

pub fn flip_vertical_2<'a>() -> Text<'a> {
    icon("\u{E35E}")
}

pub fn flower<'a>() -> Text<'a> {
    icon("\u{E2D3}")
}

pub fn flower_2<'a>() -> Text<'a> {
    icon("\u{E2D4}")
}

pub fn focus<'a>() -> Text<'a> {
    icon("\u{E29E}")
}

pub fn fold_horizontal<'a>() -> Text<'a> {
    icon("\u{E43B}")
}

pub fn fold_vertical<'a>() -> Text<'a> {
    icon("\u{E43C}")
}

pub fn folder<'a>() -> Text<'a> {
    icon("\u{E0D7}")
}

pub fn folder_archive<'a>() -> Text<'a> {
    icon("\u{E32D}")
}

pub fn folder_check<'a>() -> Text<'a> {
    icon("\u{E32E}")
}

pub fn folder_clock<'a>() -> Text<'a> {
    icon("\u{E32F}")
}

pub fn folder_closed<'a>() -> Text<'a> {
    icon("\u{E330}")
}

pub fn folder_code<'a>() -> Text<'a> {
    icon("\u{E5FB}")
}

pub fn folder_cog<'a>() -> Text<'a> {
    icon("\u{E331}")
}

pub fn folder_dot<'a>() -> Text<'a> {
    icon("\u{E4C5}")
}

pub fn folder_down<'a>() -> Text<'a> {
    icon("\u{E332}")
}

pub fn folder_git<'a>() -> Text<'a> {
    icon("\u{E409}")
}

pub fn folder_git_2<'a>() -> Text<'a> {
    icon("\u{E40A}")
}

pub fn folder_heart<'a>() -> Text<'a> {
    icon("\u{E333}")
}

pub fn folder_input<'a>() -> Text<'a> {
    icon("\u{E334}")
}

pub fn folder_kanban<'a>() -> Text<'a> {
    icon("\u{E4C6}")
}

pub fn folder_key<'a>() -> Text<'a> {
    icon("\u{E335}")
}

pub fn folder_lock<'a>() -> Text<'a> {
    icon("\u{E336}")
}

pub fn folder_minus<'a>() -> Text<'a> {
    icon("\u{E0D8}")
}

pub fn folder_open<'a>() -> Text<'a> {
    icon("\u{E247}")
}

pub fn folder_open_dot<'a>() -> Text<'a> {
    icon("\u{E4C7}")
}

pub fn folder_output<'a>() -> Text<'a> {
    icon("\u{E337}")
}

pub fn folder_pen<'a>() -> Text<'a> {
    icon("\u{E338}")
}

pub fn folder_plus<'a>() -> Text<'a> {
    icon("\u{E0D9}")
}

pub fn folder_root<'a>() -> Text<'a> {
    icon("\u{E4C8}")
}

pub fn folder_search<'a>() -> Text<'a> {
    icon("\u{E339}")
}

pub fn folder_search_2<'a>() -> Text<'a> {
    icon("\u{E33A}")
}

pub fn folder_symlink<'a>() -> Text<'a> {
    icon("\u{E33B}")
}

pub fn folder_sync<'a>() -> Text<'a> {
    icon("\u{E4C9}")
}

pub fn folder_tree<'a>() -> Text<'a> {
    icon("\u{E33C}")
}

pub fn folder_up<'a>() -> Text<'a> {
    icon("\u{E33D}")
}

pub fn folder_x<'a>() -> Text<'a> {
    icon("\u{E33E}")
}

pub fn folders<'a>() -> Text<'a> {
    icon("\u{E33F}")
}

pub fn footprints<'a>() -> Text<'a> {
    icon("\u{E3B9}")
}

pub fn forklift<'a>() -> Text<'a> {
    icon("\u{E3C1}")
}

pub fn form<'a>() -> Text<'a> {
    icon("\u{E6A8}")
}

pub fn forward<'a>() -> Text<'a> {
    icon("\u{E229}")
}

pub fn frame<'a>() -> Text<'a> {
    icon("\u{E291}")
}

pub fn framer<'a>() -> Text<'a> {
    icon("\u{E0DA}")
}

pub fn frown<'a>() -> Text<'a> {
    icon("\u{E0DB}")
}

pub fn fuel<'a>() -> Text<'a> {
    icon("\u{E2AF}")
}

pub fn fullscreen<'a>() -> Text<'a> {
    icon("\u{E534}")
}

pub fn funnel<'a>() -> Text<'a> {
    icon("\u{E0DC}")
}

pub fn funnel_plus<'a>() -> Text<'a> {
    icon("\u{E0DD}")
}

pub fn funnel_x<'a>() -> Text<'a> {
    icon("\u{E3B5}")
}

pub fn gallery_horizontal<'a>() -> Text<'a> {
    icon("\u{E4CE}")
}

pub fn gallery_horizontal_end<'a>() -> Text<'a> {
    icon("\u{E4CF}")
}

pub fn gallery_thumbnails<'a>() -> Text<'a> {
    icon("\u{E4D0}")
}

pub fn gallery_vertical<'a>() -> Text<'a> {
    icon("\u{E4D1}")
}

pub fn gallery_vertical_end<'a>() -> Text<'a> {
    icon("\u{E4D2}")
}

pub fn gamepad<'a>() -> Text<'a> {
    icon("\u{E0DE}")
}

pub fn gamepad_2<'a>() -> Text<'a> {
    icon("\u{E0DF}")
}

pub fn gamepad_directional<'a>() -> Text<'a> {
    icon("\u{E69B}")
}

pub fn gauge<'a>() -> Text<'a> {
    icon("\u{E1BF}")
}

pub fn gavel<'a>() -> Text<'a> {
    icon("\u{E0E0}")
}

pub fn gem<'a>() -> Text<'a> {
    icon("\u{E242}")
}

pub fn georgian_lari<'a>() -> Text<'a> {
    icon("\u{E678}")
}

pub fn ghost<'a>() -> Text<'a> {
    icon("\u{E20E}")
}

pub fn gift<'a>() -> Text<'a> {
    icon("\u{E0E1}")
}

pub fn git_branch<'a>() -> Text<'a> {
    icon("\u{E0E2}")
}

pub fn git_branch_minus<'a>() -> Text<'a> {
    icon("\u{E69C}")
}

pub fn git_branch_plus<'a>() -> Text<'a> {
    icon("\u{E1F4}")
}

pub fn git_commit_horizontal<'a>() -> Text<'a> {
    icon("\u{E0E3}")
}

pub fn git_commit_vertical<'a>() -> Text<'a> {
    icon("\u{E552}")
}

pub fn git_compare<'a>() -> Text<'a> {
    icon("\u{E359}")
}

pub fn git_compare_arrows<'a>() -> Text<'a> {
    icon("\u{E553}")
}

pub fn git_fork<'a>() -> Text<'a> {
    icon("\u{E28D}")
}

pub fn git_graph<'a>() -> Text<'a> {
    icon("\u{E554}")
}

pub fn git_merge<'a>() -> Text<'a> {
    icon("\u{E0E4}")
}

pub fn git_merge_conflict<'a>() -> Text<'a> {
    icon("\u{E6C3}")
}

pub fn git_pull_request<'a>() -> Text<'a> {
    icon("\u{E0E5}")
}

pub fn git_pull_request_arrow<'a>() -> Text<'a> {
    icon("\u{E555}")
}

pub fn git_pull_request_closed<'a>() -> Text<'a> {
    icon("\u{E35A}")
}

pub fn git_pull_request_create<'a>() -> Text<'a> {
    icon("\u{E556}")
}

pub fn git_pull_request_create_arrow<'a>() -> Text<'a> {
    icon("\u{E557}")
}

pub fn git_pull_request_draft<'a>() -> Text<'a> {
    icon("\u{E35B}")
}

pub fn github<'a>() -> Text<'a> {
    icon("\u{E0E6}")
}

pub fn gitlab<'a>() -> Text<'a> {
    icon("\u{E0E7}")
}

pub fn glass_water<'a>() -> Text<'a> {
    icon("\u{E2D5}")
}

pub fn glasses<'a>() -> Text<'a> {
    icon("\u{E20D}")
}

pub fn globe<'a>() -> Text<'a> {
    icon("\u{E0E8}")
}

pub fn globe_lock<'a>() -> Text<'a> {
    icon("\u{E5CD}")
}

pub fn globe_off<'a>() -> Text<'a> {
    icon("\u{E6C1}")
}

pub fn globe_x<'a>() -> Text<'a> {
    icon("\u{E6BA}")
}

pub fn goal<'a>() -> Text<'a> {
    icon("\u{E4A5}")
}

pub fn gpu<'a>() -> Text<'a> {
    icon("\u{E66A}")
}

pub fn graduation_cap<'a>() -> Text<'a> {
    icon("\u{E234}")
}

pub fn grape<'a>() -> Text<'a> {
    icon("\u{E352}")
}

pub fn grid_2x2<'a>() -> Text<'a> {
    icon("\u{E4FF}")
}

pub fn grid_2x2_check<'a>() -> Text<'a> {
    icon("\u{E5E4}")
}

pub fn grid_2x2_plus<'a>() -> Text<'a> {
    icon("\u{E628}")
}

pub fn grid_2x2_x<'a>() -> Text<'a> {
    icon("\u{E5E5}")
}

pub fn grid_3x2<'a>() -> Text<'a> {
    icon("\u{E66F}")
}

pub fn grid_3x3<'a>() -> Text<'a> {
    icon("\u{E0E9}")
}

pub fn grip<'a>() -> Text<'a> {
    icon("\u{E3B1}")
}

pub fn grip_horizontal<'a>() -> Text<'a> {
    icon("\u{E0EA}")
}

pub fn grip_vertical<'a>() -> Text<'a> {
    icon("\u{E0EB}")
}

pub fn group<'a>() -> Text<'a> {
    icon("\u{E464}")
}

pub fn guitar<'a>() -> Text<'a> {
    icon("\u{E55F}")
}

pub fn ham<'a>() -> Text<'a> {
    icon("\u{E5D7}")
}

pub fn hamburger<'a>() -> Text<'a> {
    icon("\u{E665}")
}

pub fn hammer<'a>() -> Text<'a> {
    icon("\u{E0EC}")
}

pub fn hand<'a>() -> Text<'a> {
    icon("\u{E1D7}")
}

pub fn hand_coins<'a>() -> Text<'a> {
    icon("\u{E5B8}")
}

pub fn hand_fist<'a>() -> Text<'a> {
    icon("\u{E68B}")
}

pub fn hand_grab<'a>() -> Text<'a> {
    icon("\u{E1E6}")
}

pub fn hand_heart<'a>() -> Text<'a> {
    icon("\u{E5B9}")
}

pub fn hand_helping<'a>() -> Text<'a> {
    icon("\u{E3B8}")
}

pub fn hand_metal<'a>() -> Text<'a> {
    icon("\u{E22C}")
}

pub fn hand_platter<'a>() -> Text<'a> {
    icon("\u{E5BA}")
}

pub fn handbag<'a>() -> Text<'a> {
    icon("\u{E689}")
}

pub fn handshake<'a>() -> Text<'a> {
    icon("\u{E5C0}")
}

pub fn hard_drive<'a>() -> Text<'a> {
    icon("\u{E0ED}")
}

pub fn hard_drive_download<'a>() -> Text<'a> {
    icon("\u{E4E5}")
}

pub fn hard_drive_upload<'a>() -> Text<'a> {
    icon("\u{E4E6}")
}

pub fn hard_hat<'a>() -> Text<'a> {
    icon("\u{E0EE}")
}

pub fn hash<'a>() -> Text<'a> {
    icon("\u{E0EF}")
}

pub fn hat_glasses<'a>() -> Text<'a> {
    icon("\u{E683}")
}

pub fn haze<'a>() -> Text<'a> {
    icon("\u{E0F0}")
}

pub fn hd<'a>() -> Text<'a> {
    icon("\u{E6B5}")
}

pub fn hdmi_port<'a>() -> Text<'a> {
    icon("\u{E4E7}")
}

pub fn heading<'a>() -> Text<'a> {
    icon("\u{E384}")
}

pub fn heading_1<'a>() -> Text<'a> {
    icon("\u{E385}")
}

pub fn heading_2<'a>() -> Text<'a> {
    icon("\u{E386}")
}

pub fn heading_3<'a>() -> Text<'a> {
    icon("\u{E387}")
}

pub fn heading_4<'a>() -> Text<'a> {
    icon("\u{E388}")
}

pub fn heading_5<'a>() -> Text<'a> {
    icon("\u{E389}")
}

pub fn heading_6<'a>() -> Text<'a> {
    icon("\u{E38A}")
}

pub fn headphone_off<'a>() -> Text<'a> {
    icon("\u{E629}")
}

pub fn headphones<'a>() -> Text<'a> {
    icon("\u{E0F1}")
}

pub fn headset<'a>() -> Text<'a> {
    icon("\u{E5BD}")
}

pub fn heart<'a>() -> Text<'a> {
    icon("\u{E0F2}")
}

pub fn heart_crack<'a>() -> Text<'a> {
    icon("\u{E2D6}")
}

pub fn heart_handshake<'a>() -> Text<'a> {
    icon("\u{E2D7}")
}

pub fn heart_minus<'a>() -> Text<'a> {
    icon("\u{E651}")
}

pub fn heart_off<'a>() -> Text<'a> {
    icon("\u{E295}")
}

pub fn heart_plus<'a>() -> Text<'a> {
    icon("\u{E652}")
}

pub fn heart_pulse<'a>() -> Text<'a> {
    icon("\u{E36E}")
}

pub fn heater<'a>() -> Text<'a> {
    icon("\u{E58E}")
}

pub fn helicopter<'a>() -> Text<'a> {
    icon("\u{E69D}")
}

pub fn hexagon<'a>() -> Text<'a> {
    icon("\u{E0F3}")
}

pub fn highlighter<'a>() -> Text<'a> {
    icon("\u{E0F4}")
}

pub fn history<'a>() -> Text<'a> {
    icon("\u{E1F5}")
}

pub fn hop<'a>() -> Text<'a> {
    icon("\u{E397}")
}

pub fn hop_off<'a>() -> Text<'a> {
    icon("\u{E398}")
}

pub fn hospital<'a>() -> Text<'a> {
    icon("\u{E5D8}")
}

pub fn hotel<'a>() -> Text<'a> {
    icon("\u{E3E2}")
}

pub fn hourglass<'a>() -> Text<'a> {
    icon("\u{E296}")
}

pub fn house<'a>() -> Text<'a> {
    icon("\u{E0F5}")
}

pub fn house_heart<'a>() -> Text<'a> {
    icon("\u{E695}")
}

pub fn house_plug<'a>() -> Text<'a> {
    icon("\u{E5F0}")
}

pub fn house_plus<'a>() -> Text<'a> {
    icon("\u{E5F1}")
}

pub fn house_wifi<'a>() -> Text<'a> {
    icon("\u{E63C}")
}

pub fn ice_cream_bowl<'a>() -> Text<'a> {
    icon("\u{E3A7}")
}

pub fn ice_cream_cone<'a>() -> Text<'a> {
    icon("\u{E353}")
}

pub fn id_card<'a>() -> Text<'a> {
    icon("\u{E617}")
}

pub fn id_card_lanyard<'a>() -> Text<'a> {
    icon("\u{E670}")
}

pub fn image<'a>() -> Text<'a> {
    icon("\u{E0F6}")
}

pub fn image_down<'a>() -> Text<'a> {
    icon("\u{E53C}")
}

pub fn image_minus<'a>() -> Text<'a> {
    icon("\u{E1F6}")
}

pub fn image_off<'a>() -> Text<'a> {
    icon("\u{E1C0}")
}

pub fn image_play<'a>() -> Text<'a> {
    icon("\u{E5DF}")
}

pub fn image_plus<'a>() -> Text<'a> {
    icon("\u{E1F7}")
}

pub fn image_up<'a>() -> Text<'a> {
    icon("\u{E5CB}")
}

pub fn image_upscale<'a>() -> Text<'a> {
    icon("\u{E637}")
}

pub fn images<'a>() -> Text<'a> {
    icon("\u{E5C4}")
}

pub fn import<'a>() -> Text<'a> {
    icon("\u{E22F}")
}

pub fn inbox<'a>() -> Text<'a> {
    icon("\u{E0F7}")
}

pub fn indian_rupee<'a>() -> Text<'a> {
    icon("\u{E0F8}")
}

pub fn infinity<'a>() -> Text<'a> {
    icon("\u{E1E7}")
}

pub fn info<'a>() -> Text<'a> {
    icon("\u{E0F9}")
}

pub fn inspection_panel<'a>() -> Text<'a> {
    icon("\u{E583}")
}

pub fn instagram<'a>() -> Text<'a> {
    icon("\u{E0FA}")
}

pub fn italic<'a>() -> Text<'a> {
    icon("\u{E0FB}")
}

pub fn iteration_ccw<'a>() -> Text<'a> {
    icon("\u{E423}")
}

pub fn iteration_cw<'a>() -> Text<'a> {
    icon("\u{E424}")
}

pub fn japanese_yen<'a>() -> Text<'a> {
    icon("\u{E0FC}")
}

pub fn joystick<'a>() -> Text<'a> {
    icon("\u{E355}")
}

pub fn kanban<'a>() -> Text<'a> {
    icon("\u{E4DC}")
}

pub fn kayak<'a>() -> Text<'a> {
    icon("\u{E68F}")
}

pub fn key<'a>() -> Text<'a> {
    icon("\u{E0FD}")
}

pub fn key_round<'a>() -> Text<'a> {
    icon("\u{E4A3}")
}

pub fn key_square<'a>() -> Text<'a> {
    icon("\u{E4A4}")
}

pub fn keyboard<'a>() -> Text<'a> {
    icon("\u{E284}")
}

pub fn keyboard_music<'a>() -> Text<'a> {
    icon("\u{E560}")
}

pub fn keyboard_off<'a>() -> Text<'a> {
    icon("\u{E5DE}")
}

pub fn lamp<'a>() -> Text<'a> {
    icon("\u{E2D8}")
}

pub fn lamp_ceiling<'a>() -> Text<'a> {
    icon("\u{E2D9}")
}

pub fn lamp_desk<'a>() -> Text<'a> {
    icon("\u{E2DA}")
}

pub fn lamp_floor<'a>() -> Text<'a> {
    icon("\u{E2DB}")
}

pub fn lamp_wall_down<'a>() -> Text<'a> {
    icon("\u{E2DC}")
}

pub fn lamp_wall_up<'a>() -> Text<'a> {
    icon("\u{E2DD}")
}

pub fn land_plot<'a>() -> Text<'a> {
    icon("\u{E528}")
}

pub fn landmark<'a>() -> Text<'a> {
    icon("\u{E23A}")
}

pub fn languages<'a>() -> Text<'a> {
    icon("\u{E0FE}")
}

pub fn laptop<'a>() -> Text<'a> {
    icon("\u{E1CD}")
}

pub fn laptop_minimal<'a>() -> Text<'a> {
    icon("\u{E1D8}")
}

pub fn laptop_minimal_check<'a>() -> Text<'a> {
    icon("\u{E632}")
}

pub fn lasso<'a>() -> Text<'a> {
    icon("\u{E1CE}")
}

pub fn lasso_select<'a>() -> Text<'a> {
    icon("\u{E1CF}")
}

pub fn laugh<'a>() -> Text<'a> {
    icon("\u{E300}")
}

pub fn layers<'a>() -> Text<'a> {
    icon("\u{E529}")
}

pub fn layers_2<'a>() -> Text<'a> {
    icon("\u{E52A}")
}

pub fn layers_plus<'a>() -> Text<'a> {
    icon("\u{E6B3}")
}

pub fn layout_dashboard<'a>() -> Text<'a> {
    icon("\u{E1C1}")
}

pub fn layout_grid<'a>() -> Text<'a> {
    icon("\u{E0FF}")
}

pub fn layout_list<'a>() -> Text<'a> {
    icon("\u{E1D9}")
}

pub fn layout_panel_left<'a>() -> Text<'a> {
    icon("\u{E470}")
}

pub fn layout_panel_top<'a>() -> Text<'a> {
    icon("\u{E471}")
}

pub fn layout_template<'a>() -> Text<'a> {
    icon("\u{E207}")
}

pub fn leaf<'a>() -> Text<'a> {
    icon("\u{E2DE}")
}

pub fn leafy_green<'a>() -> Text<'a> {
    icon("\u{E46F}")
}

pub fn lectern<'a>() -> Text<'a> {
    icon("\u{E5E9}")
}

pub fn lens_concave<'a>() -> Text<'a> {
    icon("\u{E6BF}")
}

pub fn lens_convex<'a>() -> Text<'a> {
    icon("\u{E6C0}")
}

pub fn library<'a>() -> Text<'a> {
    icon("\u{E100}")
}

pub fn library_big<'a>() -> Text<'a> {
    icon("\u{E54E}")
}

pub fn life_buoy<'a>() -> Text<'a> {
    icon("\u{E101}")
}

pub fn ligature<'a>() -> Text<'a> {
    icon("\u{E43A}")
}

pub fn lightbulb<'a>() -> Text<'a> {
    icon("\u{E1C2}")
}

pub fn lightbulb_off<'a>() -> Text<'a> {
    icon("\u{E208}")
}

pub fn line_dot_right_horizontal<'a>() -> Text<'a> {
    icon("\u{E6C2}")
}

pub fn line_squiggle<'a>() -> Text<'a> {
    icon("\u{E67A}")
}

pub fn link<'a>() -> Text<'a> {
    icon("\u{E102}")
}

pub fn link_2<'a>() -> Text<'a> {
    icon("\u{E103}")
}

pub fn link_2_off<'a>() -> Text<'a> {
    icon("\u{E104}")
}

pub fn linkedin<'a>() -> Text<'a> {
    icon("\u{E105}")
}

pub fn list<'a>() -> Text<'a> {
    icon("\u{E106}")
}

pub fn list_check<'a>() -> Text<'a> {
    icon("\u{E5FA}")
}

pub fn list_checks<'a>() -> Text<'a> {
    icon("\u{E1D0}")
}

pub fn list_chevrons_down_up<'a>() -> Text<'a> {
    icon("\u{E694}")
}

pub fn list_chevrons_up_down<'a>() -> Text<'a> {
    icon("\u{E696}")
}

pub fn list_collapse<'a>() -> Text<'a> {
    icon("\u{E59B}")
}

pub fn list_end<'a>() -> Text<'a> {
    icon("\u{E2DF}")
}

pub fn list_filter<'a>() -> Text<'a> {
    icon("\u{E460}")
}

pub fn list_filter_plus<'a>() -> Text<'a> {
    icon("\u{E639}")
}

pub fn list_indent_decrease<'a>() -> Text<'a> {
    icon("\u{E107}")
}

pub fn list_indent_increase<'a>() -> Text<'a> {
    icon("\u{E108}")
}

pub fn list_minus<'a>() -> Text<'a> {
    icon("\u{E23E}")
}

pub fn list_music<'a>() -> Text<'a> {
    icon("\u{E2E0}")
}

pub fn list_ordered<'a>() -> Text<'a> {
    icon("\u{E1D1}")
}

pub fn list_plus<'a>() -> Text<'a> {
    icon("\u{E23F}")
}

pub fn list_restart<'a>() -> Text<'a> {
    icon("\u{E452}")
}

pub fn list_start<'a>() -> Text<'a> {
    icon("\u{E2E1}")
}

pub fn list_todo<'a>() -> Text<'a> {
    icon("\u{E4C3}")
}

pub fn list_tree<'a>() -> Text<'a> {
    icon("\u{E408}")
}

pub fn list_video<'a>() -> Text<'a> {
    icon("\u{E2E2}")
}

pub fn list_x<'a>() -> Text<'a> {
    icon("\u{E240}")
}

pub fn loader<'a>() -> Text<'a> {
    icon("\u{E109}")
}

pub fn loader_circle<'a>() -> Text<'a> {
    icon("\u{E10A}")
}

pub fn loader_pinwheel<'a>() -> Text<'a> {
    icon("\u{E5E6}")
}

pub fn locate<'a>() -> Text<'a> {
    icon("\u{E1DA}")
}

pub fn locate_fixed<'a>() -> Text<'a> {
    icon("\u{E1DB}")
}

pub fn locate_off<'a>() -> Text<'a> {
    icon("\u{E282}")
}

pub fn lock<'a>() -> Text<'a> {
    icon("\u{E10B}")
}

pub fn lock_keyhole<'a>() -> Text<'a> {
    icon("\u{E531}")
}

pub fn lock_keyhole_open<'a>() -> Text<'a> {
    icon("\u{E532}")
}

pub fn lock_open<'a>() -> Text<'a> {
    icon("\u{E10C}")
}

pub fn log_in<'a>() -> Text<'a> {
    icon("\u{E10D}")
}

pub fn log_out<'a>() -> Text<'a> {
    icon("\u{E10E}")
}

pub fn logs<'a>() -> Text<'a> {
    icon("\u{E5F4}")
}

pub fn lollipop<'a>() -> Text<'a> {
    icon("\u{E4BD}")
}

pub fn luggage<'a>() -> Text<'a> {
    icon("\u{E2CA}")
}

pub fn magnet<'a>() -> Text<'a> {
    icon("\u{E2B5}")
}

pub fn mail<'a>() -> Text<'a> {
    icon("\u{E10F}")
}

pub fn mail_check<'a>() -> Text<'a> {
    icon("\u{E361}")
}

pub fn mail_minus<'a>() -> Text<'a> {
    icon("\u{E362}")
}

pub fn mail_open<'a>() -> Text<'a> {
    icon("\u{E363}")
}

pub fn mail_plus<'a>() -> Text<'a> {
    icon("\u{E364}")
}

pub fn mail_question_mark<'a>() -> Text<'a> {
    icon("\u{E365}")
}

pub fn mail_search<'a>() -> Text<'a> {
    icon("\u{E366}")
}

pub fn mail_warning<'a>() -> Text<'a> {
    icon("\u{E367}")
}

pub fn mail_x<'a>() -> Text<'a> {
    icon("\u{E368}")
}

pub fn mailbox<'a>() -> Text<'a> {
    icon("\u{E3D4}")
}

pub fn mails<'a>() -> Text<'a> {
    icon("\u{E369}")
}

pub fn map<'a>() -> Text<'a> {
    icon("\u{E110}")
}

pub fn map_minus<'a>() -> Text<'a> {
    icon("\u{E686}")
}

pub fn map_pin<'a>() -> Text<'a> {
    icon("\u{E111}")
}

pub fn map_pin_check<'a>() -> Text<'a> {
    icon("\u{E60F}")
}

pub fn map_pin_check_inside<'a>() -> Text<'a> {
    icon("\u{E610}")
}

pub fn map_pin_house<'a>() -> Text<'a> {
    icon("\u{E61C}")
}

pub fn map_pin_minus<'a>() -> Text<'a> {
    icon("\u{E611}")
}

pub fn map_pin_minus_inside<'a>() -> Text<'a> {
    icon("\u{E612}")
}

pub fn map_pin_off<'a>() -> Text<'a> {
    icon("\u{E2A6}")
}

pub fn map_pin_pen<'a>() -> Text<'a> {
    icon("\u{E655}")
}

pub fn map_pin_plus<'a>() -> Text<'a> {
    icon("\u{E613}")
}

pub fn map_pin_plus_inside<'a>() -> Text<'a> {
    icon("\u{E614}")
}

pub fn map_pin_x<'a>() -> Text<'a> {
    icon("\u{E615}")
}

pub fn map_pin_x_inside<'a>() -> Text<'a> {
    icon("\u{E616}")
}

pub fn map_pinned<'a>() -> Text<'a> {
    icon("\u{E53D}")
}

pub fn map_plus<'a>() -> Text<'a> {
    icon("\u{E63F}")
}

pub fn mars<'a>() -> Text<'a> {
    icon("\u{E641}")
}

pub fn mars_stroke<'a>() -> Text<'a> {
    icon("\u{E642}")
}

pub fn martini<'a>() -> Text<'a> {
    icon("\u{E2E3}")
}

pub fn maximize<'a>() -> Text<'a> {
    icon("\u{E112}")
}

pub fn maximize_2<'a>() -> Text<'a> {
    icon("\u{E113}")
}

pub fn medal<'a>() -> Text<'a> {
    icon("\u{E36F}")
}

pub fn megaphone<'a>() -> Text<'a> {
    icon("\u{E235}")
}

pub fn megaphone_off<'a>() -> Text<'a> {
    icon("\u{E370}")
}

pub fn meh<'a>() -> Text<'a> {
    icon("\u{E114}")
}

pub fn memory_stick<'a>() -> Text<'a> {
    icon("\u{E445}")
}

pub fn menu<'a>() -> Text<'a> {
    icon("\u{E115}")
}

pub fn merge<'a>() -> Text<'a> {
    icon("\u{E43F}")
}

pub fn message_circle<'a>() -> Text<'a> {
    icon("\u{E116}")
}

pub fn message_circle_check<'a>() -> Text<'a> {
    icon("\u{E6C8}")
}

pub fn message_circle_code<'a>() -> Text<'a> {
    icon("\u{E562}")
}

pub fn message_circle_dashed<'a>() -> Text<'a> {
    icon("\u{E563}")
}

pub fn message_circle_heart<'a>() -> Text<'a> {
    icon("\u{E564}")
}

pub fn message_circle_more<'a>() -> Text<'a> {
    icon("\u{E565}")
}

pub fn message_circle_off<'a>() -> Text<'a> {
    icon("\u{E566}")
}

pub fn message_circle_plus<'a>() -> Text<'a> {
    icon("\u{E567}")
}

pub fn message_circle_question_mark<'a>() -> Text<'a> {
    icon("\u{E568}")
}

pub fn message_circle_reply<'a>() -> Text<'a> {
    icon("\u{E569}")
}

pub fn message_circle_warning<'a>() -> Text<'a> {
    icon("\u{E56A}")
}

pub fn message_circle_x<'a>() -> Text<'a> {
    icon("\u{E56B}")
}

pub fn message_square<'a>() -> Text<'a> {
    icon("\u{E117}")
}

pub fn message_square_check<'a>() -> Text<'a> {
    icon("\u{E6CC}")
}

pub fn message_square_code<'a>() -> Text<'a> {
    icon("\u{E56C}")
}

pub fn message_square_dashed<'a>() -> Text<'a> {
    icon("\u{E40B}")
}

pub fn message_square_diff<'a>() -> Text<'a> {
    icon("\u{E56D}")
}

pub fn message_square_dot<'a>() -> Text<'a> {
    icon("\u{E56E}")
}

pub fn message_square_heart<'a>() -> Text<'a> {
    icon("\u{E56F}")
}

pub fn message_square_lock<'a>() -> Text<'a> {
    icon("\u{E62C}")
}

pub fn message_square_more<'a>() -> Text<'a> {
    icon("\u{E570}")
}

pub fn message_square_off<'a>() -> Text<'a> {
    icon("\u{E571}")
}

pub fn message_square_plus<'a>() -> Text<'a> {
    icon("\u{E40C}")
}

pub fn message_square_quote<'a>() -> Text<'a> {
    icon("\u{E572}")
}

pub fn message_square_reply<'a>() -> Text<'a> {
    icon("\u{E573}")
}

pub fn message_square_share<'a>() -> Text<'a> {
    icon("\u{E574}")
}

pub fn message_square_text<'a>() -> Text<'a> {
    icon("\u{E575}")
}

pub fn message_square_warning<'a>() -> Text<'a> {
    icon("\u{E576}")
}

pub fn message_square_x<'a>() -> Text<'a> {
    icon("\u{E577}")
}

pub fn messages_square<'a>() -> Text<'a> {
    icon("\u{E40D}")
}

pub fn metronome<'a>() -> Text<'a> {
    icon("\u{E6CD}")
}

pub fn mic<'a>() -> Text<'a> {
    icon("\u{E118}")
}

pub fn mic_off<'a>() -> Text<'a> {
    icon("\u{E119}")
}

pub fn mic_vocal<'a>() -> Text<'a> {
    icon("\u{E349}")
}

pub fn microchip<'a>() -> Text<'a> {
    icon("\u{E61A}")
}

pub fn microscope<'a>() -> Text<'a> {
    icon("\u{E2E4}")
}

pub fn microwave<'a>() -> Text<'a> {
    icon("\u{E37A}")
}

pub fn milestone<'a>() -> Text<'a> {
    icon("\u{E298}")
}

pub fn milk<'a>() -> Text<'a> {
    icon("\u{E399}")
}

pub fn milk_off<'a>() -> Text<'a> {
    icon("\u{E39A}")
}

pub fn minimize<'a>() -> Text<'a> {
    icon("\u{E11A}")
}

pub fn minimize_2<'a>() -> Text<'a> {
    icon("\u{E11B}")
}

pub fn minus<'a>() -> Text<'a> {
    icon("\u{E11C}")
}

pub fn mirror_rectangular<'a>() -> Text<'a> {
    icon("\u{E6C4}")
}

pub fn mirror_round<'a>() -> Text<'a> {
    icon("\u{E6C5}")
}

pub fn monitor<'a>() -> Text<'a> {
    icon("\u{E11D}")
}

pub fn monitor_check<'a>() -> Text<'a> {
    icon("\u{E482}")
}

pub fn monitor_cloud<'a>() -> Text<'a> {
    icon("\u{E699}")
}

pub fn monitor_cog<'a>() -> Text<'a> {
    icon("\u{E603}")
}

pub fn monitor_dot<'a>() -> Text<'a> {
    icon("\u{E483}")
}

pub fn monitor_down<'a>() -> Text<'a> {
    icon("\u{E421}")
}

pub fn monitor_off<'a>() -> Text<'a> {
    icon("\u{E1DC}")
}

pub fn monitor_pause<'a>() -> Text<'a> {
    icon("\u{E484}")
}

pub fn monitor_play<'a>() -> Text<'a> {
    icon("\u{E485}")
}

pub fn monitor_smartphone<'a>() -> Text<'a> {
    icon("\u{E3A2}")
}

pub fn monitor_speaker<'a>() -> Text<'a> {
    icon("\u{E210}")
}

pub fn monitor_stop<'a>() -> Text<'a> {
    icon("\u{E486}")
}

pub fn monitor_up<'a>() -> Text<'a> {
    icon("\u{E422}")
}

pub fn monitor_x<'a>() -> Text<'a> {
    icon("\u{E487}")
}

pub fn moon<'a>() -> Text<'a> {
    icon("\u{E11E}")
}

pub fn moon_star<'a>() -> Text<'a> {
    icon("\u{E410}")
}

pub fn motorbike<'a>() -> Text<'a> {
    icon("\u{E698}")
}

pub fn mountain<'a>() -> Text<'a> {
    icon("\u{E231}")
}

pub fn mountain_snow<'a>() -> Text<'a> {
    icon("\u{E232}")
}

pub fn mouse<'a>() -> Text<'a> {
    icon("\u{E28E}")
}

pub fn mouse_left<'a>() -> Text<'a> {
    icon("\u{E6C9}")
}

pub fn mouse_off<'a>() -> Text<'a> {
    icon("\u{E5DB}")
}

pub fn mouse_pointer<'a>() -> Text<'a> {
    icon("\u{E11F}")
}

pub fn mouse_pointer_2<'a>() -> Text<'a> {
    icon("\u{E1C3}")
}

pub fn mouse_pointer_2_off<'a>() -> Text<'a> {
    icon("\u{E6A6}")
}

pub fn mouse_pointer_ban<'a>() -> Text<'a> {
    icon("\u{E5E7}")
}

pub fn mouse_pointer_click<'a>() -> Text<'a> {
    icon("\u{E120}")
}

pub fn mouse_right<'a>() -> Text<'a> {
    icon("\u{E6CA}")
}

pub fn move_3d<'a>() -> Text<'a> {
    icon("\u{E2E5}")
}

pub fn move_diagonal<'a>() -> Text<'a> {
    icon("\u{E1C4}")
}

pub fn move_diagonal_2<'a>() -> Text<'a> {
    icon("\u{E1C5}")
}

pub fn move_down<'a>() -> Text<'a> {
    icon("\u{E48C}")
}

pub fn move_down_left<'a>() -> Text<'a> {
    icon("\u{E48D}")
}

pub fn move_down_right<'a>() -> Text<'a> {
    icon("\u{E48E}")
}

pub fn move_horizontal<'a>() -> Text<'a> {
    icon("\u{E1C6}")
}

pub fn move_icon<'a>() -> Text<'a> {
    icon("\u{E121}")
}

pub fn move_left<'a>() -> Text<'a> {
    icon("\u{E48F}")
}

pub fn move_right<'a>() -> Text<'a> {
    icon("\u{E490}")
}

pub fn move_up<'a>() -> Text<'a> {
    icon("\u{E491}")
}

pub fn move_up_left<'a>() -> Text<'a> {
    icon("\u{E492}")
}

pub fn move_up_right<'a>() -> Text<'a> {
    icon("\u{E493}")
}

pub fn move_vertical<'a>() -> Text<'a> {
    icon("\u{E1C7}")
}

pub fn music<'a>() -> Text<'a> {
    icon("\u{E122}")
}

pub fn music_2<'a>() -> Text<'a> {
    icon("\u{E34A}")
}

pub fn music_3<'a>() -> Text<'a> {
    icon("\u{E34B}")
}

pub fn music_4<'a>() -> Text<'a> {
    icon("\u{E34C}")
}

pub fn navigation<'a>() -> Text<'a> {
    icon("\u{E123}")
}

pub fn navigation_2<'a>() -> Text<'a> {
    icon("\u{E124}")
}

pub fn navigation_2_off<'a>() -> Text<'a> {
    icon("\u{E2A7}")
}

pub fn navigation_off<'a>() -> Text<'a> {
    icon("\u{E2A8}")
}

pub fn network<'a>() -> Text<'a> {
    icon("\u{E125}")
}

pub fn newspaper<'a>() -> Text<'a> {
    icon("\u{E348}")
}

pub fn nfc<'a>() -> Text<'a> {
    icon("\u{E3C3}")
}

pub fn non_binary<'a>() -> Text<'a> {
    icon("\u{E643}")
}

pub fn notebook<'a>() -> Text<'a> {
    icon("\u{E595}")
}

pub fn notebook_pen<'a>() -> Text<'a> {
    icon("\u{E596}")
}

pub fn notebook_tabs<'a>() -> Text<'a> {
    icon("\u{E597}")
}

pub fn notebook_text<'a>() -> Text<'a> {
    icon("\u{E598}")
}

pub fn notepad_text<'a>() -> Text<'a> {
    icon("\u{E599}")
}

pub fn notepad_text_dashed<'a>() -> Text<'a> {
    icon("\u{E59A}")
}

pub fn nut<'a>() -> Text<'a> {
    icon("\u{E39B}")
}

pub fn nut_off<'a>() -> Text<'a> {
    icon("\u{E39C}")
}

pub fn octagon<'a>() -> Text<'a> {
    icon("\u{E126}")
}

pub fn octagon_alert<'a>() -> Text<'a> {
    icon("\u{E127}")
}

pub fn octagon_minus<'a>() -> Text<'a> {
    icon("\u{E627}")
}

pub fn octagon_pause<'a>() -> Text<'a> {
    icon("\u{E21B}")
}

pub fn octagon_x<'a>() -> Text<'a> {
    icon("\u{E128}")
}

pub fn omega<'a>() -> Text<'a> {
    icon("\u{E619}")
}

pub fn option<'a>() -> Text<'a> {
    icon("\u{E1F8}")
}

pub fn orbit<'a>() -> Text<'a> {
    icon("\u{E3E7}")
}

pub fn origami<'a>() -> Text<'a> {
    icon("\u{E5E3}")
}

pub fn package<'a>() -> Text<'a> {
    icon("\u{E129}")
}

pub fn package_2<'a>() -> Text<'a> {
    icon("\u{E340}")
}

pub fn package_check<'a>() -> Text<'a> {
    icon("\u{E266}")
}

pub fn package_minus<'a>() -> Text<'a> {
    icon("\u{E267}")
}

pub fn package_open<'a>() -> Text<'a> {
    icon("\u{E2CC}")
}

pub fn package_plus<'a>() -> Text<'a> {
    icon("\u{E268}")
}

pub fn package_search<'a>() -> Text<'a> {
    icon("\u{E269}")
}

pub fn package_x<'a>() -> Text<'a> {
    icon("\u{E26A}")
}

pub fn paint_bucket<'a>() -> Text<'a> {
    icon("\u{E2E6}")
}

pub fn paint_roller<'a>() -> Text<'a> {
    icon("\u{E59E}")
}

pub fn paintbrush<'a>() -> Text<'a> {
    icon("\u{E2E7}")
}

pub fn paintbrush_vertical<'a>() -> Text<'a> {
    icon("\u{E2E8}")
}

pub fn palette<'a>() -> Text<'a> {
    icon("\u{E1DD}")
}

pub fn panda<'a>() -> Text<'a> {
    icon("\u{E668}")
}

pub fn panel_bottom<'a>() -> Text<'a> {
    icon("\u{E42C}")
}

pub fn panel_bottom_close<'a>() -> Text<'a> {
    icon("\u{E42D}")
}

pub fn panel_bottom_dashed<'a>() -> Text<'a> {
    icon("\u{E42E}")
}

pub fn panel_bottom_open<'a>() -> Text<'a> {
    icon("\u{E42F}")
}

pub fn panel_left<'a>() -> Text<'a> {
    icon("\u{E12A}")
}

pub fn panel_left_close<'a>() -> Text<'a> {
    icon("\u{E21C}")
}

pub fn panel_left_dashed<'a>() -> Text<'a> {
    icon("\u{E430}")
}

pub fn panel_left_open<'a>() -> Text<'a> {
    icon("\u{E21D}")
}

pub fn panel_left_right_dashed<'a>() -> Text<'a> {
    icon("\u{E692}")
}

pub fn panel_right<'a>() -> Text<'a> {
    icon("\u{E431}")
}

pub fn panel_right_close<'a>() -> Text<'a> {
    icon("\u{E432}")
}

pub fn panel_right_dashed<'a>() -> Text<'a> {
    icon("\u{E433}")
}

pub fn panel_right_open<'a>() -> Text<'a> {
    icon("\u{E434}")
}

pub fn panel_top<'a>() -> Text<'a> {
    icon("\u{E435}")
}

pub fn panel_top_bottom_dashed<'a>() -> Text<'a> {
    icon("\u{E693}")
}

pub fn panel_top_close<'a>() -> Text<'a> {
    icon("\u{E436}")
}

pub fn panel_top_dashed<'a>() -> Text<'a> {
    icon("\u{E437}")
}

pub fn panel_top_open<'a>() -> Text<'a> {
    icon("\u{E438}")
}

pub fn panels_left_bottom<'a>() -> Text<'a> {
    icon("\u{E12B}")
}

pub fn panels_right_bottom<'a>() -> Text<'a> {
    icon("\u{E588}")
}

pub fn panels_top_left<'a>() -> Text<'a> {
    icon("\u{E12C}")
}

pub fn paperclip<'a>() -> Text<'a> {
    icon("\u{E12D}")
}

pub fn parentheses<'a>() -> Text<'a> {
    icon("\u{E444}")
}

pub fn parking_meter<'a>() -> Text<'a> {
    icon("\u{E500}")
}

pub fn party_popper<'a>() -> Text<'a> {
    icon("\u{E343}")
}

pub fn pause<'a>() -> Text<'a> {
    icon("\u{E12E}")
}

pub fn paw_print<'a>() -> Text<'a> {
    icon("\u{E4F5}")
}

pub fn pc_case<'a>() -> Text<'a> {
    icon("\u{E446}")
}

pub fn pen<'a>() -> Text<'a> {
    icon("\u{E12F}")
}

pub fn pen_line<'a>() -> Text<'a> {
    icon("\u{E130}")
}

pub fn pen_off<'a>() -> Text<'a> {
    icon("\u{E5EE}")
}

pub fn pen_tool<'a>() -> Text<'a> {
    icon("\u{E131}")
}

pub fn pencil<'a>() -> Text<'a> {
    icon("\u{E1F9}")
}

pub fn pencil_line<'a>() -> Text<'a> {
    icon("\u{E4F0}")
}

pub fn pencil_off<'a>() -> Text<'a> {
    icon("\u{E5EF}")
}

pub fn pencil_ruler<'a>() -> Text<'a> {
    icon("\u{E4F1}")
}

pub fn pentagon<'a>() -> Text<'a> {
    icon("\u{E52B}")
}

pub fn percent<'a>() -> Text<'a> {
    icon("\u{E132}")
}

pub fn person_standing<'a>() -> Text<'a> {
    icon("\u{E21E}")
}

pub fn philippine_peso<'a>() -> Text<'a> {
    icon("\u{E604}")
}

pub fn phone<'a>() -> Text<'a> {
    icon("\u{E133}")
}

pub fn phone_call<'a>() -> Text<'a> {
    icon("\u{E134}")
}

pub fn phone_forwarded<'a>() -> Text<'a> {
    icon("\u{E135}")
}

pub fn phone_incoming<'a>() -> Text<'a> {
    icon("\u{E136}")
}

pub fn phone_missed<'a>() -> Text<'a> {
    icon("\u{E137}")
}

pub fn phone_off<'a>() -> Text<'a> {
    icon("\u{E138}")
}

pub fn phone_outgoing<'a>() -> Text<'a> {
    icon("\u{E139}")
}

pub fn pi<'a>() -> Text<'a> {
    icon("\u{E472}")
}

pub fn piano<'a>() -> Text<'a> {
    icon("\u{E561}")
}

pub fn pickaxe<'a>() -> Text<'a> {
    icon("\u{E5C6}")
}

pub fn picture_in_picture<'a>() -> Text<'a> {
    icon("\u{E3AE}")
}

pub fn picture_in_picture_2<'a>() -> Text<'a> {
    icon("\u{E3AF}")
}

pub fn piggy_bank<'a>() -> Text<'a> {
    icon("\u{E13A}")
}

pub fn pilcrow<'a>() -> Text<'a> {
    icon("\u{E3A3}")
}

pub fn pilcrow_left<'a>() -> Text<'a> {
    icon("\u{E5DC}")
}

pub fn pilcrow_right<'a>() -> Text<'a> {
    icon("\u{E5DD}")
}

pub fn pill<'a>() -> Text<'a> {
    icon("\u{E3BD}")
}

pub fn pill_bottle<'a>() -> Text<'a> {
    icon("\u{E5EA}")
}

pub fn pin<'a>() -> Text<'a> {
    icon("\u{E259}")
}

pub fn pin_off<'a>() -> Text<'a> {
    icon("\u{E2B6}")
}

pub fn pipette<'a>() -> Text<'a> {
    icon("\u{E13B}")
}

pub fn pizza<'a>() -> Text<'a> {
    icon("\u{E354}")
}

pub fn plane<'a>() -> Text<'a> {
    icon("\u{E1DE}")
}

pub fn plane_landing<'a>() -> Text<'a> {
    icon("\u{E3CD}")
}

pub fn plane_takeoff<'a>() -> Text<'a> {
    icon("\u{E3CE}")
}

pub fn play<'a>() -> Text<'a> {
    icon("\u{E13C}")
}

pub fn plug<'a>() -> Text<'a> {
    icon("\u{E37F}")
}

pub fn plug_2<'a>() -> Text<'a> {
    icon("\u{E380}")
}

pub fn plug_zap<'a>() -> Text<'a> {
    icon("\u{E45C}")
}

pub fn plus<'a>() -> Text<'a> {
    icon("\u{E13D}")
}

pub fn pocket<'a>() -> Text<'a> {
    icon("\u{E13E}")
}

pub fn pocket_knife<'a>() -> Text<'a> {
    icon("\u{E4A0}")
}

pub fn podcast<'a>() -> Text<'a> {
    icon("\u{E1FA}")
}

pub fn pointer<'a>() -> Text<'a> {
    icon("\u{E1E8}")
}

pub fn pointer_off<'a>() -> Text<'a> {
    icon("\u{E57F}")
}

pub fn popcorn<'a>() -> Text<'a> {
    icon("\u{E4BE}")
}

pub fn popsicle<'a>() -> Text<'a> {
    icon("\u{E4BF}")
}

pub fn pound_sterling<'a>() -> Text<'a> {
    icon("\u{E13F}")
}

pub fn power<'a>() -> Text<'a> {
    icon("\u{E140}")
}

pub fn power_off<'a>() -> Text<'a> {
    icon("\u{E209}")
}

pub fn presentation<'a>() -> Text<'a> {
    icon("\u{E4AE}")
}

pub fn printer<'a>() -> Text<'a> {
    icon("\u{E141}")
}

pub fn printer_check<'a>() -> Text<'a> {
    icon("\u{E5F5}")
}

pub fn printer_x<'a>() -> Text<'a> {
    icon("\u{E6BB}")
}

pub fn projector<'a>() -> Text<'a> {
    icon("\u{E4AF}")
}

pub fn proportions<'a>() -> Text<'a> {
    icon("\u{E5CF}")
}

pub fn puzzle<'a>() -> Text<'a> {
    icon("\u{E29C}")
}

pub fn pyramid<'a>() -> Text<'a> {
    icon("\u{E52C}")
}

pub fn qr_code<'a>() -> Text<'a> {
    icon("\u{E1DF}")
}

pub fn quote<'a>() -> Text<'a> {
    icon("\u{E239}")
}

pub fn rabbit<'a>() -> Text<'a> {
    icon("\u{E4F6}")
}

pub fn radar<'a>() -> Text<'a> {
    icon("\u{E497}")
}

pub fn radiation<'a>() -> Text<'a> {
    icon("\u{E442}")
}

pub fn radical<'a>() -> Text<'a> {
    icon("\u{E5C2}")
}

pub fn radio<'a>() -> Text<'a> {
    icon("\u{E142}")
}

pub fn radio_receiver<'a>() -> Text<'a> {
    icon("\u{E1FB}")
}

pub fn radio_tower<'a>() -> Text<'a> {
    icon("\u{E404}")
}

pub fn radius<'a>() -> Text<'a> {
    icon("\u{E52D}")
}

pub fn rail_symbol<'a>() -> Text<'a> {
    icon("\u{E501}")
}

pub fn rainbow<'a>() -> Text<'a> {
    icon("\u{E4C2}")
}

pub fn rat<'a>() -> Text<'a> {
    icon("\u{E3EB}")
}

pub fn ratio<'a>() -> Text<'a> {
    icon("\u{E4E8}")
}

pub fn receipt<'a>() -> Text<'a> {
    icon("\u{E3D3}")
}

pub fn receipt_cent<'a>() -> Text<'a> {
    icon("\u{E5A5}")
}

pub fn receipt_euro<'a>() -> Text<'a> {
    icon("\u{E5A6}")
}

pub fn receipt_indian_rupee<'a>() -> Text<'a> {
    icon("\u{E5A7}")
}

pub fn receipt_japanese_yen<'a>() -> Text<'a> {
    icon("\u{E5A8}")
}

pub fn receipt_pound_sterling<'a>() -> Text<'a> {
    icon("\u{E5A9}")
}

pub fn receipt_russian_ruble<'a>() -> Text<'a> {
    icon("\u{E5AA}")
}

pub fn receipt_swiss_franc<'a>() -> Text<'a> {
    icon("\u{E5AB}")
}

pub fn receipt_text<'a>() -> Text<'a> {
    icon("\u{E5AC}")
}

pub fn receipt_turkish_lira<'a>() -> Text<'a> {
    icon("\u{E67F}")
}

pub fn rectangle_circle<'a>() -> Text<'a> {
    icon("\u{E673}")
}

pub fn rectangle_ellipsis<'a>() -> Text<'a> {
    icon("\u{E21F}")
}

pub fn rectangle_goggles<'a>() -> Text<'a> {
    icon("\u{E656}")
}

pub fn rectangle_horizontal<'a>() -> Text<'a> {
    icon("\u{E376}")
}

pub fn rectangle_vertical<'a>() -> Text<'a> {
    icon("\u{E377}")
}

pub fn recycle<'a>() -> Text<'a> {
    icon("\u{E2E9}")
}

pub fn redo<'a>() -> Text<'a> {
    icon("\u{E143}")
}

pub fn redo_2<'a>() -> Text<'a> {
    icon("\u{E2A0}")
}

pub fn redo_dot<'a>() -> Text<'a> {
    icon("\u{E450}")
}

pub fn refresh_ccw<'a>() -> Text<'a> {
    icon("\u{E144}")
}

pub fn refresh_ccw_dot<'a>() -> Text<'a> {
    icon("\u{E4B2}")
}

pub fn refresh_cw<'a>() -> Text<'a> {
    icon("\u{E145}")
}

pub fn refresh_cw_off<'a>() -> Text<'a> {
    icon("\u{E498}")
}

pub fn refrigerator<'a>() -> Text<'a> {
    icon("\u{E37B}")
}

pub fn regex<'a>() -> Text<'a> {
    icon("\u{E1FC}")
}

pub fn remove_formatting<'a>() -> Text<'a> {
    icon("\u{E3B3}")
}

pub fn repeat<'a>() -> Text<'a> {
    icon("\u{E146}")
}

pub fn repeat_1<'a>() -> Text<'a> {
    icon("\u{E1FD}")
}

pub fn repeat_2<'a>() -> Text<'a> {
    icon("\u{E411}")
}

pub fn replace<'a>() -> Text<'a> {
    icon("\u{E3DB}")
}

pub fn replace_all<'a>() -> Text<'a> {
    icon("\u{E3DC}")
}

pub fn reply<'a>() -> Text<'a> {
    icon("\u{E22A}")
}

pub fn reply_all<'a>() -> Text<'a> {
    icon("\u{E22B}")
}

pub fn rewind<'a>() -> Text<'a> {
    icon("\u{E147}")
}

pub fn ribbon<'a>() -> Text<'a> {
    icon("\u{E558}")
}

pub fn rocket<'a>() -> Text<'a> {
    icon("\u{E286}")
}

pub fn rocking_chair<'a>() -> Text<'a> {
    icon("\u{E233}")
}

pub fn roller_coaster<'a>() -> Text<'a> {
    icon("\u{E480}")
}

pub fn rose<'a>() -> Text<'a> {
    icon("\u{E691}")
}

pub fn rotate_3d<'a>() -> Text<'a> {
    icon("\u{E2EA}")
}

pub fn rotate_ccw<'a>() -> Text<'a> {
    icon("\u{E148}")
}

pub fn rotate_ccw_key<'a>() -> Text<'a> {
    icon("\u{E650}")
}

pub fn rotate_ccw_square<'a>() -> Text<'a> {
    icon("\u{E5D0}")
}

pub fn rotate_cw<'a>() -> Text<'a> {
    icon("\u{E149}")
}

pub fn rotate_cw_square<'a>() -> Text<'a> {
    icon("\u{E5D1}")
}

pub fn route<'a>() -> Text<'a> {
    icon("\u{E53E}")
}

pub fn route_off<'a>() -> Text<'a> {
    icon("\u{E53F}")
}

pub fn router<'a>() -> Text<'a> {
    icon("\u{E3BF}")
}

pub fn rows_2<'a>() -> Text<'a> {
    icon("\u{E439}")
}

pub fn rows_3<'a>() -> Text<'a> {
    icon("\u{E58A}")
}

pub fn rows_4<'a>() -> Text<'a> {
    icon("\u{E58B}")
}

pub fn rss<'a>() -> Text<'a> {
    icon("\u{E14A}")
}

pub fn ruler<'a>() -> Text<'a> {
    icon("\u{E14B}")
}

pub fn ruler_dimension_line<'a>() -> Text<'a> {
    icon("\u{E662}")
}

pub fn russian_ruble<'a>() -> Text<'a> {
    icon("\u{E14C}")
}

pub fn sailboat<'a>() -> Text<'a> {
    icon("\u{E37E}")
}

pub fn salad<'a>() -> Text<'a> {
    icon("\u{E3A8}")
}

pub fn sandwich<'a>() -> Text<'a> {
    icon("\u{E3A9}")
}

pub fn satellite<'a>() -> Text<'a> {
    icon("\u{E447}")
}

pub fn satellite_dish<'a>() -> Text<'a> {
    icon("\u{E448}")
}

pub fn saudi_riyal<'a>() -> Text<'a> {
    icon("\u{E64B}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{E14D}")
}

pub fn save_all<'a>() -> Text<'a> {
    icon("\u{E40F}")
}

pub fn save_off<'a>() -> Text<'a> {
    icon("\u{E5F3}")
}

pub fn scale<'a>() -> Text<'a> {
    icon("\u{E212}")
}

pub fn scale_3d<'a>() -> Text<'a> {
    icon("\u{E2EB}")
}

pub fn scaling<'a>() -> Text<'a> {
    icon("\u{E2EC}")
}

pub fn scan<'a>() -> Text<'a> {
    icon("\u{E257}")
}

pub fn scan_barcode<'a>() -> Text<'a> {
    icon("\u{E535}")
}

pub fn scan_eye<'a>() -> Text<'a> {
    icon("\u{E536}")
}

pub fn scan_face<'a>() -> Text<'a> {
    icon("\u{E371}")
}

pub fn scan_heart<'a>() -> Text<'a> {
    icon("\u{E63A}")
}

pub fn scan_line<'a>() -> Text<'a> {
    icon("\u{E258}")
}

pub fn scan_qr_code<'a>() -> Text<'a> {
    icon("\u{E5F6}")
}

pub fn scan_search<'a>() -> Text<'a> {
    icon("\u{E537}")
}

pub fn scan_text<'a>() -> Text<'a> {
    icon("\u{E538}")
}

pub fn school<'a>() -> Text<'a> {
    icon("\u{E3E3}")
}

pub fn scissors<'a>() -> Text<'a> {
    icon("\u{E14E}")
}

pub fn scissors_line_dashed<'a>() -> Text<'a> {
    icon("\u{E4E9}")
}

pub fn scooter<'a>() -> Text<'a> {
    icon("\u{E6AC}")
}

pub fn screen_share<'a>() -> Text<'a> {
    icon("\u{E14F}")
}

pub fn screen_share_off<'a>() -> Text<'a> {
    icon("\u{E150}")
}

pub fn scroll<'a>() -> Text<'a> {
    icon("\u{E2ED}")
}

pub fn scroll_text<'a>() -> Text<'a> {
    icon("\u{E45F}")
}

pub fn search<'a>() -> Text<'a> {
    icon("\u{E151}")
}

pub fn search_alert<'a>() -> Text<'a> {
    icon("\u{E6B4}")
}

pub fn search_check<'a>() -> Text<'a> {
    icon("\u{E4AA}")
}

pub fn search_code<'a>() -> Text<'a> {
    icon("\u{E4AB}")
}

pub fn search_slash<'a>() -> Text<'a> {
    icon("\u{E4AC}")
}

pub fn search_x<'a>() -> Text<'a> {
    icon("\u{E4AD}")
}

pub fn section<'a>() -> Text<'a> {
    icon("\u{E5E8}")
}

pub fn send<'a>() -> Text<'a> {
    icon("\u{E152}")
}

pub fn send_horizontal<'a>() -> Text<'a> {
    icon("\u{E4F2}")
}

pub fn send_to_back<'a>() -> Text<'a> {
    icon("\u{E4F3}")
}

pub fn separator_horizontal<'a>() -> Text<'a> {
    icon("\u{E1C8}")
}

pub fn separator_vertical<'a>() -> Text<'a> {
    icon("\u{E1C9}")
}

pub fn server<'a>() -> Text<'a> {
    icon("\u{E153}")
}

pub fn server_cog<'a>() -> Text<'a> {
    icon("\u{E341}")
}

pub fn server_crash<'a>() -> Text<'a> {
    icon("\u{E1E9}")
}

pub fn server_off<'a>() -> Text<'a> {
    icon("\u{E1EA}")
}

pub fn settings<'a>() -> Text<'a> {
    icon("\u{E154}")
}

pub fn settings_2<'a>() -> Text<'a> {
    icon("\u{E245}")
}

pub fn shapes<'a>() -> Text<'a> {
    icon("\u{E4B3}")
}

pub fn share<'a>() -> Text<'a> {
    icon("\u{E155}")
}

pub fn share_2<'a>() -> Text<'a> {
    icon("\u{E156}")
}

pub fn sheet<'a>() -> Text<'a> {
    icon("\u{E157}")
}

pub fn shell<'a>() -> Text<'a> {
    icon("\u{E4F7}")
}

pub fn shelving_unit<'a>() -> Text<'a> {
    icon("\u{E6C7}")
}

pub fn shield<'a>() -> Text<'a> {
    icon("\u{E158}")
}

pub fn shield_alert<'a>() -> Text<'a> {
    icon("\u{E1FE}")
}

pub fn shield_ban<'a>() -> Text<'a> {
    icon("\u{E159}")
}

pub fn shield_check<'a>() -> Text<'a> {
    icon("\u{E1FF}")
}

pub fn shield_ellipsis<'a>() -> Text<'a> {
    icon("\u{E516}")
}

pub fn shield_half<'a>() -> Text<'a> {
    icon("\u{E517}")
}

pub fn shield_minus<'a>() -> Text<'a> {
    icon("\u{E518}")
}

pub fn shield_off<'a>() -> Text<'a> {
    icon("\u{E15A}")
}

pub fn shield_plus<'a>() -> Text<'a> {
    icon("\u{E519}")
}

pub fn shield_question_mark<'a>() -> Text<'a> {
    icon("\u{E40E}")
}

pub fn shield_user<'a>() -> Text<'a> {
    icon("\u{E647}")
}

pub fn shield_x<'a>() -> Text<'a> {
    icon("\u{E200}")
}

pub fn ship<'a>() -> Text<'a> {
    icon("\u{E3BA}")
}

pub fn ship_wheel<'a>() -> Text<'a> {
    icon("\u{E502}")
}

pub fn shirt<'a>() -> Text<'a> {
    icon("\u{E1CA}")
}

pub fn shopping_bag<'a>() -> Text<'a> {
    icon("\u{E15B}")
}

pub fn shopping_basket<'a>() -> Text<'a> {
    icon("\u{E4EA}")
}

pub fn shopping_cart<'a>() -> Text<'a> {
    icon("\u{E15C}")
}

pub fn shovel<'a>() -> Text<'a> {
    icon("\u{E15D}")
}

pub fn shower_head<'a>() -> Text<'a> {
    icon("\u{E37C}")
}

pub fn shredder<'a>() -> Text<'a> {
    icon("\u{E65B}")
}

pub fn shrimp<'a>() -> Text<'a> {
    icon("\u{E649}")
}

pub fn shrink<'a>() -> Text<'a> {
    icon("\u{E220}")
}

pub fn shrub<'a>() -> Text<'a> {
    icon("\u{E2EE}")
}

pub fn shuffle<'a>() -> Text<'a> {
    icon("\u{E15E}")
}

pub fn sigma<'a>() -> Text<'a> {
    icon("\u{E201}")
}

pub fn signal<'a>() -> Text<'a> {
    icon("\u{E25F}")
}

pub fn signal_high<'a>() -> Text<'a> {
    icon("\u{E260}")
}

pub fn signal_low<'a>() -> Text<'a> {
    icon("\u{E261}")
}

pub fn signal_medium<'a>() -> Text<'a> {
    icon("\u{E262}")
}

pub fn signal_zero<'a>() -> Text<'a> {
    icon("\u{E263}")
}

pub fn signature<'a>() -> Text<'a> {
    icon("\u{E5F2}")
}

pub fn signpost<'a>() -> Text<'a> {
    icon("\u{E540}")
}

pub fn signpost_big<'a>() -> Text<'a> {
    icon("\u{E541}")
}

pub fn siren<'a>() -> Text<'a> {
    icon("\u{E2EF}")
}

pub fn skip_back<'a>() -> Text<'a> {
    icon("\u{E15F}")
}

pub fn skip_forward<'a>() -> Text<'a> {
    icon("\u{E160}")
}

pub fn skull<'a>() -> Text<'a> {
    icon("\u{E221}")
}

pub fn slack<'a>() -> Text<'a> {
    icon("\u{E161}")
}

pub fn slash<'a>() -> Text<'a> {
    icon("\u{E51D}")
}

pub fn slice<'a>() -> Text<'a> {
    icon("\u{E2F0}")
}

pub fn sliders_horizontal<'a>() -> Text<'a> {
    icon("\u{E29A}")
}

pub fn sliders_vertical<'a>() -> Text<'a> {
    icon("\u{E162}")
}

pub fn smartphone<'a>() -> Text<'a> {
    icon("\u{E163}")
}

pub fn smartphone_charging<'a>() -> Text<'a> {
    icon("\u{E22E}")
}

pub fn smartphone_nfc<'a>() -> Text<'a> {
    icon("\u{E3C4}")
}

pub fn smile<'a>() -> Text<'a> {
    icon("\u{E164}")
}

pub fn smile_plus<'a>() -> Text<'a> {
    icon("\u{E301}")
}

pub fn snail<'a>() -> Text<'a> {
    icon("\u{E4F8}")
}

pub fn snowflake<'a>() -> Text<'a> {
    icon("\u{E165}")
}

pub fn soap_dispenser_droplet<'a>() -> Text<'a> {
    icon("\u{E669}")
}

pub fn sofa<'a>() -> Text<'a> {
    icon("\u{E2C4}")
}

pub fn solar_panel<'a>() -> Text<'a> {
    icon("\u{E69F}")
}

pub fn soup<'a>() -> Text<'a> {
    icon("\u{E3AA}")
}

pub fn space<'a>() -> Text<'a> {
    icon("\u{E3DD}")
}

pub fn spade<'a>() -> Text<'a> {
    icon("\u{E499}")
}

pub fn sparkle<'a>() -> Text<'a> {
    icon("\u{E47E}")
}

pub fn sparkles<'a>() -> Text<'a> {
    icon("\u{E412}")
}

pub fn speaker<'a>() -> Text<'a> {
    icon("\u{E166}")
}

pub fn speech<'a>() -> Text<'a> {
    icon("\u{E51E}")
}

pub fn spell_check<'a>() -> Text<'a> {
    icon("\u{E49A}")
}

pub fn spell_check_2<'a>() -> Text<'a> {
    icon("\u{E49B}")
}

pub fn spline<'a>() -> Text<'a> {
    icon("\u{E38B}")
}

pub fn spline_pointer<'a>() -> Text<'a> {
    icon("\u{E64F}")
}

pub fn split<'a>() -> Text<'a> {
    icon("\u{E440}")
}

pub fn spool<'a>() -> Text<'a> {
    icon("\u{E677}")
}

pub fn spotlight<'a>() -> Text<'a> {
    icon("\u{E682}")
}

pub fn spray_can<'a>() -> Text<'a> {
    icon("\u{E495}")
}

pub fn sprout<'a>() -> Text<'a> {
    icon("\u{E1EB}")
}

pub fn square<'a>() -> Text<'a> {
    icon("\u{E167}")
}

pub fn square_activity<'a>() -> Text<'a> {
    icon("\u{E4B4}")
}

pub fn square_arrow_down<'a>() -> Text<'a> {
    icon("\u{E427}")
}

pub fn square_arrow_down_left<'a>() -> Text<'a> {
    icon("\u{E4B5}")
}

pub fn square_arrow_down_right<'a>() -> Text<'a> {
    icon("\u{E4B6}")
}

pub fn square_arrow_left<'a>() -> Text<'a> {
    icon("\u{E428}")
}

pub fn square_arrow_out_down_left<'a>() -> Text<'a> {
    icon("\u{E5A1}")
}

pub fn square_arrow_out_down_right<'a>() -> Text<'a> {
    icon("\u{E5A2}")
}

pub fn square_arrow_out_up_left<'a>() -> Text<'a> {
    icon("\u{E5A3}")
}

pub fn square_arrow_out_up_right<'a>() -> Text<'a> {
    icon("\u{E5A4}")
}

pub fn square_arrow_right<'a>() -> Text<'a> {
    icon("\u{E429}")
}

pub fn square_arrow_right_enter<'a>() -> Text<'a> {
    icon("\u{E6CE}")
}

pub fn square_arrow_right_exit<'a>() -> Text<'a> {
    icon("\u{E6CF}")
}

pub fn square_arrow_up<'a>() -> Text<'a> {
    icon("\u{E42A}")
}

pub fn square_arrow_up_left<'a>() -> Text<'a> {
    icon("\u{E4B7}")
}

pub fn square_arrow_up_right<'a>() -> Text<'a> {
    icon("\u{E4B8}")
}

pub fn square_asterisk<'a>() -> Text<'a> {
    icon("\u{E168}")
}

pub fn square_bottom_dashed_scissors<'a>() -> Text<'a> {
    icon("\u{E4EB}")
}

pub fn square_centerline_dashed_horizontal<'a>() -> Text<'a> {
    icon("\u{E35F}")
}

pub fn square_centerline_dashed_vertical<'a>() -> Text<'a> {
    icon("\u{E360}")
}

pub fn square_chart_gantt<'a>() -> Text<'a> {
    icon("\u{E169}")
}

pub fn square_check<'a>() -> Text<'a> {
    icon("\u{E559}")
}

pub fn square_check_big<'a>() -> Text<'a> {
    icon("\u{E16A}")
}

pub fn square_chevron_down<'a>() -> Text<'a> {
    icon("\u{E3CF}")
}

pub fn square_chevron_left<'a>() -> Text<'a> {
    icon("\u{E3D0}")
}

pub fn square_chevron_right<'a>() -> Text<'a> {
    icon("\u{E3D1}")
}

pub fn square_chevron_up<'a>() -> Text<'a> {
    icon("\u{E3D2}")
}

pub fn square_code<'a>() -> Text<'a> {
    icon("\u{E16B}")
}

pub fn square_dashed<'a>() -> Text<'a> {
    icon("\u{E1CB}")
}

pub fn square_dashed_bottom<'a>() -> Text<'a> {
    icon("\u{E4C0}")
}

pub fn square_dashed_bottom_code<'a>() -> Text<'a> {
    icon("\u{E4C1}")
}

pub fn square_dashed_kanban<'a>() -> Text<'a> {
    icon("\u{E16C}")
}

pub fn square_dashed_mouse_pointer<'a>() -> Text<'a> {
    icon("\u{E509}")
}

pub fn square_dashed_top_solid<'a>() -> Text<'a> {
    icon("\u{E66C}")
}

pub fn square_divide<'a>() -> Text<'a> {
    icon("\u{E16D}")
}

pub fn square_dot<'a>() -> Text<'a> {
    icon("\u{E16E}")
}

pub fn square_equal<'a>() -> Text<'a> {
    icon("\u{E16F}")
}

pub fn square_function<'a>() -> Text<'a> {
    icon("\u{E22D}")
}

pub fn square_kanban<'a>() -> Text<'a> {
    icon("\u{E170}")
}

pub fn square_library<'a>() -> Text<'a> {
    icon("\u{E54F}")
}

pub fn square_m<'a>() -> Text<'a> {
    icon("\u{E503}")
}

pub fn square_menu<'a>() -> Text<'a> {
    icon("\u{E453}")
}

pub fn square_minus<'a>() -> Text<'a> {
    icon("\u{E171}")
}

pub fn square_mouse_pointer<'a>() -> Text<'a> {
    icon("\u{E202}")
}

pub fn square_parking<'a>() -> Text<'a> {
    icon("\u{E3CB}")
}

pub fn square_parking_off<'a>() -> Text<'a> {
    icon("\u{E3CC}")
}

pub fn square_pause<'a>() -> Text<'a> {
    icon("\u{E684}")
}

pub fn square_pen<'a>() -> Text<'a> {
    icon("\u{E172}")
}

pub fn square_percent<'a>() -> Text<'a> {
    icon("\u{E51C}")
}

pub fn square_pi<'a>() -> Text<'a> {
    icon("\u{E488}")
}

pub fn square_pilcrow<'a>() -> Text<'a> {
    icon("\u{E48B}")
}

pub fn square_play<'a>() -> Text<'a> {
    icon("\u{E481}")
}

pub fn square_plus<'a>() -> Text<'a> {
    icon("\u{E173}")
}

pub fn square_power<'a>() -> Text<'a> {
    icon("\u{E551}")
}

pub fn square_radical<'a>() -> Text<'a> {
    icon("\u{E5C3}")
}

pub fn square_round_corner<'a>() -> Text<'a> {
    icon("\u{E648}")
}

pub fn square_scissors<'a>() -> Text<'a> {
    icon("\u{E4EC}")
}

pub fn square_sigma<'a>() -> Text<'a> {
    icon("\u{E489}")
}

pub fn square_slash<'a>() -> Text<'a> {
    icon("\u{E174}")
}

pub fn square_split_horizontal<'a>() -> Text<'a> {
    icon("\u{E3B6}")
}

pub fn square_split_vertical<'a>() -> Text<'a> {
    icon("\u{E3B7}")
}

pub fn square_square<'a>() -> Text<'a> {
    icon("\u{E60E}")
}

pub fn square_stack<'a>() -> Text<'a> {
    icon("\u{E4A2}")
}

pub fn square_star<'a>() -> Text<'a> {
    icon("\u{E68E}")
}

pub fn square_stop<'a>() -> Text<'a> {
    icon("\u{E685}")
}

pub fn square_terminal<'a>() -> Text<'a> {
    icon("\u{E20A}")
}

pub fn square_user<'a>() -> Text<'a> {
    icon("\u{E465}")
}

pub fn square_user_round<'a>() -> Text<'a> {
    icon("\u{E466}")
}

pub fn square_x<'a>() -> Text<'a> {
    icon("\u{E175}")
}

pub fn squares_exclude<'a>() -> Text<'a> {
    icon("\u{E657}")
}

pub fn squares_intersect<'a>() -> Text<'a> {
    icon("\u{E658}")
}

pub fn squares_subtract<'a>() -> Text<'a> {
    icon("\u{E659}")
}

pub fn squares_unite<'a>() -> Text<'a> {
    icon("\u{E65A}")
}

pub fn squircle<'a>() -> Text<'a> {
    icon("\u{E57A}")
}

pub fn squircle_dashed<'a>() -> Text<'a> {
    icon("\u{E679}")
}

pub fn squirrel<'a>() -> Text<'a> {
    icon("\u{E49F}")
}

pub fn stamp<'a>() -> Text<'a> {
    icon("\u{E3BB}")
}

pub fn star<'a>() -> Text<'a> {
    icon("\u{E176}")
}

pub fn star_half<'a>() -> Text<'a> {
    icon("\u{E20B}")
}

pub fn star_off<'a>() -> Text<'a> {
    icon("\u{E2B0}")
}

pub fn step_back<'a>() -> Text<'a> {
    icon("\u{E3E9}")
}

pub fn step_forward<'a>() -> Text<'a> {
    icon("\u{E3EA}")
}

pub fn stethoscope<'a>() -> Text<'a> {
    icon("\u{E2F1}")
}

pub fn sticker<'a>() -> Text<'a> {
    icon("\u{E302}")
}

pub fn sticky_note<'a>() -> Text<'a> {
    icon("\u{E303}")
}

pub fn stone<'a>() -> Text<'a> {
    icon("\u{E6B8}")
}

pub fn store<'a>() -> Text<'a> {
    icon("\u{E3E4}")
}

pub fn stretch_horizontal<'a>() -> Text<'a> {
    icon("\u{E27C}")
}

pub fn stretch_vertical<'a>() -> Text<'a> {
    icon("\u{E27D}")
}

pub fn strikethrough<'a>() -> Text<'a> {
    icon("\u{E177}")
}

pub fn subscript<'a>() -> Text<'a> {
    icon("\u{E25C}")
}

pub fn sun<'a>() -> Text<'a> {
    icon("\u{E178}")
}

pub fn sun_dim<'a>() -> Text<'a> {
    icon("\u{E299}")
}

pub fn sun_medium<'a>() -> Text<'a> {
    icon("\u{E2B1}")
}

pub fn sun_moon<'a>() -> Text<'a> {
    icon("\u{E2B2}")
}

pub fn sun_snow<'a>() -> Text<'a> {
    icon("\u{E372}")
}

pub fn sunrise<'a>() -> Text<'a> {
    icon("\u{E179}")
}

pub fn sunset<'a>() -> Text<'a> {
    icon("\u{E17A}")
}

pub fn superscript<'a>() -> Text<'a> {
    icon("\u{E25E}")
}

pub fn swatch_book<'a>() -> Text<'a> {
    icon("\u{E59F}")
}

pub fn swiss_franc<'a>() -> Text<'a> {
    icon("\u{E17B}")
}

pub fn switch_camera<'a>() -> Text<'a> {
    icon("\u{E17C}")
}

pub fn sword<'a>() -> Text<'a> {
    icon("\u{E2B3}")
}

pub fn swords<'a>() -> Text<'a> {
    icon("\u{E2B4}")
}

pub fn syringe<'a>() -> Text<'a> {
    icon("\u{E2F2}")
}

pub fn table<'a>() -> Text<'a> {
    icon("\u{E17D}")
}

pub fn table_2<'a>() -> Text<'a> {
    icon("\u{E2F9}")
}

pub fn table_cells_merge<'a>() -> Text<'a> {
    icon("\u{E5C7}")
}

pub fn table_cells_split<'a>() -> Text<'a> {
    icon("\u{E5C8}")
}

pub fn table_columns_split<'a>() -> Text<'a> {
    icon("\u{E5C9}")
}

pub fn table_of_contents<'a>() -> Text<'a> {
    icon("\u{E61E}")
}

pub fn table_properties<'a>() -> Text<'a> {
    icon("\u{E4DB}")
}

pub fn table_rows_split<'a>() -> Text<'a> {
    icon("\u{E5CA}")
}

pub fn tablet<'a>() -> Text<'a> {
    icon("\u{E17E}")
}

pub fn tablet_smartphone<'a>() -> Text<'a> {
    icon("\u{E50A}")
}

pub fn tablets<'a>() -> Text<'a> {
    icon("\u{E3BE}")
}

pub fn tag<'a>() -> Text<'a> {
    icon("\u{E17F}")
}

pub fn tags<'a>() -> Text<'a> {
    icon("\u{E35C}")
}

pub fn tally_1<'a>() -> Text<'a> {
    icon("\u{E4D6}")
}

pub fn tally_2<'a>() -> Text<'a> {
    icon("\u{E4D7}")
}

pub fn tally_3<'a>() -> Text<'a> {
    icon("\u{E4D8}")
}

pub fn tally_4<'a>() -> Text<'a> {
    icon("\u{E4D9}")
}

pub fn tally_5<'a>() -> Text<'a> {
    icon("\u{E4DA}")
}

pub fn tangent<'a>() -> Text<'a> {
    icon("\u{E52E}")
}

pub fn target<'a>() -> Text<'a> {
    icon("\u{E180}")
}

pub fn telescope<'a>() -> Text<'a> {
    icon("\u{E5C5}")
}

pub fn tent<'a>() -> Text<'a> {
    icon("\u{E227}")
}

pub fn tent_tree<'a>() -> Text<'a> {
    icon("\u{E53B}")
}

pub fn terminal<'a>() -> Text<'a> {
    icon("\u{E181}")
}

pub fn test_tube<'a>() -> Text<'a> {
    icon("\u{E405}")
}

pub fn test_tube_diagonal<'a>() -> Text<'a> {
    icon("\u{E406}")
}

pub fn test_tubes<'a>() -> Text<'a> {
    icon("\u{E407}")
}

pub fn text_align_center<'a>() -> Text<'a> {
    icon("\u{E182}")
}

pub fn text_align_end<'a>() -> Text<'a> {
    icon("\u{E183}")
}

pub fn text_align_justify<'a>() -> Text<'a> {
    icon("\u{E184}")
}

pub fn text_align_start<'a>() -> Text<'a> {
    icon("\u{E185}")
}

pub fn text_cursor<'a>() -> Text<'a> {
    icon("\u{E264}")
}

pub fn text_cursor_input<'a>() -> Text<'a> {
    icon("\u{E265}")
}

pub fn text_initial<'a>() -> Text<'a> {
    icon("\u{E605}")
}

pub fn text_quote<'a>() -> Text<'a> {
    icon("\u{E49E}")
}

pub fn text_search<'a>() -> Text<'a> {
    icon("\u{E5AD}")
}

pub fn text_select<'a>() -> Text<'a> {
    icon("\u{E3DE}")
}

pub fn text_wrap<'a>() -> Text<'a> {
    icon("\u{E248}")
}

pub fn theater<'a>() -> Text<'a> {
    icon("\u{E522}")
}

pub fn thermometer<'a>() -> Text<'a> {
    icon("\u{E186}")
}

pub fn thermometer_snowflake<'a>() -> Text<'a> {
    icon("\u{E187}")
}

pub fn thermometer_sun<'a>() -> Text<'a> {
    icon("\u{E188}")
}

pub fn thumbs_down<'a>() -> Text<'a> {
    icon("\u{E189}")
}

pub fn thumbs_up<'a>() -> Text<'a> {
    icon("\u{E18A}")
}

pub fn ticket<'a>() -> Text<'a> {
    icon("\u{E20F}")
}

pub fn ticket_check<'a>() -> Text<'a> {
    icon("\u{E5AE}")
}

pub fn ticket_minus<'a>() -> Text<'a> {
    icon("\u{E5AF}")
}

pub fn ticket_percent<'a>() -> Text<'a> {
    icon("\u{E5B0}")
}

pub fn ticket_plus<'a>() -> Text<'a> {
    icon("\u{E5B1}")
}

pub fn ticket_slash<'a>() -> Text<'a> {
    icon("\u{E5B2}")
}

pub fn ticket_x<'a>() -> Text<'a> {
    icon("\u{E5B3}")
}

pub fn tickets<'a>() -> Text<'a> {
    icon("\u{E622}")
}

pub fn tickets_plane<'a>() -> Text<'a> {
    icon("\u{E623}")
}

pub fn timer<'a>() -> Text<'a> {
    icon("\u{E1E0}")
}

pub fn timer_off<'a>() -> Text<'a> {
    icon("\u{E249}")
}

pub fn timer_reset<'a>() -> Text<'a> {
    icon("\u{E236}")
}

pub fn toggle_left<'a>() -> Text<'a> {
    icon("\u{E18B}")
}

pub fn toggle_right<'a>() -> Text<'a> {
    icon("\u{E18C}")
}

pub fn toilet<'a>() -> Text<'a> {
    icon("\u{E635}")
}

pub fn tool_case<'a>() -> Text<'a> {
    icon("\u{E67D}")
}

pub fn toolbox<'a>() -> Text<'a> {
    icon("\u{E6B9}")
}

pub fn tornado<'a>() -> Text<'a> {
    icon("\u{E218}")
}

pub fn torus<'a>() -> Text<'a> {
    icon("\u{E52F}")
}

pub fn touchpad<'a>() -> Text<'a> {
    icon("\u{E449}")
}

pub fn touchpad_off<'a>() -> Text<'a> {
    icon("\u{E44A}")
}

pub fn towel_rack<'a>() -> Text<'a> {
    icon("\u{E6C6}")
}

pub fn tower_control<'a>() -> Text<'a> {
    icon("\u{E3BC}")
}

pub fn toy_brick<'a>() -> Text<'a> {
    icon("\u{E347}")
}

pub fn tractor<'a>() -> Text<'a> {
    icon("\u{E504}")
}

pub fn traffic_cone<'a>() -> Text<'a> {
    icon("\u{E505}")
}

pub fn train_front<'a>() -> Text<'a> {
    icon("\u{E506}")
}

pub fn train_front_tunnel<'a>() -> Text<'a> {
    icon("\u{E507}")
}

pub fn train_track<'a>() -> Text<'a> {
    icon("\u{E508}")
}

pub fn tram_front<'a>() -> Text<'a> {
    icon("\u{E2A9}")
}

pub fn transgender<'a>() -> Text<'a> {
    icon("\u{E644}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E18D}")
}

pub fn trash_2<'a>() -> Text<'a> {
    icon("\u{E18E}")
}

pub fn tree_deciduous<'a>() -> Text<'a> {
    icon("\u{E2F3}")
}

pub fn tree_palm<'a>() -> Text<'a> {
    icon("\u{E281}")
}

pub fn tree_pine<'a>() -> Text<'a> {
    icon("\u{E2F4}")
}

pub fn trees<'a>() -> Text<'a> {
    icon("\u{E2F5}")
}

pub fn trello<'a>() -> Text<'a> {
    icon("\u{E18F}")
}

pub fn trending_down<'a>() -> Text<'a> {
    icon("\u{E190}")
}

pub fn trending_up<'a>() -> Text<'a> {
    icon("\u{E191}")
}

pub fn trending_up_down<'a>() -> Text<'a> {
    icon("\u{E625}")
}

pub fn triangle<'a>() -> Text<'a> {
    icon("\u{E192}")
}

pub fn triangle_alert<'a>() -> Text<'a> {
    icon("\u{E193}")
}

pub fn triangle_dashed<'a>() -> Text<'a> {
    icon("\u{E63D}")
}

pub fn triangle_right<'a>() -> Text<'a> {
    icon("\u{E4ED}")
}

pub fn trophy<'a>() -> Text<'a> {
    icon("\u{E373}")
}

pub fn truck<'a>() -> Text<'a> {
    icon("\u{E194}")
}

pub fn truck_electric<'a>() -> Text<'a> {
    icon("\u{E65F}")
}

pub fn turkish_lira<'a>() -> Text<'a> {
    icon("\u{E680}")
}

pub fn turntable<'a>() -> Text<'a> {
    icon("\u{E68C}")
}

pub fn turtle<'a>() -> Text<'a> {
    icon("\u{E4F9}")
}

pub fn tv<'a>() -> Text<'a> {
    icon("\u{E195}")
}

pub fn tv_minimal<'a>() -> Text<'a> {
    icon("\u{E203}")
}

pub fn tv_minimal_play<'a>() -> Text<'a> {
    icon("\u{E5EC}")
}

pub fn twitch<'a>() -> Text<'a> {
    icon("\u{E196}")
}

pub fn twitter<'a>() -> Text<'a> {
    icon("\u{E197}")
}

pub fn type_icon<'a>() -> Text<'a> {
    icon("\u{E198}")
}

pub fn type_outline<'a>() -> Text<'a> {
    icon("\u{E602}")
}

pub fn umbrella<'a>() -> Text<'a> {
    icon("\u{E199}")
}

pub fn umbrella_off<'a>() -> Text<'a> {
    icon("\u{E543}")
}

pub fn underline<'a>() -> Text<'a> {
    icon("\u{E19A}")
}

pub fn undo<'a>() -> Text<'a> {
    icon("\u{E19B}")
}

pub fn undo_2<'a>() -> Text<'a> {
    icon("\u{E2A1}")
}

pub fn undo_dot<'a>() -> Text<'a> {
    icon("\u{E451}")
}

pub fn unfold_horizontal<'a>() -> Text<'a> {
    icon("\u{E43D}")
}

pub fn unfold_vertical<'a>() -> Text<'a> {
    icon("\u{E43E}")
}

pub fn ungroup<'a>() -> Text<'a> {
    icon("\u{E467}")
}

pub fn university<'a>() -> Text<'a> {
    icon("\u{E3E5}")
}

pub fn unlink<'a>() -> Text<'a> {
    icon("\u{E19C}")
}

pub fn unlink_2<'a>() -> Text<'a> {
    icon("\u{E19D}")
}

pub fn unplug<'a>() -> Text<'a> {
    icon("\u{E45D}")
}

pub fn upload<'a>() -> Text<'a> {
    icon("\u{E19E}")
}

pub fn usb<'a>() -> Text<'a> {
    icon("\u{E356}")
}

pub fn user<'a>() -> Text<'a> {
    icon("\u{E19F}")
}

pub fn user_check<'a>() -> Text<'a> {
    icon("\u{E1A0}")
}

pub fn user_cog<'a>() -> Text<'a> {
    icon("\u{E342}")
}

pub fn user_key<'a>() -> Text<'a> {
    icon("\u{E6BD}")
}

pub fn user_lock<'a>() -> Text<'a> {
    icon("\u{E660}")
}

pub fn user_minus<'a>() -> Text<'a> {
    icon("\u{E1A1}")
}

pub fn user_pen<'a>() -> Text<'a> {
    icon("\u{E5FC}")
}

pub fn user_plus<'a>() -> Text<'a> {
    icon("\u{E1A2}")
}

pub fn user_round<'a>() -> Text<'a> {
    icon("\u{E468}")
}

pub fn user_round_check<'a>() -> Text<'a> {
    icon("\u{E469}")
}

pub fn user_round_cog<'a>() -> Text<'a> {
    icon("\u{E46A}")
}

pub fn user_round_key<'a>() -> Text<'a> {
    icon("\u{E6BE}")
}

pub fn user_round_minus<'a>() -> Text<'a> {
    icon("\u{E46B}")
}

pub fn user_round_pen<'a>() -> Text<'a> {
    icon("\u{E5FD}")
}

pub fn user_round_plus<'a>() -> Text<'a> {
    icon("\u{E46C}")
}

pub fn user_round_search<'a>() -> Text<'a> {
    icon("\u{E578}")
}

pub fn user_round_x<'a>() -> Text<'a> {
    icon("\u{E46D}")
}

pub fn user_search<'a>() -> Text<'a> {
    icon("\u{E579}")
}

pub fn user_star<'a>() -> Text<'a> {
    icon("\u{E687}")
}

pub fn user_x<'a>() -> Text<'a> {
    icon("\u{E1A3}")
}

pub fn users<'a>() -> Text<'a> {
    icon("\u{E1A4}")
}

pub fn users_round<'a>() -> Text<'a> {
    icon("\u{E46E}")
}

pub fn utensils<'a>() -> Text<'a> {
    icon("\u{E2F6}")
}

pub fn utensils_crossed<'a>() -> Text<'a> {
    icon("\u{E2F7}")
}

pub fn utility_pole<'a>() -> Text<'a> {
    icon("\u{E3C2}")
}

pub fn van<'a>() -> Text<'a> {
    icon("\u{E6AD}")
}

pub fn variable<'a>() -> Text<'a> {
    icon("\u{E473}")
}

pub fn vault<'a>() -> Text<'a> {
    icon("\u{E58F}")
}

pub fn vector_square<'a>() -> Text<'a> {
    icon("\u{E67C}")
}

pub fn vegan<'a>() -> Text<'a> {
    icon("\u{E39D}")
}

pub fn venetian_mask<'a>() -> Text<'a> {
    icon("\u{E2AA}")
}

pub fn venus<'a>() -> Text<'a> {
    icon("\u{E645}")
}

pub fn venus_and_mars<'a>() -> Text<'a> {
    icon("\u{E646}")
}

pub fn vibrate<'a>() -> Text<'a> {
    icon("\u{E223}")
}

pub fn vibrate_off<'a>() -> Text<'a> {
    icon("\u{E29D}")
}

pub fn video<'a>() -> Text<'a> {
    icon("\u{E1A5}")
}

pub fn video_off<'a>() -> Text<'a> {
    icon("\u{E1A6}")
}

pub fn videotape<'a>() -> Text<'a> {
    icon("\u{E4CB}")
}

pub fn view<'a>() -> Text<'a> {
    icon("\u{E1A7}")
}

pub fn voicemail<'a>() -> Text<'a> {
    icon("\u{E1A8}")
}

pub fn volleyball<'a>() -> Text<'a> {
    icon("\u{E62F}")
}

pub fn volume<'a>() -> Text<'a> {
    icon("\u{E1A9}")
}

pub fn volume_1<'a>() -> Text<'a> {
    icon("\u{E1AA}")
}

pub fn volume_2<'a>() -> Text<'a> {
    icon("\u{E1AB}")
}

pub fn volume_off<'a>() -> Text<'a> {
    icon("\u{E626}")
}

pub fn volume_x<'a>() -> Text<'a> {
    icon("\u{E1AC}")
}

pub fn vote<'a>() -> Text<'a> {
    icon("\u{E3AD}")
}

pub fn wallet<'a>() -> Text<'a> {
    icon("\u{E204}")
}

pub fn wallet_cards<'a>() -> Text<'a> {
    icon("\u{E4CC}")
}

pub fn wallet_minimal<'a>() -> Text<'a> {
    icon("\u{E4CD}")
}

pub fn wallpaper<'a>() -> Text<'a> {
    icon("\u{E44B}")
}

pub fn wand<'a>() -> Text<'a> {
    icon("\u{E246}")
}

pub fn wand_sparkles<'a>() -> Text<'a> {
    icon("\u{E357}")
}

pub fn warehouse<'a>() -> Text<'a> {
    icon("\u{E3E6}")
}

pub fn washing_machine<'a>() -> Text<'a> {
    icon("\u{E590}")
}

pub fn watch<'a>() -> Text<'a> {
    icon("\u{E1AD}")
}

pub fn waves<'a>() -> Text<'a> {
    icon("\u{E283}")
}

pub fn waves_arrow_down<'a>() -> Text<'a> {
    icon("\u{E6A9}")
}

pub fn waves_arrow_up<'a>() -> Text<'a> {
    icon("\u{E6AA}")
}

pub fn waves_ladder<'a>() -> Text<'a> {
    icon("\u{E63B}")
}

pub fn waypoints<'a>() -> Text<'a> {
    icon("\u{E542}")
}

pub fn webcam<'a>() -> Text<'a> {
    icon("\u{E205}")
}

pub fn webhook<'a>() -> Text<'a> {
    icon("\u{E374}")
}

pub fn webhook_off<'a>() -> Text<'a> {
    icon("\u{E5B7}")
}

pub fn weight<'a>() -> Text<'a> {
    icon("\u{E530}")
}

pub fn weight_tilde<'a>() -> Text<'a> {
    icon("\u{E6AE}")
}

pub fn wheat<'a>() -> Text<'a> {
    icon("\u{E39E}")
}

pub fn wheat_off<'a>() -> Text<'a> {
    icon("\u{E39F}")
}

pub fn whole_word<'a>() -> Text<'a> {
    icon("\u{E3DF}")
}

pub fn wifi<'a>() -> Text<'a> {
    icon("\u{E1AE}")
}

pub fn wifi_cog<'a>() -> Text<'a> {
    icon("\u{E674}")
}

pub fn wifi_high<'a>() -> Text<'a> {
    icon("\u{E5F7}")
}

pub fn wifi_low<'a>() -> Text<'a> {
    icon("\u{E5F8}")
}

pub fn wifi_off<'a>() -> Text<'a> {
    icon("\u{E1AF}")
}

pub fn wifi_pen<'a>() -> Text<'a> {
    icon("\u{E663}")
}

pub fn wifi_sync<'a>() -> Text<'a> {
    icon("\u{E681}")
}

pub fn wifi_zero<'a>() -> Text<'a> {
    icon("\u{E5F9}")
}

pub fn wind<'a>() -> Text<'a> {
    icon("\u{E1B0}")
}

pub fn wind_arrow_down<'a>() -> Text<'a> {
    icon("\u{E631}")
}

pub fn wine<'a>() -> Text<'a> {
    icon("\u{E2F8}")
}

pub fn wine_off<'a>() -> Text<'a> {
    icon("\u{E3A0}")
}

pub fn workflow<'a>() -> Text<'a> {
    icon("\u{E425}")
}

pub fn worm<'a>() -> Text<'a> {
    icon("\u{E5DA}")
}

pub fn wrench<'a>() -> Text<'a> {
    icon("\u{E1B1}")
}

pub fn x<'a>() -> Text<'a> {
    icon("\u{E1B2}")
}

pub fn x_line_top<'a>() -> Text<'a> {
    icon("\u{E6CB}")
}

pub fn youtube<'a>() -> Text<'a> {
    icon("\u{E1B3}")
}

pub fn zap<'a>() -> Text<'a> {
    icon("\u{E1B4}")
}

pub fn zap_off<'a>() -> Text<'a> {
    icon("\u{E1B5}")
}

pub fn zoom_in<'a>() -> Text<'a> {
    icon("\u{E1B6}")
}

pub fn zoom_out<'a>() -> Text<'a> {
    icon("\u{E1B7}")
}

/// Render any Lucide icon by its codepoint string.
/// Use this together with [`ALL_ICONS`] to display icons dynamically:
/// ```ignore
/// for (name, cp) in ALL_ICONS {
///     button(render(cp)).on_press(Msg::Pick(name.to_string()))
/// }
/// ```
pub fn render(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("lucide"))
}

fn icon(codepoint: &str) -> Text<'_> {
    render(codepoint)
}
