use super::button::Button;
use core::cell::Cell;
use tao::keyboard::KeyCode;

macro_rules! key_pressed {
    ($name:ident, $key:ident) => {
        pub fn $name(&self) -> bool {
            self.is_pressed(KeyCode::$key)
        }
    };
}

macro_rules! key_released {
    ($name:ident, $key:ident) => {
        pub fn $name(&self) -> bool {
            self.is_released(KeyCode::$key)
        }
    };
}

macro_rules! key_down {
    ($name:ident, $key:ident) => {
        pub fn $name(&self) -> bool {
            self.is_down(KeyCode::$key)
        }
    };
}

#[derive(Clone)]
pub struct Keyboard {
    keys: Cell<[Button; 512]>,
}

impl Keyboard {
    pub fn new() -> Self {
        let keys = Cell::new([Button::default(); 512]);
        Self { keys }
    }

    pub fn update(&mut self, key: KeyCode, down: bool) {
        if let Some(scancode) = key.to_scancode() {
            let button = &mut self.keys.get_mut()[scancode as usize];
            button.update(down);
        }
    }

    pub fn key(&self, key: KeyCode) -> Option<Button> {
        key.to_scancode().map(|scancode| self.keys.get()[scancode as usize])
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        match self.key(key) {
            Some(button) => button.pressed(),
            None => false,
        }
    }

    pub fn is_released(&self, key: KeyCode) -> bool {
        match self.key(key) {
            Some(button) => button.released(),
            None => false,
        }
    }

    pub fn is_down(&self, key: KeyCode) -> bool {
        match self.key(key) {
            Some(button) => button.down(),
            None => false,
        }
    }

    pub fn is_up(&self, key: KeyCode) -> bool {
        !self.is_down(key)
    }

    key_pressed!(backquote_pressed, Backquote);
    key_pressed!(backslash_pressed, Backslash);
    key_pressed!(backspace_pressed, Backspace);
    key_pressed!(bracket_left_pressed, BracketLeft);
    key_pressed!(bracket_right_pressed, BracketRight);
    key_pressed!(comma_pressed, Comma);
    key_pressed!(digit0_pressed, Digit0);
    key_pressed!(digit1_pressed, Digit1);
    key_pressed!(digit2_pressed, Digit2);
    key_pressed!(digit3_pressed, Digit3);
    key_pressed!(digit4_pressed, Digit4);
    key_pressed!(digit5_pressed, Digit5);
    key_pressed!(digit6_pressed, Digit6);
    key_pressed!(digit7_pressed, Digit7);
    key_pressed!(digit8_pressed, Digit8);
    key_pressed!(digit9_pressed, Digit9);
    key_pressed!(equal_pressed, Equal);
    key_pressed!(intl_backslash_pressed, IntlBackslash);
    key_pressed!(intl_ro_pressed, IntlRo);
    key_pressed!(intl_yen_pressed, IntlYen);
    key_pressed!(a_key_pressed, KeyA);
    key_pressed!(b_key_pressed, KeyB);
    key_pressed!(c_key_pressed, KeyC);
    key_pressed!(d_key_pressed, KeyD);
    key_pressed!(e_key_pressed, KeyE);
    key_pressed!(f_key_pressed, KeyF);
    key_pressed!(g_key_pressed, KeyG);
    key_pressed!(h_key_pressed, KeyH);
    key_pressed!(i_key_pressed, KeyI);
    key_pressed!(j_key_pressed, KeyJ);
    key_pressed!(k_key_pressed, KeyK);
    key_pressed!(l_key_pressed, KeyL);
    key_pressed!(m_key_pressed, KeyM);
    key_pressed!(n_key_pressed, KeyN);
    key_pressed!(o_key_pressed, KeyO);
    key_pressed!(p_key_pressed, KeyP);
    key_pressed!(q_key_pressed, KeyQ);
    key_pressed!(r_key_pressed, KeyR);
    key_pressed!(s_key_pressed, KeyS);
    key_pressed!(t_key_pressed, KeyT);
    key_pressed!(u_key_pressed, KeyU);
    key_pressed!(v_key_pressed, KeyV);
    key_pressed!(w_key_pressed, KeyW);
    key_pressed!(x_key_pressed, KeyX);
    key_pressed!(y_key_pressed, KeyY);
    key_pressed!(z_key_pressed, KeyZ);
    key_pressed!(minus_pressed, Minus);
    key_pressed!(plus_pressed, Plus);
    key_pressed!(period_pressed, Period);
    key_pressed!(quote_pressed, Quote);
    key_pressed!(semicolon_pressed, Semicolon);
    key_pressed!(slash_pressed, Slash);
    key_pressed!(alt_left_pressed, AltLeft);
    key_pressed!(alt_right_pressed, AltRight);
    key_pressed!(caps_lock_pressed, CapsLock);
    key_pressed!(context_menu_pressed, ContextMenu);
    key_pressed!(control_left_pressed, ControlLeft);
    key_pressed!(control_right_pressed, ControlRight);
    key_pressed!(enter_pressed, Enter);
    key_pressed!(super_left_pressed, SuperLeft);
    key_pressed!(super_right_pressed, SuperRight);
    key_pressed!(shift_left_pressed, ShiftLeft);
    key_pressed!(shift_right_pressed, ShiftRight);
    key_pressed!(space_pressed, Space);
    key_pressed!(tab_pressed, Tab);
    key_pressed!(convert_pressed, Convert);
    key_pressed!(kana_mode_pressed, KanaMode);
    key_pressed!(lang1_pressed, Lang1);
    key_pressed!(lang2_pressed, Lang2);
    key_pressed!(lang3_pressed, Lang3);
    key_pressed!(lang4_pressed, Lang4);
    key_pressed!(lang5_pressed, Lang5);
    key_pressed!(non_convert_pressed, NonConvert);
    key_pressed!(delete_pressed, Delete);
    key_pressed!(end_pressed, End);
    key_pressed!(help_pressed, Help);
    key_pressed!(home_pressed, Home);
    key_pressed!(insert_pressed, Insert);
    key_pressed!(page_down_pressed, PageDown);
    key_pressed!(page_up_pressed, PageUp);
    key_pressed!(arrow_down_pressed, ArrowDown);
    key_pressed!(arrow_left_pressed, ArrowLeft);
    key_pressed!(arrow_right_pressed, ArrowRight);
    key_pressed!(arrow_up_pressed, ArrowUp);
    key_pressed!(num_lock_pressed, NumLock);
    key_pressed!(numpad0_pressed, Numpad0);
    key_pressed!(numpad1_pressed, Numpad1);
    key_pressed!(numpad2_pressed, Numpad2);
    key_pressed!(numpad3_pressed, Numpad3);
    key_pressed!(numpad4_pressed, Numpad4);
    key_pressed!(numpad5_pressed, Numpad5);
    key_pressed!(numpad6_pressed, Numpad6);
    key_pressed!(numpad7_pressed, Numpad7);
    key_pressed!(numpad8_pressed, Numpad8);
    key_pressed!(numpad9_pressed, Numpad9);
    key_pressed!(numpad_add_pressed, NumpadAdd);
    key_pressed!(numpad_backspace_pressed, NumpadBackspace);
    key_pressed!(numpad_clear_pressed, NumpadClear);
    key_pressed!(numpad_clear_entry_pressed, NumpadClearEntry);
    key_pressed!(numpad_comma_pressed, NumpadComma);
    key_pressed!(numpad_decimal_pressed, NumpadDecimal);
    key_pressed!(numpad_divide_pressed, NumpadDivide);
    key_pressed!(numpad_enter_pressed, NumpadEnter);
    key_pressed!(numpad_equal_pressed, NumpadEqual);
    key_pressed!(numpad_hash_pressed, NumpadHash);
    key_pressed!(numpad_memory_add_pressed, NumpadMemoryAdd);
    key_pressed!(numpad_memory_clear_pressed, NumpadMemoryClear);
    key_pressed!(numpad_memory_recall_pressed, NumpadMemoryRecall);
    key_pressed!(numpad_memory_store_pressed, NumpadMemoryStore);
    key_pressed!(numpad_memory_subtract_pressed, NumpadMemorySubtract);
    key_pressed!(numpad_multiply_pressed, NumpadMultiply);
    key_pressed!(numpad_paren_left_pressed, NumpadParenLeft);
    key_pressed!(numpad_paren_right_pressed, NumpadParenRight);
    key_pressed!(numpad_star_pressed, NumpadStar);
    key_pressed!(numpad_subtract_pressed, NumpadSubtract);
    key_pressed!(escape_pressed, Escape);
    key_pressed!(fn_pressed, Fn);
    key_pressed!(fn_lock_pressed, FnLock);
    key_pressed!(print_screen_pressed, PrintScreen);
    key_pressed!(scroll_lock_pressed, ScrollLock);
    key_pressed!(pause_pressed, Pause);
    key_pressed!(browser_back_pressed, BrowserBack);
    key_pressed!(browser_favorites_pressed, BrowserFavorites);
    key_pressed!(browser_forward_pressed, BrowserForward);
    key_pressed!(browser_home_pressed, BrowserHome);
    key_pressed!(browser_refresh_pressed, BrowserRefresh);
    key_pressed!(browser_search_pressed, BrowserSearch);
    key_pressed!(browser_stop_pressed, BrowserStop);
    key_pressed!(eject_pressed, Eject);
    key_pressed!(launch_app1_pressed, LaunchApp1);
    key_pressed!(launch_app2_pressed, LaunchApp2);
    key_pressed!(launch_mail_pressed, LaunchMail);
    key_pressed!(media_play_pause_pressed, MediaPlayPause);
    key_pressed!(media_select_pressed, MediaSelect);
    key_pressed!(media_stop_pressed, MediaStop);
    key_pressed!(media_track_next_pressed, MediaTrackNext);
    key_pressed!(media_track_previous_pressed, MediaTrackPrevious);
    key_pressed!(power_pressed, Power);
    key_pressed!(sleep_pressed, Sleep);
    key_pressed!(audio_volume_down_pressed, AudioVolumeDown);
    key_pressed!(audio_volume_mute_pressed, AudioVolumeMute);
    key_pressed!(audio_volume_up_pressed, AudioVolumeUp);
    key_pressed!(wake_up_pressed, WakeUp);
    key_pressed!(hyper_pressed, Hyper);
    key_pressed!(turbo_pressed, Turbo);
    key_pressed!(abort_pressed, Abort);
    key_pressed!(resume_pressed, Resume);
    key_pressed!(suspend_pressed, Suspend);
    key_pressed!(again_pressed, Again);
    key_pressed!(copy_pressed, Copy);
    key_pressed!(cut_pressed, Cut);
    key_pressed!(find_pressed, Find);
    key_pressed!(open_pressed, Open);
    key_pressed!(paste_pressed, Paste);
    key_pressed!(props_pressed, Props);
    key_pressed!(select_pressed, Select);
    key_pressed!(undo_pressed, Undo);
    key_pressed!(hiragana_pressed, Hiragana);
    key_pressed!(katakana_pressed, Katakana);
    key_pressed!(f1_pressed, F1);
    key_pressed!(f2_pressed, F2);
    key_pressed!(f3_pressed, F3);
    key_pressed!(f4_pressed, F4);
    key_pressed!(f5_pressed, F5);
    key_pressed!(f6_pressed, F6);
    key_pressed!(f7_pressed, F7);
    key_pressed!(f8_pressed, F8);
    key_pressed!(f9_pressed, F9);
    key_pressed!(f10_pressed, F10);
    key_pressed!(f11_pressed, F11);
    key_pressed!(f12_pressed, F12);
    key_pressed!(f13_pressed, F13);
    key_pressed!(f14_pressed, F14);
    key_pressed!(f15_pressed, F15);
    key_pressed!(f16_pressed, F16);
    key_pressed!(f17_pressed, F17);
    key_pressed!(f18_pressed, F18);
    key_pressed!(f19_pressed, F19);
    key_pressed!(f20_pressed, F20);
    key_pressed!(f21_pressed, F21);
    key_pressed!(f22_pressed, F22);
    key_pressed!(f23_pressed, F23);
    key_pressed!(f24_pressed, F24);
    key_pressed!(f25_pressed, F25);
    key_pressed!(f26_pressed, F26);
    key_pressed!(f27_pressed, F27);
    key_pressed!(f28_pressed, F28);
    key_pressed!(f29_pressed, F29);
    key_pressed!(f30_pressed, F30);
    key_pressed!(f31_pressed, F31);
    key_pressed!(f32_pressed, F32);
    key_pressed!(f33_pressed, F33);
    key_pressed!(f34_pressed, F34);
    key_pressed!(f35_pressed, F35);

    key_released!(backquote_released, Backquote);
    key_released!(backslash_released, Backslash);
    key_released!(backspace_released, Backspace);
    key_released!(bracket_left_released, BracketLeft);
    key_released!(bracket_right_released, BracketRight);
    key_released!(comma_released, Comma);
    key_released!(digit0_released, Digit0);
    key_released!(digit1_released, Digit1);
    key_released!(digit2_released, Digit2);
    key_released!(digit3_released, Digit3);
    key_released!(digit4_released, Digit4);
    key_released!(digit5_released, Digit5);
    key_released!(digit6_released, Digit6);
    key_released!(digit7_released, Digit7);
    key_released!(digit8_released, Digit8);
    key_released!(digit9_released, Digit9);
    key_released!(equal_released, Equal);
    key_released!(intl_backslash_released, IntlBackslash);
    key_released!(intl_ro_released, IntlRo);
    key_released!(intl_yen_released, IntlYen);
    key_released!(a_key_released, KeyA);
    key_released!(b_key_released, KeyB);
    key_released!(c_key_released, KeyC);
    key_released!(d_key_released, KeyD);
    key_released!(e_key_released, KeyE);
    key_released!(f_key_released, KeyF);
    key_released!(g_key_released, KeyG);
    key_released!(h_key_released, KeyH);
    key_released!(i_key_released, KeyI);
    key_released!(j_key_released, KeyJ);
    key_released!(k_key_released, KeyK);
    key_released!(l_key_released, KeyL);
    key_released!(m_key_released, KeyM);
    key_released!(n_key_released, KeyN);
    key_released!(o_key_released, KeyO);
    key_released!(p_key_released, KeyP);
    key_released!(q_key_released, KeyQ);
    key_released!(r_key_released, KeyR);
    key_released!(s_key_released, KeyS);
    key_released!(t_key_released, KeyT);
    key_released!(u_key_released, KeyU);
    key_released!(v_key_released, KeyV);
    key_released!(w_key_released, KeyW);
    key_released!(x_key_released, KeyX);
    key_released!(y_key_released, KeyY);
    key_released!(z_key_released, KeyZ);
    key_released!(minus_released, Minus);
    key_released!(plus_released, Plus);
    key_released!(period_released, Period);
    key_released!(quote_released, Quote);
    key_released!(semicolon_released, Semicolon);
    key_released!(slash_released, Slash);
    key_released!(alt_left_released, AltLeft);
    key_released!(alt_right_released, AltRight);
    key_released!(caps_lock_released, CapsLock);
    key_released!(context_menu_released, ContextMenu);
    key_released!(control_left_released, ControlLeft);
    key_released!(control_right_released, ControlRight);
    key_released!(enter_released, Enter);
    key_released!(super_left_released, SuperLeft);
    key_released!(super_right_released, SuperRight);
    key_released!(shift_left_released, ShiftLeft);
    key_released!(shift_right_released, ShiftRight);
    key_released!(space_released, Space);
    key_released!(tab_released, Tab);
    key_released!(convert_released, Convert);
    key_released!(kana_mode_released, KanaMode);
    key_released!(lang1_released, Lang1);
    key_released!(lang2_released, Lang2);
    key_released!(lang3_released, Lang3);
    key_released!(lang4_released, Lang4);
    key_released!(lang5_released, Lang5);
    key_released!(non_convert_released, NonConvert);
    key_released!(delete_released, Delete);
    key_released!(end_released, End);
    key_released!(help_released, Help);
    key_released!(home_released, Home);
    key_released!(insert_released, Insert);
    key_released!(page_down_released, PageDown);
    key_released!(page_up_released, PageUp);
    key_released!(arrow_released_released, ArrowDown);
    key_released!(arrow_left_released, ArrowLeft);
    key_released!(arrow_right_released, ArrowRight);
    key_released!(arrow_up_released, ArrowUp);
    key_released!(num_lock_released, NumLock);
    key_released!(numpad0_released, Numpad0);
    key_released!(numpad1_released, Numpad1);
    key_released!(numpad2_released, Numpad2);
    key_released!(numpad3_released, Numpad3);
    key_released!(numpad4_released, Numpad4);
    key_released!(numpad5_released, Numpad5);
    key_released!(numpad6_released, Numpad6);
    key_released!(numpad7_released, Numpad7);
    key_released!(numpad8_released, Numpad8);
    key_released!(numpad9_released, Numpad9);
    key_released!(numpad_add_released, NumpadAdd);
    key_released!(numpad_backspace_released, NumpadBackspace);
    key_released!(numpad_clear_released, NumpadClear);
    key_released!(numpad_clear_entry_released, NumpadClearEntry);
    key_released!(numpad_comma_released, NumpadComma);
    key_released!(numpad_decimal_released, NumpadDecimal);
    key_released!(numpad_divide_released, NumpadDivide);
    key_released!(numpad_enter_released, NumpadEnter);
    key_released!(numpad_equal_released, NumpadEqual);
    key_released!(numpad_hash_released, NumpadHash);
    key_released!(numpad_memory_add_released, NumpadMemoryAdd);
    key_released!(numpad_memory_clear_released, NumpadMemoryClear);
    key_released!(numpad_memory_recall_released, NumpadMemoryRecall);
    key_released!(numpad_memory_store_released, NumpadMemoryStore);
    key_released!(numpad_memory_subtract_released, NumpadMemorySubtract);
    key_released!(numpad_multiply_released, NumpadMultiply);
    key_released!(numpad_paren_left_released, NumpadParenLeft);
    key_released!(numpad_paren_right_released, NumpadParenRight);
    key_released!(numpad_star_released, NumpadStar);
    key_released!(numpad_subtract_released, NumpadSubtract);
    key_released!(escape_released, Escape);
    key_released!(fn_released, Fn);
    key_released!(fn_lock_released, FnLock);
    key_released!(print_screen_released, PrintScreen);
    key_released!(scroll_lock_released, ScrollLock);
    key_released!(pause_released, Pause);
    key_released!(browser_back_released, BrowserBack);
    key_released!(browser_favorites_released, BrowserFavorites);
    key_released!(browser_forward_released, BrowserForward);
    key_released!(browser_home_released, BrowserHome);
    key_released!(browser_refresh_released, BrowserRefresh);
    key_released!(browser_search_released, BrowserSearch);
    key_released!(browser_stop_released, BrowserStop);
    key_released!(eject_released, Eject);
    key_released!(launch_app1_released, LaunchApp1);
    key_released!(launch_app2_released, LaunchApp2);
    key_released!(launch_mail_released, LaunchMail);
    key_released!(media_play_pause_released, MediaPlayPause);
    key_released!(media_select_released, MediaSelect);
    key_released!(media_stop_released, MediaStop);
    key_released!(media_track_next_released, MediaTrackNext);
    key_released!(media_track_previous_released, MediaTrackPrevious);
    key_released!(power_released, Power);
    key_released!(sleep_released, Sleep);
    key_released!(audio_volume_down_released, AudioVolumeDown);
    key_released!(audio_volume_mute_released, AudioVolumeMute);
    key_released!(audio_volume_up_released, AudioVolumeUp);
    key_released!(wake_up_released, WakeUp);
    key_released!(hyper_released, Hyper);
    key_released!(turbo_released, Turbo);
    key_released!(abort_released, Abort);
    key_released!(resume_released, Resume);
    key_released!(suspend_released, Suspend);
    key_released!(again_released, Again);
    key_released!(copy_released, Copy);
    key_released!(cut_released, Cut);
    key_released!(find_released, Find);
    key_released!(open_released, Open);
    key_released!(paste_released, Paste);
    key_released!(props_released, Props);
    key_released!(select_released, Select);
    key_released!(undo_released, Undo);
    key_released!(hiragana_released, Hiragana);
    key_released!(katakana_released, Katakana);
    key_released!(f1_released, F1);
    key_released!(f2_released, F2);
    key_released!(f3_released, F3);
    key_released!(f4_released, F4);
    key_released!(f5_released, F5);
    key_released!(f6_released, F6);
    key_released!(f7_released, F7);
    key_released!(f8_released, F8);
    key_released!(f9_released, F9);
    key_released!(f10_released, F10);
    key_released!(f11_released, F11);
    key_released!(f12_released, F12);
    key_released!(f13_released, F13);
    key_released!(f14_released, F14);
    key_released!(f15_released, F15);
    key_released!(f16_released, F16);
    key_released!(f17_released, F17);
    key_released!(f18_released, F18);
    key_released!(f19_released, F19);
    key_released!(f20_released, F20);
    key_released!(f21_released, F21);
    key_released!(f22_released, F22);
    key_released!(f23_released, F23);
    key_released!(f24_released, F24);
    key_released!(f25_released, F25);
    key_released!(f26_released, F26);
    key_released!(f27_released, F27);
    key_released!(f28_released, F28);
    key_released!(f29_released, F29);
    key_released!(f30_released, F30);
    key_released!(f31_released, F31);
    key_released!(f32_released, F32);
    key_released!(f33_released, F33);
    key_released!(f34_released, F34);
    key_released!(f35_released, F35);

    key_down!(backquote_down, Backquote);
    key_down!(backslash_down, Backslash);
    key_down!(backspace_down, Backspace);
    key_down!(bracket_left_down, BracketLeft);
    key_down!(bracket_right_down, BracketRight);
    key_down!(comma_down, Comma);
    key_down!(digit0_down, Digit0);
    key_down!(digit1_down, Digit1);
    key_down!(digit2_down, Digit2);
    key_down!(digit3_down, Digit3);
    key_down!(digit4_down, Digit4);
    key_down!(digit5_down, Digit5);
    key_down!(digit6_down, Digit6);
    key_down!(digit7_down, Digit7);
    key_down!(digit8_down, Digit8);
    key_down!(digit9_down, Digit9);
    key_down!(equal_down, Equal);
    key_down!(intl_backslash_down, IntlBackslash);
    key_down!(intl_ro_down, IntlRo);
    key_down!(intl_yen_down, IntlYen);
    key_down!(a_key_down, KeyA);
    key_down!(b_key_down, KeyB);
    key_down!(c_key_down, KeyC);
    key_down!(d_key_down, KeyD);
    key_down!(e_key_down, KeyE);
    key_down!(f_key_down, KeyF);
    key_down!(g_key_down, KeyG);
    key_down!(h_key_down, KeyH);
    key_down!(i_key_down, KeyI);
    key_down!(j_key_down, KeyJ);
    key_down!(k_key_down, KeyK);
    key_down!(l_key_down, KeyL);
    key_down!(m_key_down, KeyM);
    key_down!(n_key_down, KeyN);
    key_down!(o_key_down, KeyO);
    key_down!(p_key_down, KeyP);
    key_down!(q_key_down, KeyQ);
    key_down!(r_key_down, KeyR);
    key_down!(s_key_down, KeyS);
    key_down!(t_key_down, KeyT);
    key_down!(u_key_down, KeyU);
    key_down!(v_key_down, KeyV);
    key_down!(w_key_down, KeyW);
    key_down!(x_key_down, KeyX);
    key_down!(y_key_down, KeyY);
    key_down!(z_key_down, KeyZ);
    key_down!(minus_down, Minus);
    key_down!(plus_down, Plus);
    key_down!(period_down, Period);
    key_down!(quote_down, Quote);
    key_down!(semicolon_down, Semicolon);
    key_down!(slash_down, Slash);
    key_down!(alt_left_down, AltLeft);
    key_down!(alt_right_down, AltRight);
    key_down!(caps_lock_down, CapsLock);
    key_down!(context_menu_down, ContextMenu);
    key_down!(control_left_down, ControlLeft);
    key_down!(control_right_down, ControlRight);
    key_down!(enter_down, Enter);
    key_down!(super_left_down, SuperLeft);
    key_down!(super_right_down, SuperRight);
    key_down!(shift_left_down, ShiftLeft);
    key_down!(shift_right_down, ShiftRight);
    key_down!(space_down, Space);
    key_down!(tab_down, Tab);
    key_down!(convert_down, Convert);
    key_down!(kana_mode_down, KanaMode);
    key_down!(lang1_down, Lang1);
    key_down!(lang2_down, Lang2);
    key_down!(lang3_down, Lang3);
    key_down!(lang4_down, Lang4);
    key_down!(lang5_down, Lang5);
    key_down!(non_convert_down, NonConvert);
    key_down!(delete_down, Delete);
    key_down!(end_down, End);
    key_down!(help_down, Help);
    key_down!(home_down, Home);
    key_down!(insert_down, Insert);
    key_down!(page_down_down, PageDown);
    key_down!(page_up_down, PageUp);
    key_down!(arrow_down_down, ArrowDown);
    key_down!(arrow_left_down, ArrowLeft);
    key_down!(arrow_right_down, ArrowRight);
    key_down!(arrow_up_down, ArrowUp);
    key_down!(num_lock_down, NumLock);
    key_down!(numpad0_down, Numpad0);
    key_down!(numpad1_down, Numpad1);
    key_down!(numpad2_down, Numpad2);
    key_down!(numpad3_down, Numpad3);
    key_down!(numpad4_down, Numpad4);
    key_down!(numpad5_down, Numpad5);
    key_down!(numpad6_down, Numpad6);
    key_down!(numpad7_down, Numpad7);
    key_down!(numpad8_down, Numpad8);
    key_down!(numpad9_down, Numpad9);
    key_down!(numpad_add_down, NumpadAdd);
    key_down!(numpad_backspace_down, NumpadBackspace);
    key_down!(numpad_clear_down, NumpadClear);
    key_down!(numpad_clear_entry_down, NumpadClearEntry);
    key_down!(numpad_comma_down, NumpadComma);
    key_down!(numpad_decimal_down, NumpadDecimal);
    key_down!(numpad_divide_down, NumpadDivide);
    key_down!(numpad_enter_down, NumpadEnter);
    key_down!(numpad_equal_down, NumpadEqual);
    key_down!(numpad_hash_down, NumpadHash);
    key_down!(numpad_memory_add_down, NumpadMemoryAdd);
    key_down!(numpad_memory_clear_down, NumpadMemoryClear);
    key_down!(numpad_memory_recall_down, NumpadMemoryRecall);
    key_down!(numpad_memory_store_down, NumpadMemoryStore);
    key_down!(numpad_memory_subtract_down, NumpadMemorySubtract);
    key_down!(numpad_multiply_down, NumpadMultiply);
    key_down!(numpad_paren_left_down, NumpadParenLeft);
    key_down!(numpad_paren_right_down, NumpadParenRight);
    key_down!(numpad_star_down, NumpadStar);
    key_down!(numpad_subtract_down, NumpadSubtract);
    key_down!(escape_down, Escape);
    key_down!(fn_down, Fn);
    key_down!(fn_lock_down, FnLock);
    key_down!(print_screen_down, PrintScreen);
    key_down!(scroll_lock_down, ScrollLock);
    key_down!(pause_down, Pause);
    key_down!(browser_back_down, BrowserBack);
    key_down!(browser_favorites_down, BrowserFavorites);
    key_down!(browser_forward_down, BrowserForward);
    key_down!(browser_home_down, BrowserHome);
    key_down!(browser_refresh_down, BrowserRefresh);
    key_down!(browser_search_down, BrowserSearch);
    key_down!(browser_stop_down, BrowserStop);
    key_down!(eject_down, Eject);
    key_down!(launch_app1_down, LaunchApp1);
    key_down!(launch_app2_down, LaunchApp2);
    key_down!(launch_mail_down, LaunchMail);
    key_down!(media_play_pause_down, MediaPlayPause);
    key_down!(media_select_down, MediaSelect);
    key_down!(media_stop_down, MediaStop);
    key_down!(media_track_next_down, MediaTrackNext);
    key_down!(media_track_previous_down, MediaTrackPrevious);
    key_down!(power_down, Power);
    key_down!(sleep_down, Sleep);
    key_down!(audio_volume_down_down, AudioVolumeDown);
    key_down!(audio_volume_mute_down, AudioVolumeMute);
    key_down!(audio_volume_up_down, AudioVolumeUp);
    key_down!(wake_up_down, WakeUp);
    key_down!(hyper_down, Hyper);
    key_down!(turbo_down, Turbo);
    key_down!(abort_down, Abort);
    key_down!(resume_down, Resume);
    key_down!(suspend_down, Suspend);
    key_down!(again_down, Again);
    key_down!(copy_down, Copy);
    key_down!(cut_down, Cut);
    key_down!(find_down, Find);
    key_down!(open_down, Open);
    key_down!(paste_down, Paste);
    key_down!(props_down, Props);
    key_down!(select_down, Select);
    key_down!(undo_down, Undo);
    key_down!(hiragana_down, Hiragana);
    key_down!(katakana_down, Katakana);
    key_down!(f1_down, F1);
    key_down!(f2_down, F2);
    key_down!(f3_down, F3);
    key_down!(f4_down, F4);
    key_down!(f5_down, F5);
    key_down!(f6_down, F6);
    key_down!(f7_down, F7);
    key_down!(f8_down, F8);
    key_down!(f9_down, F9);
    key_down!(f10_down, F10);
    key_down!(f11_down, F11);
    key_down!(f12_down, F12);
    key_down!(f13_down, F13);
    key_down!(f14_down, F14);
    key_down!(f15_down, F15);
    key_down!(f16_down, F16);
    key_down!(f17_down, F17);
    key_down!(f18_down, F18);
    key_down!(f19_down, F19);
    key_down!(f20_down, F20);
    key_down!(f21_down, F21);
    key_down!(f22_down, F22);
    key_down!(f23_down, F23);
    key_down!(f24_down, F24);
    key_down!(f25_down, F25);
    key_down!(f26_down, F26);
    key_down!(f27_down, F27);
    key_down!(f28_down, F28);
    key_down!(f29_down, F29);
    key_down!(f30_down, F30);
    key_down!(f31_down, F31);
    key_down!(f32_down, F32);
    key_down!(f33_down, F33);
    key_down!(f34_down, F34);
    key_down!(f35_down, F35);
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}
